use std::collections::{HashMap, VecDeque};

/// Edmonds-Karp algorithm for finding the maximum flow in a graph
#[derive(Debug)]
pub struct EdmondsKarp {
    pub graph: HashMap<usize, Vec<(usize, i32)>>, // Adjacency list
    pub residual: HashMap<(usize, usize), i32>,  // Residual capacities
}

impl EdmondsKarp {
    /// Create a new EdmondsKarp instance
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
            residual: HashMap::new(),
        }
    }

    /// Add an edge to the graph with a specified capacity
    pub fn add_edge(&mut self, from: usize, to: usize, capacity: i32) {
        self.graph.entry(from).or_default().push((to, capacity));
        self.graph.entry(to).or_default();

        self.residual.insert((from, to), capacity);
        self.residual.insert((to, from), 0);
    }

    /// Perform a BFS to find an augmenting path
    fn bfs(&self, source: usize, sink: usize, parent: &mut HashMap<usize, usize>) -> bool {
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();

        queue.push_back(source);
        visited.insert(source, true);

        while let Some(current) = queue.pop_front() {
            if let Some(neighbors) = self.graph.get(&current) {
                for &(neighbor, _) in neighbors {
                    if !visited.get(&neighbor).unwrap_or(&false)
                        && *self.residual.get(&(current, neighbor)).unwrap_or(&0) > 0
                    {
                        parent.insert(neighbor, current);
                        if neighbor == sink {
                            return true;
                        }
                        visited.insert(neighbor, true);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        false
    }

    pub fn max_flow(&mut self, source: usize, sink: usize) -> i32 {
        let mut max_flow = 0;
        let mut parent = HashMap::new();
    
        while self.bfs(source, sink, &mut parent) {
            // Find bottleneck capacity
            let mut path_flow = i32::MAX;
            let mut current = sink;
    
            while current != source {
                let prev = parent[&current];
                path_flow = path_flow.min(*self.residual.get(&(prev, current)).unwrap());
                current = prev;
            }
    
            // Update residual capacities
            current = sink;
            while current != source {
                let prev = parent[&current];
                // Decrease forward capacity
                *self.residual.get_mut(&(prev, current)).unwrap() -= path_flow;
                // Increase reverse capacity
                *self.residual.get_mut(&(current, prev)).unwrap() += path_flow;
                current = prev;
            }
    
            max_flow += path_flow;
    
            // Debug: Log residual graph after each augmenting path
            eprintln!("Augmenting path added with flow: {}", path_flow);
            eprintln!("Residual capacities: {:?}", self.residual);
        }
    
        max_flow
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edmonds_karp_solver() {
        let mut ek = EdmondsKarp::new();
        ek.add_edge(0, 1, 16);
        ek.add_edge(0, 2, 13);
        ek.add_edge(1, 2, 10);
        ek.add_edge(1, 3, 12);
        ek.add_edge(2, 4, 14);
        ek.add_edge(3, 5, 20);
        ek.add_edge(4, 3, 7);
        ek.add_edge(4, 5, 4);

        let max_flow = ek.max_flow(0, 5);
        assert_eq!(max_flow, 23);
    }
}
