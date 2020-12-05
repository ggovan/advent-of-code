mod day_02;
pub use day_02::Day02;
mod day_03;
pub use day_03::Day03;
mod day_04;
pub use day_04::Day04;
mod day_05;
pub use day_05::Day05;
mod day_1;
pub use day_1::Day01;

use crate::files::Res;
use std::fmt::Display;

pub fn run_all(day: Option<usize>) -> Res<()> {
    Day01::run_me_maybe(day)?;
    Day02::run_me_maybe(day)?;
    Day03::run_me_maybe(day)?;
    Day04::run_me_maybe(day)?;
    Day05::run_me_maybe(day)?;

    Ok(())
}

pub trait Aoc2020 {
    type Input;
    type Result1: Display;
    type Result2: Display;

    fn day() -> usize;
    fn load() -> Res<Self::Input>;
    fn part_1(input: &Self::Input) -> Self::Result1;
    fn part_2(input: &Self::Input) -> Self::Result2;

    fn run() -> Res<()> {
        println!("Day {}", Self::day());

        let input = Self::load()?;

        println!("  part 1: {}", Self::part_1(&input));
        println!("  part 2: {}", Self::part_2(&input));

        Ok(())
    }

    fn run_me_maybe(day: Option<usize>) -> Res<()> {
        if Some(Self::day()) == day || day.is_none() {
            Self::run()?;
        }
        Ok(())
    }
}
