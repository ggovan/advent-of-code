use crate::files::{read_lines, Res};
use lazy_static::*;
use regex::Regex;
use std::str::FromStr;

pub fn day_02() -> Res<()> {
    println!("Day 2");

    let input = load()?;

    println!("  part 1: {}", part_1(&input));
    println!("  part 2: {}", part_2(&input));

    Ok(())
}

pub fn load() -> Res<Vec<PasswordRule>> {
    Ok(read_lines("data/2020/day_02.in")?
        .map(|l| l.unwrap().parse::<PasswordRule>().unwrap())
        .collect::<Vec<_>>())
}

pub struct PasswordRule {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl FromStr for PasswordRule {
    type Err = Box<dyn std::error::Error>;

    fn from_str(input: &str) -> Res<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();
        }

        let captures = RE.captures(input).unwrap();

        Ok(PasswordRule {
            min: captures[1].parse()?,
            max: captures[2].parse()?,
            letter: captures[3].parse()?,
            password: captures[4].parse()?,
        })
    }
}

pub fn part_1(input: &[PasswordRule]) -> usize {
    input
        .iter()
        .filter(|pr| {
            let count = pr.password.chars().filter(|c| *c == pr.letter).count();
            count >= pr.min && count <= pr.max
        })
        .count()
}

pub fn part_2(input: &[PasswordRule]) -> usize {
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
