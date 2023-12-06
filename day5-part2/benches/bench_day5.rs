use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day5_part2::do_aoc;
use std::fs;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = fs::read_to_string("input").unwrap();
    c.bench_function("do_aoc", |b| b.iter(|| do_aoc(black_box(&input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
