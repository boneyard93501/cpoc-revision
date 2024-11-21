use petgraph::graph::{DiGraph, NodeIndex};
use rand::{ Rng, SeedableRng };

/// GraphConfig for generating random directed graphs
#[derive(Debug)]
pub struct GraphConfig {
    pub num_nodes: usize,
    pub edge_prob: f64,
    pub capacity_range: (u32, u32),
    pub random_seed: u64,
}

impl GraphConfig {
    /// Create a new GraphConfig instance
    pub fn new(num_nodes: usize, edge_prob: f64, capacity_range: (u32, u32), random_seed: u64) -> Self {
        assert!((0.0..=1.0).contains(&edge_prob), "edge_prob must be in [0.0, 1.0]");
        assert!(capacity_range.0 <= capacity_range.1, "Invalid capacity range");

        Self {
            num_nodes,
            edge_prob,
            capacity_range,
            random_seed,
        }
    }

    /// Generate a random directed graph with edge capacities
    pub fn create_random_flow_graph(&self) -> DiGraph<u32, u32> {
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.random_seed);
        let mut graph = DiGraph::new();
        let nodes: Vec<NodeIndex> = (0..self.num_nodes).map(|_| graph.add_node(0)).collect();

        for i in 0..self.num_nodes {
            for j in 0..self.num_nodes {
                if i != j && rng.gen::<f64>() < self.edge_prob {
                    let capacity = rng.gen_range(self.capacity_range.0..=self.capacity_range.1);
                    graph.add_edge(nodes[i], nodes[j], capacity);
                }
            }
        }

        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_generation_reproducibility() {
        let seed = 42;
        let config1 = GraphConfig::new(5, 0.8, (1, 10), seed);
        let config2 = GraphConfig::new(5, 0.8, (1, 10), seed);

        let graph1 = config1.create_random_flow_graph();
        let graph2 = config2.create_random_flow_graph();

        assert_eq!(graph1.node_count(), graph2.node_count());
        assert_eq!(graph1.edge_count(), graph2.edge_count());
    }

    #[test]
    fn test_graph_node_and_edge_counts() {
        let config = GraphConfig::new(6, 0.5, (1, 20), 42);
        let graph = config.create_random_flow_graph();

        assert_eq!(graph.node_count(), 6);
        assert!(graph.edge_count() > 0); // With 50% edge probability, expect at least some edges
    }
}
