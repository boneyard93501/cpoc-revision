# Revisiting cPoC And RandomX

Discussion note
wbb, 11/20/24

## Overview

When designing Fluence's Proof of Capacity for CPUs (cPoC), considerable effort was dedicated to selecting the appropriate Proof of Work (PoW) algorithm. The chosen PoW algorithm needed to excel on CPUs while outperforming GPUs for comparable workloads. Additionally, it had to be sufficiently resource-intensive to resist FPGA and ASIC implementations. At that time, RandomX, which is actively utilized by Monero and other blockchains, was readily available off-the-shelf, had a proven performance history, and met all these criteria. Consequently, RandomX was selected as the algorithm for Fluence's cPoC.

While RandomX performs well in terms of hashing, its implementation for efficient on-chain verification has proven challenging. The heavy reliance on floating-point operations, coupled with the time-consuming initialization of the RandomX verifier, has made direct usage impractical. As a result, zero-knowledge proofs (ZKPs) have been explored as a solution to enable on-chain verification. However, integrating ZKPs—whether it's Risco-0 [] or SP1 []—has introduced its own set of challenges and continues to slow down progress. Moreover, at the time of this writing, RandomX is no longer ASIC-resistant, and an updated version has not been released. However, the modifications made by Fluence to the RandomX fork may significantly limit the potential for easy ASIC exploitation.

Given the ongoing challenges with on-chain verification of RandomX, it seems prudent to explore alternatives. Rather than continuing to seek another tool for efficient on-chain RandomX verification, we should consider other options. Notably, cPoC was designed to support "pluggable" PoW choices, and we should be mindful of this flexibility.

For the remainder of this note, we will revisit the key attributes of compute algorithms that can outperform GPU-based implementations when deployed on CPUs, while also providing some resistance to FPGAs and ASICs. Specifically, we propose the network flow graph (NFG) problem as a potential solution and discuss the conditions under which NFG is suitable for CPU deployment and can outperform GPU implementations. Additionally, we will examine its compatibility with zero-knowledge proof (ZKP) styles, such as zk-STARKs, as well as its resistance to FPGA and ASIC optimization.

## CPU-Optimized Compute

For the CPU side of the marketplace, we need a PoW system that demonstrates the availability of CPUs to the network. We are not interested in rewarding hardware that either doesn't exist or is ultimately unusable by the customers renting it. To achieve this, we are seeking compute algorithms that perform significantly better on CPUs than on GPUs, all else being equal. Additionally, we want the algorithm to be easily adjustable or to possess other characteristics that make implementation on FPGA and ASIC hardware costly.

What makes a compute algorithm CPU-friendly and GPU-unfriendly? There are quite a number of attributes but we'll focus on the most important ones:

1. Serial processing
2. Irregular memory access
3. Complex branching and control flows
4. Low compute-to-memory ratio
5. High synchronization and coordination among threads
6. Use os hierarchical caches

Hence, the intersection of CPU-friendly and GPU-unfriendly algorithms typically occurs in problems characterized by low parallelism, sequential dependencies, irregular memory access patterns, or complex control flow. RandomX, for example, leverages several of these attributes to achieve its comparative CPU over GPU advantage.

Based on these characteristics, problems such as graph traversal, dynamic programming with recursive dependencies, linear programming with sparse matrices (especially irregular ones), simulations with complex interdependencies (e.g., Monte Carlo methods), pathfinding algorithms (e.g., Dijkstra's) can be considered CPU-friendly and GPU-unfriendly.

Additionally, we must consider further constraints: verifying the result of an algorithm should be faster than computing it, and there should be minimal effort required to create specialized hardware, such as ASICs or FPGAs, to solve the problem.

### Network Flow Problem

The Network Flow problem (NFP) involves finding the maximum flow of resources from a source to a sink in a directed graph, where each edge has a capacity. The objective is to maximize the flow while ensuring that the flow through each edge does not exceed its capacity and that flow is conserved at all intermediate nodes. Inherently, the algorithms used to solve NFPs entail a great deal of serialization, relatively complex control flows and low(er) arithmetic intensity. The latter attribute is of further interest if we want to add memory (RAM) requirements to our PoW.

A flow network $N$ can be defined as a tuple: $N = (G, c, s, t)$, where

* $G = (V, E)$ is a directed graph with vertices $V$ and edges $E$
* $c: E \rightarrow \mathbb{R}_0^+$ assigns a non-negative capacity to each edge
* $s$ and $t$ are specific vertices in $G$, representing the source and sink respectively.

The NFP commonly occurs in the areas of supply chain management, transportation, telecom, social network analysis and more.

Of course, there are multiple algorithms to solve the problem including the Edmonds-Karp implementation of the Ford-Fulkeron algorithm and the Push-Relabel algorithm. Edmonds-Karp, which guarantees optimality if the computation completes, relies on breadth-first-search (BFS) with a time complexity of $(V E)$ and the sequential nature favors CPUs. Another algorithm, Push-Relabel, offers the same completeness guarantee and is available as a parallelized version of that can provide significant speed-ups on GPUs given the network (problem) is large enough, i.e., more than 10 million edges. Hence, we already have a constraint on our network size to be in the medium range, i.e., less than three or four million edges. Also note that the Push-Relabel has a time complexity of $O(V^2E)$, while the optimized version achieves $O(V^3)$. Hence, the improvements of Push-Relabel over Edmonds-Karp only com to bear for dense graphs where the ratio of edges to vertices is high. So our second constraint is that we want lower density graphs, which may align rather well with our memory (RAM) requirements.

From a verification perspective for a given graph and result, we need to check that the flow does not exceed capacity for each edge, $O(E)$, and that for every vertex, not including source and sink, the incoming flows equal the outgoing flows, $O(V)$, for a total time complexity of $O(V + E), which is a significant improvement over any of the compute algorithms.

Given our graph sizing and density requirements, Edmonds-Karp is most likely the best performing compute algorithm for the task at hand and will be used as the reference algorithm for the remainder of this note. See the [src directory](./../src/) for a demo implementation of graph-generation, solver and verifier. Note, the verifier currently takes the entire solver graph, which may not be necessary and can be fleshed out further if there is interest in adopting this solution.

#### Memory Considerations

An integral aspect of cPoC is the proving of CPU and RAM and at the very least, we should consider this verification attribute in alternative solution approaches, such as NFP. Most basically, the memory requirement to represent a flow graph depends on the number of edges, $E$, the bytes size of the flow parameter, the density of the graph and the storage type used. For example, adjacency matrices are constant in size regardless of graph density, $O(V^2)$, whereas adjacency lists are much more efficient. See Table 1.

Table 1: Complexity Considerations With Respect To Graph Density

| Operation                     | Adjacency Matrix (Dense Graph) | Adjacency Matrix (Sparse Graph) | Adjacency List (Dense Graph) | Adjacency List (Sparse Graph) |
|-------------------------------|---------------------------------|----------------------------------|-------------------------------|---------------------------------|
| **Adding a Vertex**           | $O(V^2)$                          | $O(V^2)$                           | $O(1)$                          | $O(1)$                           |
| **Removing a Vertex**         | $O(V^2)$                          | $O(V^2)$                           | $O(V + E)$                      | $O(V + E)$                      |
| **Adding an Edge**            | $O(1)$                            | $O(1)$                             | $O(1)$                          | $O(1)$                           |
| **Removing an Edge**          | $O(1)$                            | $O(1)$                             | $O(E)$                          | $O(E)$                           |
| **Querying for an Edge**      | $O(1)$                            | $O(1)$                             | $O(V)$                          | $O(d)$                           |
| **Finding Neighbors**         | $O(V)$                            | $O(V)$                             | $O(d)$                          | $O(d)$                           |

Assuming we are implementing the Edmonds-Karp algorithm using an adjacency list, four byte integers for flow and a 25% graph density, where we calculate $\text{Density} = \frac{2}{V(V-1)} E$, we get the following memory requirements for 100,000 edges for the solver and verifier, respectively:

* calculate edges: $E = 0.25 \times \frac{100,000(100,000 - 1)}{2} \approx 2,499,750,000$
* assuming each edge and node requires a four byte integer
* solver memory requirements:
    * $\text{Memory} = O(100,000 + 2,499,750,000) = O(2,499,850,000)$
    * $\text{Memory} = (100,000 \times 4) + (2,499,750,000 \times 8) \approx 20GB$
* verifier memory requirements:
    * $\text{Memory} = O(100,000) \approx 400 KB$

If i didn't mess up my quadratic solving, we require about 46,300 vertices for 2GB or memory and about 65,500 vertices for 4 GB of memory. The respective verifier requirements are approx. 185KB and 262 KB, respectively.


Overall, NFP and the Edmonds-Karp algorithm look like suitabkle candidates for a PoW solution for our cPoC. Of course, the seeding can be similar to the RandomX parameterization and we may retain a particular graph structure for an extended period of blocks and use a just a small jitter per (hashing) iteration to change flow value(s) between calculations.

#### Performance Considerations

We implemented [benches](./../benches/nfp_bench.rs) for 1,000, 10,000 and 100,000 vertices 

#### Proving In Zero Knowledge

Given a solution to the NFP problem, we cna use the result and the much more efficient verifier to implement a proof in zero knowledge. For illustrative purposes, wee examine a zk-STARK implementation for both the trace table and constraints. See Table 1.

Table 1: Stylized Trace Table Structure

 Column                | Description                                                       |
|-----------------------|-------------------------------------------------------------------|
| `edge_source`         | Index of the source node for an edge.                            |
| `edge_target`         | Index of the target node for an edge.                            |
| `initial_capacity`    | The original capacity of the edge.                               |
| `residual_capacity`   | The remaining capacity of the edge after flows.                  |
| `flow`                | The flow through the edge.                                       |
| `node_balance`        | Net balance of each node (\( \text{inflow} - \text{outflow} \)). |
| `cumulative_flow`     | Tracks cumulative flow from the source to validate the solution. |

For the corresponding interface, see Figure 1:

Figure 1: Trace Table Interface

```Rust
pub struct VerifierTrace {
    pub edge_source: Vec<BaseElement>,
    pub edge_target: Vec<BaseElement>,
    pub initial_capacity: Vec<BaseElement>,
    pub residual_capacity: Vec<BaseElement>,
    pub flow: Vec<BaseElement>,
    pub node_balance: Vec<BaseElement>,
    pub cumulative_flow: Vec<BaseElement>,
}
```

Based on the [test data](./../src/verifier.rs), we can populate the trace table, see Table 2.

Table 2: Populated Trace Table Corresponding To `verifier.rs` Tests

 Step | edge_source | edge_target | initial_capacity | residual_capacity | flow | node_balance | cumulative_flow |
|------|-------------|-------------|-------------------|-------------------|------|--------------|-----------------|
| 0    | 0           | 1           | 16                | 4                 | 12   | -12          | 12              |
| 0    | 0           | 2           | 13                | 9                 | 4    | -4           | 16              |
| 0    | 1           | 3           | 12                | 0                 | 12   | 0            | 16              |
| 0    | 2           | 4           | 14                | 10                | 4    | 0            | 16              |
| 1    | 4           | 3           | 7                 | 3                 | 4    | 0            | 16              |
| 1    | 3           | 5           | 20                | 16                | 4    | 4            | 16              |


See Figure 2 for the verifier trace pseudo-code.

Figure 2: Verifier Trace Pseudocode

```Rust
impl VerifierTrace {
    pub fn new(num_edges: usize, num_nodes: usize) -> Self {
        VerifierTrace {
            edge_source: vec![BaseElement::ZERO; num_edges],
            edge_target: vec![BaseElement::ZERO; num_edges],
            initial_capacity: vec![BaseElement::ZERO; num_edges],
            residual_capacity: vec![BaseElement::ZERO; num_edges],
            flow: vec![BaseElement::ZERO; num_edges],
            node_balance: vec![BaseElement::ZERO; num_nodes],
            cumulative_flow: vec![BaseElement::ZERO; num_edges],
        }
    }
    // recall that we may not need to take the entire EdmondsKarp graph. TBD!
    pub fn populate_from_verifier(&mut self, network: &EdmondsKarp, source: usize, sink: usize) {
        let mut cumulative_flow = BaseElement::ZERO;

        for (i, (&(from, to), &residual_capacity)) in network.residual.iter().enumerate() {
            let initial_capacity = network
                .graph
                .get(&from)
                .and_then(|edges| edges.iter().find(|&&(target, _)| target == to))
                .map(|&(_, capacity)| capacity)
                .unwrap_or(0);

            let flow = initial_capacity - residual_capacity;

            self.edge_source[i] = BaseElement::new(from as u128);
            self.edge_target[i] = BaseElement::new(to as u128);
            self.initial_capacity[i] = BaseElement::new(initial_capacity as u128);
            self.residual_capacity[i] = BaseElement::new(residual_capacity as u128);
            self.flow[i] = BaseElement::new(flow as u128);

            self.node_balance[from] -= BaseElement::new(flow as u128);
            self.node_balance[to] += BaseElement::new(flow as u128);

            if from == source {
                cumulative_flow += BaseElement::new(flow as u128);
            }
            self.cumulative_flow[i] = cumulative_flow;
        }
    }
}
```

The next step is to implement the AIR constraints. See Figure 3:

Figure 3: AIR Constraints Pseudocode

```Rust
pub struct VerifierAir {
    context: AirContext<BaseElement>,
    num_edges: usize,
    num_nodes: usize,
    max_flow: BaseElement,
}

impl Air for VerifierAir {
    fn evaluate_transition<F: FieldElement<BaseElement>>(
        &self,
        frame: &EvaluationFrame<BaseElement>,
        result: &mut [BaseElement],
    ) {
        let current = frame.current();
        let next = frame.next();

        // Residual capacity consistency
        let initial_minus_flow = current[COL_INITIAL_CAPACITY] - current[COL_FLOW];
        result[0] = initial_minus_flow - current[COL_RESIDUAL_CAPACITY];

        // Flow conservation for intermediate nodes
        let inflow = current[COL_NODE_BALANCE] + current[COL_FLOW];
        let outflow = current[COL_FLOW] - next[COL_NODE_BALANCE];
        result[1] = inflow - outflow;

        // Cumulative flow validation
        let cumulative_check = current[COL_CUMULATIVE_FLOW] - next[COL_CUMULATIVE_FLOW];
        result[2] = cumulative_check;
    }
}
```

## Summary

We presented a credible alternative to RandomX that entails generating and solving network flow problems. We illustrate that the network flow problem, when properly constrained with respect graph size and density, falls into the intersection of CPU-friendly and GPU-unfriendly deemed desirable for cPoC. Moreover, we introduce the Edmonds-Karp solver algorithm (supposedly) for small-to-midsize network flow graphs, illustrate its use of memory (RAM) requirements and demonstrate the comparative efficiency of a (general verifier). The latter insight is deemed important as the verifier can be used as the proof generator including proofs in zero knowledge. WE further demonstrate this approach with (pseudo-coded) zk-STARK trace table and AIR constraint designs and implementations. 