use criterion::{criterion_group, criterion_main, Criterion};
use shogi_piece_values::{simulate_n, simulate_n_par};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Simulate 100", |b| b.iter(|| simulate_n(100)));
    c.bench_function("Simulate 100 (parallel)", |b| {
        b.iter(|| simulate_n_par(100))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
