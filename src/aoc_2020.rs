use std::env;

mod day_02;
mod day_1;
use crate::files::Res;

pub fn run_all() -> Res<()> {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1).map(|n| n.parse::<usize>().unwrap());

    if Some(1) == day || day.is_none() {
        day_1::day_1()?;
    }

    if Some(2) == day || day.is_none() {
        day_02::day_02()?;
    }

    Ok(())
}
