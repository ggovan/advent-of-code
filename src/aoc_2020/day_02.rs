use super::Aoc2020;
use crate::files::{read_lines, Res};
use lazy_static::*;
use regex::Regex;
use std::str::FromStr;

pub struct Day02;

impl Aoc2020 for Day02 {
    type Input = Vec<PasswordRule>;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        2
    }

    fn load() -> Res<Self::Input> {
        Ok(read_lines("data/2020/day_02.in")?
            .map(|l| l.unwrap().parse::<PasswordRule>().unwrap())
            .collect::<Vec<_>>())
    }

    fn part_1(input: &Vec<PasswordRule>) -> usize {
        input
            .iter()
            .filter(|pr| {
                let count = pr.password.chars().filter(|c| *c == pr.letter).count();
                count >= pr.min && count <= pr.max
            })
            .count()
    }

    fn part_2(input: &Vec<PasswordRule>) -> usize {
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
