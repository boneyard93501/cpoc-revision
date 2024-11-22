use crate::edmonds_karp_solver::EdmondsKarp;
use std::collections::HashMap;

pub fn verify_max_flow(
    ek: &EdmondsKarp,
    source: usize,
    sink: usize,
    expected_flow: i32,
) -> bool {
    let mut flow_balance: HashMap<usize, i32> = HashMap::new();
    let mut source_outflow = 0;
    let mut sink_inflow = 0;

    // Traverse the residual graph to calculate flow balances
    for (&(from, to), &residual_capacity) in &ek.residual {
        if let Some(initial_capacity) = ek
            .graph
            .get(&from)
            .and_then(|edges| edges.iter().find(|&&(target, _)| target == to))
            .map(|&(_, capacity)| capacity)
        {
            // Flow is the difference between initial and residual capacities
            let flow = initial_capacity - residual_capacity;

            if flow > 0 {
                // Update flow balance for nodes
                *flow_balance.entry(from).or_insert(0) -= flow;
                *flow_balance.entry(to).or_insert(0) += flow;

                // Track source outflow and sink inflow
                if from == source {
                    source_outflow += flow;
                }
                if to == sink {
                    sink_inflow += flow;
                }
            }
        }
    }

    // Debug output (can be removed in production)
    //eprintln!("Flow balances: {:?}", flow_balance);
    //eprintln!("Source outflow: {}", source_outflow);
    //eprintln!("Sink inflow: {}", sink_inflow);

    // Verify source and sink flow match the expected max flow
    if source_outflow != expected_flow {
        eprintln!(
            "Source flow mismatch: {} != {}",
            source_outflow, expected_flow
        );
        return false;
    }

    if sink_inflow != expected_flow {
        eprintln!(
            "Sink flow mismatch: {} != {}",
            sink_inflow, expected_flow
        );
        return false;
    }

    // Verify flow conservation at intermediate nodes
    for (&node, &balance) in &flow_balance {
        if node != source && node != sink && balance != 0 {
            eprintln!(
                "Flow conservation violated at node {}: balance = {}",
                node, balance
            );
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::edmonds_karp_solver::EdmondsKarp;

    #[test]
    fn test_verify_max_flow() {
        let mut ek = EdmondsKarp::new();
        ek.add_edge(0, 1, 16);
        ek.add_edge(0, 2, 13);
        ek.add_edge(1, 2, 10);
        ek.add_edge(1, 3, 12);
        ek.add_edge(2, 4, 14);
        ek.add_edge(3, 5, 20);
        ek.add_edge(4, 3, 7);
        ek.add_edge(4, 5, 4);

        // Compute max flow
        let max_flow = ek.max_flow(0, 5);
        assert_eq!(max_flow, 23);

        // Verify the flow
        assert!(verify_max_flow(&ek, 0, 5, max_flow));
    }
}