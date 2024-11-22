use criterion::{criterion_group, criterion_main, Criterion};
use cpoc_revision::graph_generator::GraphConfig;
use cpoc_revision::edmonds_karp_solver::EdmondsKarp;
use petgraph::visit::EdgeRef;

fn bench_1000_vertices(c: &mut Criterion) {
    let config = GraphConfig::new(1000, 0.5, (1, 100), 42, 0.25); // 25% density, capacity range 1-100
    let graph = config.create_random_flow_graph();
    let source = 0;
    let sink = 999;

    c.bench_function("Edmonds-Karp: 1,000 vertices", |b| {
        b.iter(|| {
            let mut ek = EdmondsKarp::new();
            for edge in graph.edge_references() {
                ek.add_edge(
                    edge.source().index(), // Corrected source access
                    edge.target().index(), // Corrected target access
                    *edge.weight() as i32, // Access edge weight
                );
            }
            let max_flow = ek.max_flow(source, sink);
            criterion::black_box(max_flow); // Prevent compiler optimizations
        });
    });
}

fn bench_10000_vertices(c: &mut Criterion) {
    let config = GraphConfig::new(10000, 0.5, (1, 100), 42, 0.25); // 25% density, capacity range 1-100
    let graph = config.create_random_flow_graph();
    let source = 0;
    let sink = 9999;

    c.bench_function("Edmonds-Karp: 10,000 vertices", |b| {
        b.iter(|| {
            let mut ek = EdmondsKarp::new();
            for edge in graph.edge_references() {
                ek.add_edge(
                    edge.source().index(),
                    edge.target().index(),
                    *edge.weight() as i32,
                );
            }
            let max_flow = ek.max_flow(source, sink);
            criterion::black_box(max_flow);
        });
    });
}

fn bench_100000_vertices(c: &mut Criterion) {
    let config = GraphConfig::new(100000, 0.5, (1, 100), 42, 0.25); // 25% density, capacity range 1-100
    let graph = config.create_random_flow_graph();
    let source = 0;
    let sink = 99999;

    c.bench_function("Edmonds-Karp: 100,000 vertices", |b| {
        b.iter(|| {
            let mut ek = EdmondsKarp::new();
            for edge in graph.edge_references() {
                ek.add_edge(
                    edge.source().index(),
                    edge.target().index(),
                    *edge.weight() as i32,
                );
            }
            let max_flow = ek.max_flow(source, sink);
            criterion::black_box(max_flow);
        });
    });
}

criterion_group!(
    benches,
    bench_1000_vertices,
    bench_10000_vertices,
    bench_100000_vertices
);
criterion_main!(benches);