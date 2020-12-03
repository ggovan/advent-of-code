use advent_of_code::aoc_2020;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = aoc_2020::day_1::load().unwrap();
    c.bench_function("2020 1 1", |b| {
        b.iter(|| aoc_2020::day_1::part_1(&input, 2020))
    });
    c.bench_function("2020 1 2", |b| b.iter(|| aoc_2020::day_1::part_2(&input)));

    let input = aoc_2020::day_02::load().unwrap();
    c.bench_function("2020 2 1", |b| b.iter(|| aoc_2020::day_02::part_1(&input)));
    c.bench_function("2020 2 2", |b| b.iter(|| aoc_2020::day_02::part_2(&input)));

    let input = aoc_2020::day_03::load().unwrap();
    c.bench_function("2020 3 1", |b| b.iter(|| aoc_2020::day_03::part_1(&input)));
    c.bench_function("2020 3 2", |b| b.iter(|| aoc_2020::day_03::part_2(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
