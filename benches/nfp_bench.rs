use criterion::{criterion_group, criterion_main, Criterion};
use petgraph::visit::EdgeRef;
use cpoc_revision::graph_generator::GraphConfig;
use cpoc_revision::edmonds_karp_solver::EdmondsKarp;
use cpoc_revision::verifier::verify_max_flow;

const DENSITY:f64 = 0.25;
const RAND_SEED:u64 = 42;

fn bench_1k(c: &mut Criterion) {
    let config = GraphConfig::new(1_000, (1, 100), RAND_SEED, DENSITY);
    let graph = config.create_random_flow_graph();
    let source = 0;
    let sink = 999;

    c.bench_function("Edmonds-Karp: 1,000 vertices", |b| {
        b.iter(|| {
            let mut ek = EdmondsKarp::new();
            for edge in graph.edge_references() {
                ek.add_edge(edge.source().index(), edge.target().index(), *edge.weight() as i32);
            }
            let max_flow = ek.max_flow(source, sink);
            criterion::black_box(max_flow); // Prevent compiler optimizations
        });
    });
}

/*
fn bench_verifier(c: &mut Criterion) {
    let config = GraphConfig::new(1_000, (1, 100), RAND_SEED, DENSITY);
    let graph = config.create_random_flow_graph();
    let source = 0;
    let sink = 999;

    let mut ek = EdmondsKarp::new();
    for edge in graph.edge_references() {
        ek.add_edge(edge.source().index(), edge.target().index(), *edge.weight() as i32);
    }
    let max_flow = ek.max_flow(source, sink);

    c.bench_function("Verifier: 1,000 vertices", |b| {
        b.iter(|| {
            let result = verify_max_flow(&ek, source, sink, max_flow);
            criterion::black_box(result); // Prevent compiler optimizations
        });
    });
}
*/

fn bench_10k(c: &mut Criterion) {
    let config = GraphConfig::new(10_000, (1, 100), RAND_SEED, DENSITY); 
    let graph = config.create_random_flow_graph();
    let source = 0;
    let sink = 9999;

    c.bench_function("Edmonds-Karp: 10,000 vertices", |b| {
        b.iter(|| {
            let mut ek = EdmondsKarp::new();
            for edge in graph.edge_references() {
                ek.add_edge(edge.source().index(), edge.target().index(), *edge.weight() as i32);
            }
            let max_flow = ek.max_flow(source, sink);
            criterion::black_box(max_flow); // Prevent compiler optimizations
        });
    });
}

fn bench_100k(c: &mut Criterion) {
    let config = GraphConfig::new(100_000, (1, 100), RAND_SEED, DENSITY); 
    let graph = config.create_random_flow_graph();
    let source = 0;
    let sink = 9999;

    c.bench_function("Edmonds-Karp: 10,000 vertices", |b| {
        b.iter(|| {
            let mut ek = EdmondsKarp::new();
            for edge in graph.edge_references() {
                ek.add_edge(edge.source().index(), edge.target().index(), *edge.weight() as i32);
            }
            let max_flow = ek.max_flow(source, sink);
            criterion::black_box(max_flow); // Prevent compiler optimizations
        });
    });
}

// criterion_group!(benches, bench_edmonds_karp, bench_verifier, bench_large_graph);
criterion_group!(benches, bench_1k, bench_10k, bench_100k);
criterion_main!(benches);
