use criterion::{black_box, criterion_group, criterion_main, Criterion};
use shogi_piece_values::simulate;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("100 iterations", |b| b.iter(|| simulate()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
