use advent_of_code::aoc_2020;
use advent_of_code::files::Res;
use std::env;

#[macro_use]
extern crate lazy_static;

fn main() -> Res<()> {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1).and_then(|n| n.parse::<usize>().ok());

    aoc_2020::run_all(day)
}
