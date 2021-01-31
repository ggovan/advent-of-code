use aoc_2019;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1).and_then(|n| n.parse::<usize>().ok());

    aoc_2019::main(day)
}
