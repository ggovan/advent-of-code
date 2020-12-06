use super::Aoc2020;
use crate::files::Res;
use std::fs::read_to_string;

pub struct Day05;

impl Aoc2020 for Day05 {
    type Input = Vec<String>;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        5
    }

    fn load() -> Res<Self::Input> {
        Ok(read_to_string("data/2020/day_05.in")?
            .lines()
            .map(|s| s.to_owned())
            .collect())
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        input.iter().map(|s| seat_number(s)).max().unwrap()
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        let mut seats: Vec<usize> = input.iter().map(|s| seat_number(s)).collect();
        seats.sort_unstable();

        for x in 0..(seats.len() - 1) {
            if seats[x] + 2 == seats[x + 1] {
                return seats[x] + 1;
            }
        }
        panic!("Seat number not found");
    }
}

fn seat_number(s: &str) -> usize {
    s.chars().fold(0, |n, c| {
        (n << 1) + if c == 'B' || c == 'R' { 1 } else { 0 }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(seat_number("BFFFBBFRRR"), 567);
        assert_eq!(seat_number("FFFBBBFRRR"), 119);
        assert_eq!(seat_number("BBFFBBFRLL"), 820);
    }
}
