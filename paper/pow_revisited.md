# Revisiting cPoC And RandomX

Discussion note
wbb, 11/20/24

## Overview

When designing Fluence's Proof of Capacity for CPUs (cPOC), a significant amount of work went into choosing the Proof of Work (PoW) algorithm. Specifically, the PoW algorithms needed to not only perform on CPUs but outperform GPUs while making it sufficiently expensive to be FPGA and ASIC resistance. At the time, RandomX [], which is actively used by Montero and other blockchains, came ready to use off-the -shelf with a strong performance history and checked all the boxes. As a result, RandomX was selected.

While RandomX performs well in terms of hashing, its implementation for efficient on-chain verification has proven challenging. The heavy reliance on floating-point operations, coupled with the time-consuming initialization of the RandomX verifier, has made direct usage impractical. As a result, zero-knowledge proofs (ZKPs) have been explored as a solution to enable on-chain verification. However, integrating ZKPs—whether it's Risco-0 [] or SP1 []—has introduced its own set of challenges and continues to slow down progress.

Given the persistence of these challenges and the need for eventual on-chain verification, it seems prudent to take a step back and explore alternatives to RandomX, rather than to continue to seeking another tool for efficient on-chain RandomX verification. Notably, cPoC was designed to support "pluggable" PoW choices, and we should be cognizant of this flexibility.

For the remainder of this note, we will revisit the key attributes of compute algorithms that, when deployed on CPUs, outperform GPU-based implementations, while also offering some degree of resistance to FPGAs and ASICs. Specifically, we propose the network flow graph (NFG) problem as a solution candidate and discuss the conditions under which NFG is suitable for CPU deployment and can outperform GPU implementations. We will also examine its compatibility with ZKP-style proofs, such as zk-STARKs, and its resistance to FPGA and ASIC optimization.

## CPU-Optimized Compute

For the CPU side of the marketplace, we continue to require a PoW system that proves the availability of CPUs to the network, as we are not interested in rewarding hardware that either doesn't exist or is unusable by the customer eventually renting it. To this end, we are seeking compute algorithms that perform significantly better, all else being equal, on CPUs than on GPUs. Additionally, we want the algorithm to be easily adjustable or possess other characteristics that make it costly to implement on FPGA and ASIC hardware.

What makes a compute algorithm CPU-friendly and GPU-unfriendly? There are actually quite a number of attributes but we'll focus on the most important ones:

1. Serial processing
2. Irregular memory access
3. Complex branching and control flows
4. Low compute-to-memory ratio
5. High synchronization and coordination among threads
6. Use os hierarchical caches

Hence, the intersection of CPU-friendly and GPU-unfriendly algorithms typically occurs in problems characterized by low parallelism, sequential dependencies, irregular memory access patterns, or complex control flow. RandomX, for example, leverages several of these attributes to achieve its comparative CPU over GPU advantage.

Based on these characteristics, problems such as graph traversal, dynamic programming with recursive dependencies, linear programming with sparse matrices (especially irregular ones), simulations with complex interdependencies (e.g., Monte Carlo methods), pathfinding algorithms (e.g., Dijkstra's), and non-parallelizable cryptographic algorithms (e.g., elliptic curve cryptography and RSA) can, in some form, be considered CPU-friendly and GPU-unfriendly.

Additionally, we must consider further constraints: verifying the result of an algorithm should be faster than actually computing it and there should be minimal effort required to create specialized hardware, such as ASICs or FPGAs, to solve the problem as is increasingly the case with many GPU-unfriendly cryptographic algorithms.

### Network Flow Problem

The Network Flow problem (NFP) involves finding the maximum flow of resources from a source to a sink in a directed graph, where each edge has a capacity. The objective is to maximize the flow while ensuring that the flow through each edge does not exceed its capacity and that flow is conserved at all intermediate nodes. 
A flow network $N$ can be defined as a tuple: $N = (G, c, s, t)$, where
* $G = (V, E)$ is a directed graph with vertices $V$ and edges $E$
* $c: E \rightarrow \mathbb{R}_0^+$ assigns a non-negative capacity to each edge
* $s$ and $t$ are specific vertices in $G$, representing the source and sink respectively.


The NFP is commonly used in supply chain management, transportation, telecom, social network analysis and more.

Of course, there are multiple algorithms to solve the problem including the Edmonds-Karp
implementation of the Ford-Fulkeron algorithm and the Push-Relable algorithm. Edmonds-Karp relies on breadth-first-search (BFS) and the sequential nature favors CPUs, while Push-Relable 
Not surprisingly, there is a parallelized verison of Push-Relable that can provide significant speed-ups on GPUs given the network (problem) is large enough, i.e., more than 10 million nodes + edges


