extern crate lazy_static;

use aoc_common::aoc_day::AocDay;

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

use aoc_common::files::Res;
use aoc_common::time_async;

pub async fn run_all(day: Option<usize>) -> Res<()> {
    let (_, t): (Res<()>, _) = time_async(|| async move {
        let handles = vec![
            tokio::spawn(async move { Day12::run_me_maybe(day) }),
            tokio::spawn(async move { Day13::run_me_maybe(day) }),
            tokio::spawn(async move { Day14::run_me_maybe(day) }),
            tokio::spawn(async move { Day15::run_me_maybe(day) }),
            tokio::spawn(async move { Day16::run_me_maybe(day) }),
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
