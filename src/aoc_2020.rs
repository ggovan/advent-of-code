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
    Day06::run_me_maybe(day)?;
    Day07::run_me_maybe(day)?;
    Day08::run_me_maybe(day)?;
    Day09::run_me_maybe(day)?;
    Day10::run_me_maybe(day)?;
    Day11::run_me_maybe(day)?;
    Day12::run_me_maybe(day)?;
    Day13::run_me_maybe(day)?;
    Day14::run_me_maybe(day)?;

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
