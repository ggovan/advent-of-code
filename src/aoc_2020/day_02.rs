use crate::files::{read_lines, Res};
use std::str::FromStr;

pub fn day_02() -> Res<()> {
    println!("Day 2");

    let input = read_lines("data/2020/day_02.in")?
        .map(|l| l.unwrap().parse::<PasswordRule>().unwrap())
        .collect::<Vec<_>>();

    println!("  part 1: {}", part_1(&input));
    println!("  part 2: {}", part_2(&input));

    Ok(())
}

struct PasswordRule {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl FromStr for PasswordRule {
    type Err = Box<dyn std::error::Error>;

    fn from_str(input: &str) -> Res<Self> {
        let (min, rest) = split_two(input, "-");
        let (max, rest) = split_two(rest, " ");
        let (letter, password) = split_two(rest, ": ");
        Ok(PasswordRule {
            min: min.trim().parse()?,
            max: max.trim().parse()?,
            letter: letter.trim().parse()?,
            password: password.trim().parse()?,
        })
    }
}

fn split_two<'a>(s: &'a str, split_on: &str) -> (&'a str, &'a str) {
    let mut iter = s.splitn(2, split_on);
    (iter.next().unwrap(), iter.next().unwrap())
}

fn part_1(input: &[PasswordRule]) -> usize {
    input
        .iter()
        .filter(|pr| {
            let count = pr.password.chars().filter(|c| *c == pr.letter).count();
            count >= pr.min && count <= pr.max
        })
        .count()
}

fn part_2(input: &[PasswordRule]) -> usize {
    input
        .iter()
        .filter(|pr| {
            // indexing strings is a pain due to utf-8, just use bytes
            let bytes = pr.password.as_bytes();
            let target = pr.letter as u8;
            (bytes[pr.min - 1] == target) ^ (bytes[pr.max - 1] == target)
        })
        .count()
}
