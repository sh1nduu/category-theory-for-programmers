use category_theory::memoize;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fib(n: i64) -> i64 {
    match n {
        0 | 1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

fn bm1(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fib(black_box(20))));
}

fn bm2(c: &mut Criterion) {
    let mut fib_memo = memoize(fib);
    c.bench_function("fib memo 20", |b| b.iter(|| fib_memo(black_box(20))));
}

criterion_group!(benches, bm1, bm2);
criterion_main!(benches);
