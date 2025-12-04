extern crate lazy_static;

use aoc_common::aoc_day::AocDay;

// Days 01 - 03 done in browser console
mod day_04;
pub use day_04::Day04;

use aoc_common::files::Res;
use aoc_common::time_async;

pub async fn run_all(day: Option<usize>) -> Res<()> {
    let (_, t): (Res<()>, _) = time_async(|| async move {
        let handles = vec![
            tokio::spawn(async move { Day04::run_me_maybe(day) }),

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
