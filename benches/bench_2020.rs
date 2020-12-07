use advent_of_code::aoc_2020;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    bench::<aoc_2020::Day01>(c);
    bench::<aoc_2020::Day02>(c);
    bench::<aoc_2020::Day03>(c);
    bench::<aoc_2020::Day04>(c);
    bench::<aoc_2020::Day05>(c);
    bench::<aoc_2020::Day06>(c);
    bench::<aoc_2020::Day07>(c);
}

fn bench<Day: aoc_2020::Aoc2020>(c: &mut Criterion) {
    let input = Day::load().unwrap();
    c.bench_function(&format!("2020 {} 1", Day::day()), |b| {
        b.iter(|| Day::part_1(&input))
    });
    c.bench_function(&format!("2020 {} 2", Day::day()), |b| {
        b.iter(|| Day::part_2(&input))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
