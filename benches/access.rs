use criterion::{black_box, criterion_group, criterion_main, Criterion};
use webster::{preload, dictionary};

fn criterion_benchmark(c: &mut Criterion) {
  preload();

  c.bench_function("define rust", |b| b.iter(|| dictionary(black_box("rust"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);