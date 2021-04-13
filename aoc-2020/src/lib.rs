#![feature(str_split_once)]
extern crate lazy_static;

use aoc_common::aoc_day::AocDay;

mod day_02;
pub use day_02::Day02;
mod day_03;
pub use day_03::Day03;
mod day_04;
pub use day_04::Day04;
mod day_05;
pub use day_05::Day05;
mod day_06;
pub use day_06::Day06;
mod day_07;
pub use day_07::Day07;
mod day_08;
pub use day_08::Day08;
mod day_09;
pub use day_09::Day09;
mod day_10;
pub use day_10::Day10;
mod day_11;
pub use day_11::Day11;
mod day_12;
pub use day_12::Day12;
mod day_13;
pub use day_13::Day13;
mod day_14;
pub use day_14::Day14;
mod day_15;
pub use day_15::Day15;
mod day_16;
pub use day_16::Day16;
mod day_17;
pub use day_17::Day17;
mod day_18;
pub use day_18::Day18;
mod day_19;
pub use day_19::Day19;
mod day_20;
pub use day_20::Day20;
mod day_21;
pub use day_21::Day21;
mod day_22;
pub use day_22::Day22;
mod day_23;
pub use day_23::Day23;
mod day_24;
pub use day_24::Day24;
mod day_25;
pub use day_25::Day25;
mod day_1;
pub use day_1::Day01;

use aoc_common::files::Res;
use aoc_common::time_async;

pub async fn run_all(day: Option<usize>) -> Res<()> {
    let (_, t): (Res<()>, _) = time_async(|| async move {
        let handles = vec![
            tokio::spawn(async move { Day01::run_me_maybe(day) }),
            tokio::spawn(async move { Day02::run_me_maybe(day) }),
            tokio::spawn(async move { Day03::run_me_maybe(day) }),
            tokio::spawn(async move { Day04::run_me_maybe(day) }),
            tokio::spawn(async move { Day05::run_me_maybe(day) }),
            tokio::spawn(async move { Day06::run_me_maybe(day) }),
            tokio::spawn(async move { Day07::run_me_maybe(day) }),
            tokio::spawn(async move { Day08::run_me_maybe(day) }),
            tokio::spawn(async move { Day09::run_me_maybe(day) }),
            tokio::spawn(async move { Day10::run_me_maybe(day) }),
            tokio::spawn(async move { Day11::run_me_maybe(day) }),
            tokio::spawn(async move { Day12::run_me_maybe(day) }),
            tokio::spawn(async move { Day13::run_me_maybe(day) }),
            tokio::spawn(async move { Day14::run_me_maybe(day) }),
            tokio::spawn(async move { Day15::run_me_maybe(day) }),
            tokio::spawn(async move { Day16::run_me_maybe(day) }),
            tokio::spawn(async move { Day17::run_me_maybe(day) }),
            tokio::spawn(async move { Day18::run_me_maybe(day) }),
            tokio::spawn(async move { Day19::run_me_maybe(day) }),
            tokio::spawn(async move { Day20::run_me_maybe(day) }),
            tokio::spawn(async move { Day21::run_me_maybe(day) }),
            tokio::spawn(async move { Day22::run_me_maybe(day) }),
            tokio::spawn(async move { Day23::run_me_maybe(day) }),
            tokio::spawn(async move { Day24::run_me_maybe(day) }),
            tokio::spawn(async move { Day25::run_me_maybe(day) }),
        ];
        for h in handles {
            let output = h.await.unwrap()?;
            for l in output {
                println!("{}", l);
            }
        }
        Ok(())
    })
    .await;

    println!("Total time: {:?}", t);

    Ok(())
}
