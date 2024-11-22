use petgraph::graph::{DiGraph, NodeIndex};
use rand::{ Rng, SeedableRng };

/// GraphConfig for generating random directed graphs
#[derive(Debug)]
pub struct GraphConfig {
    pub num_nodes: usize,
    pub edge_prob: f64,
    pub capacity_range: (u32, u32),
    pub random_seed: u64,
    pub density: f64,
}

impl GraphConfig {
    /// Create a new GraphConfig instance
    pub fn new(
        num_nodes: usize,
        edge_prob: f64,
        capacity_range: (u32, u32),
        random_seed: u64,
        density: f64,
    ) -> Self {
        assert!((0.0..=1.0).contains(&edge_prob), "edge_prob must be in [0.0, 1.0]");
        assert!((0.0..=1.0).contains(&density), "density must be in [0.0, 1.0]");
        assert!(capacity_range.0 <= capacity_range.1, "Invalid capacity range");

        Self {
            num_nodes,
            edge_prob,
            capacity_range,
            random_seed,
            density,
        }
    }

    /// Generate a random directed graph with edge capacities
    pub fn create_random_flow_graph(&self) -> DiGraph<u32, u32> {
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.random_seed);
        let mut graph = DiGraph::new();
        let nodes: Vec<NodeIndex> = (0..self.num_nodes).map(|_| graph.add_node(0)).collect();

        let total_possible_edges = self.num_nodes * (self.num_nodes - 1);
        let target_edges = (total_possible_edges as f64 * self.density).round() as usize;
        let mut edges_added = 0;

        while edges_added < target_edges {
            let i = rng.gen_range(0..self.num_nodes);
            let j = rng.gen_range(0..self.num_nodes);

            // Avoid self-loops and duplicate edges
            if i != j && graph.find_edge(nodes[i], nodes[j]).is_none() {
                let capacity = rng.gen_range(self.capacity_range.0..=self.capacity_range.1);
                graph.add_edge(nodes[i], nodes[j], capacity);
                edges_added += 1;
            }
        }

        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::visit::EdgeRef; // Import EdgeRef trait to access `source` and `target` methods

    #[test]
    fn test_graph_generation_reproducibility() {
        let seed = 42;
        let density = 0.5;
        let config1 = GraphConfig::new(5, 0.8, (1, 10), seed, density);
        let config2 = GraphConfig::new(5, 0.8, (1, 10), seed, density);

        let graph1 = config1.create_random_flow_graph();
        let graph2 = config2.create_random_flow_graph();

        assert_eq!(graph1.node_count(), graph2.node_count());
        assert_eq!(graph1.edge_count(), graph2.edge_count());
        // Verify that the edges themselves are identical
        for edge in graph1.edge_references() {
            assert!(graph2.contains_edge(edge.source(), edge.target()));
        }
    }

    #[test]
    fn test_graph_node_and_edge_counts() {
        let seed = 42;
        let density = 0.3; // Expect ~30% of possible edges
        let config = GraphConfig::new(6, 0.5, (1, 20), seed, density);
        let graph = config.create_random_flow_graph();

        assert_eq!(graph.node_count(), 6);
        let total_possible_edges = 6 * (6 - 1);
        let expected_edges = (total_possible_edges as f64 * density).round() as usize;

        assert!(graph.edge_count() > 0); // Ensure at least some edges are created
        assert!(graph.edge_count() <= total_possible_edges); // Should not exceed possible edges
        assert_eq!(graph.edge_count(), expected_edges); // Matches expected density
    }

    #[test]
    fn test_empty_graph_generation() {
        let seed = 42;
        let density = 0.0; // No edges
        let config = GraphConfig::new(4, 0.8, (1, 10), seed, density);
        let graph = config.create_random_flow_graph();

        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 0); // Should have no edges
    }

    #[test]
    fn test_fully_connected_graph_generation() {
        let seed = 42;
        let density = 1.0; // Fully connected
        let config = GraphConfig::new(4, 0.8, (1, 10), seed, density);
        let graph = config.create_random_flow_graph();

        let total_possible_edges = 4 * (4 - 1); // Directed graph
        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), total_possible_edges); // Fully connected graph
    }
}
