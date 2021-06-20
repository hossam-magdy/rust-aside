use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_workshop::p3_reverse_string::{reverse_string1, reverse_string2};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("reverse_string1", |b| {
        b.iter(|| reverse_string1(black_box("abc")))
    });
    c.bench_function("reverse_string2", |b| {
        b.iter(|| reverse_string2(black_box("abc")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
