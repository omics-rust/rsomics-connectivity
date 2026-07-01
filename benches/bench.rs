use criterion::{criterion_group, criterion_main, Criterion};
use rsomics_connectivity::{edge_connectivity_from_edge_list, node_connectivity_from_edge_list};
use std::hint::black_box;

const GNM_150_600: &str = include_str!("gnm_150_600.txt");

fn bench(c: &mut Criterion) {
    c.bench_function("edge_connectivity_gnm_150_600", |b| {
        b.iter(|| edge_connectivity_from_edge_list(black_box(GNM_150_600)));
    });
    c.bench_function("node_connectivity_gnm_150_600", |b| {
        b.iter(|| node_connectivity_from_edge_list(black_box(GNM_150_600)));
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
