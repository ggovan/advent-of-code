use super::Aoc2020;
use crate::files::Res;
use std::fs::read_to_string;

pub struct Day06;

impl Aoc2020 for Day06 {
    type Input = Vec<Vec<String>>;
    type Result1 = u32;
    type Result2 = u32;

    fn day() -> usize {
        6
    }

    fn load() -> Res<Self::Input> {
        Ok(parse_in(&read_to_string("data/2020/day_06.in")?))
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        input
            .iter()
            .map(|g| {
                g.iter()
                    .map(|p| {
                        p.chars()
                            .fold(0_u32, |acc, c| acc | 1 << (c as usize - 'a' as usize))
                    })
                    .fold(0_u32, |acc, p| acc | p)
                    .count_ones()
            })
            .sum()
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        input
            .iter()
            .map(|g| {
                g.iter()
                    .map(|p| {
                        p.chars()
                            .fold(0_u32, |acc, c| acc | (1 << (c as u32 - 'a' as u32)))
                    })
                    .fold(0xffffffff, |acc, c| acc & c)
                    .count_ones()
            })
            .sum()
    }
}

fn parse_in(s: &str) -> Vec<Vec<String>> {
    s.split("\n\n")
        .map(|s| {
            s.split('\n')
                .filter(|s| !s.is_empty())
                .map(|s2| s2.to_owned())
                .collect::<Vec<_>>()
        })
        .filter(|g| !g.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_part_1() {
        let input = parse_in(
            "abd\n\
            \n\
            a\n\
            b\n\
            c\n\
            \n\
            ab\n\
            ac\n\
            \n\
            a\n\
            a\n\
            a\n\
            a\n\
            \n\
            b",
        );
        assert_eq!(Day06::part_1(&input), 11);
    }

    #[test]
    fn test_example_part_2() {
        let input = parse_in(
            "abd\n\
            \n\
            a\n\
            b\n\
            c\n\
            \n\
            ab\n\
            ac\n\
            \n\
            a\n\
            a\n\
            a\n\
            a\n\
            \n\
            b",
        );
        assert_eq!(Day06::part_2(&input), 6);
    }
}
