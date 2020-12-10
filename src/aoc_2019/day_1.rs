use crate::aoc_2020::Aoc2020;
use crate::files::{read_lines, Res};

pub struct Day01;

impl Aoc2020 for Day01 {
    type Input = Vec<i32>;
    type Result1 = i32;
    type Result2 = i32;

    fn day() -> usize {
        1
    }
    fn load() -> Res<Self::Input> {
        Ok(read_lines("data/2019/day_1.in")?
            .map(|l| l.unwrap().trim().parse::<i32>().unwrap())
            .collect())
    }

    fn part_1(masses: &Self::Input) -> Self::Result1 {
        masses.iter().map(|m| m / 3 - 2).sum()
    }

    fn part_2(masses: &Vec<i32>) -> Self::Result2 {
        masses
            .iter()
            .map(|m| m / 3 - 2)
            .map(|m| m + day_1_part_2_fuel(m))
            .sum()
    }
}

fn day_1_part_2_fuel(mass: i32) -> i32 {
    let more_fuel = mass / 3 - 2;
    if more_fuel <= 0 {
        0
    } else {
        more_fuel + day_1_part_2_fuel(more_fuel)
    }
}
