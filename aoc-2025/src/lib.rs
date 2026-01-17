#![feature(binary_heap_into_iter_sorted)]
#![feature(binary_heap_drain_sorted)]
#![feature(trim_prefix_suffix)]
extern crate lazy_static;

use aoc_common::aoc_day::AocDay;

// Days 01 - 03 done in browser console
mod day_04;
pub use day_04::Day04;
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

use aoc_common::files::Res;
use aoc_common::time_async;

pub async fn run_all(day: Option<usize>) -> Res<()> {
    let (_, t): (Res<()>, _) = time_async(|| async move {
        let handles = vec![
            tokio::spawn(async move { Day04::run_me_maybe(day) }),
            tokio::spawn(async move { Day06::run_me_maybe(day) }),
            tokio::spawn(async move { Day07::run_me_maybe(day) }),
            tokio::spawn(async move { Day08::run_me_maybe(day) }),
            tokio::spawn(async move { Day09::run_me_maybe(day) }),
            tokio::spawn(async move { Day10::run_me_maybe(day) }),
            tokio::spawn(async move { Day11::run_me_maybe(day) }),
            tokio::spawn(async move { Day12::run_me_maybe(day) }),
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

pub fn run_all_sync(day: Option<usize>) -> Res<()> {
    for d in [
        Day04::run_me_maybe(day)?,
        Day06::run_me_maybe(day)?,
        Day07::run_me_maybe(day)?,
        Day08::run_me_maybe(day)?,
        Day09::run_me_maybe(day)?,
        Day10::run_me_maybe(day)?,
        Day11::run_me_maybe(day)?,
        Day12::run_me_maybe(day)?,
    ] {
        let output = d;
        for l in output {
            println!("{}", l);
        }
    }

    // println!("Total time: {:?}", t);

    Ok(())
}
