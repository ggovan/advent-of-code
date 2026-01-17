use aoc_common::aoc_day::AocDay;
use aoc_common::files::Res;
use std::fs::read_to_string;
use std::mem::swap;

pub struct Day07;

impl AocDay for Day07 {
    type Input = Vec<Vec<u8>>;
    type Result1 = i64;
    type Result2 = i64;

    fn day() -> usize {
        7
    }

    fn load() -> Res<Self::Input> {
        Ok(parse(&read_to_string("data/2025/day_07.in")?))
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        let mut splits = 0;
        let mut lines = input.clone();

        for i in 1..lines.len() {
            let (above, below) = lines.split_at_mut(i);
            let line_above = &above[i - 1];
            let line = &mut below[0];
            for j in 0..line.len() {
                if b'.' == line[j] {
                    if line_above[j] == b'|' || line_above[j] == b'S' {
                        line[j] = b'|'
                    }
                } else if line[j] == b'^' {
                    if line_above[j] == b'|' {
                        if j > 0 {
                            line[j - 1] = b'|';
                        }
                        if j < line.len() - 1 {
                            line[j + 1] = b'|';
                        }
                        splits += 1;
                    }
                }
            }
        }
        splits
    }

    fn part_2(lines: &Self::Input) -> Self::Result2 {
        let mut out_line_above: Vec<i64> = lines[0]
                .iter()
                .map(|&v| if v == b'S' { 1 } else { 0 })
                .collect();
        let mut out_line: Vec<i64> = vec![0 ;out_line_above.len()];


        for i in 1..lines.len() {
            let line_above = &lines[i - 1];
            let line = &lines[i];

            for j in 0..line.len() {
                if line[j] != b'^' {
                    if line_above[j] != b'^' {
                        out_line[j] += out_line_above[j];
                    }
                } else if line[j] == b'^' {
                    if j > 0 {
                        out_line[j - 1] += out_line_above[j];
                    }
                    if j < line.len() - 1 {
                        out_line[j + 1] += out_line_above[j];
                    }
                }
            }

            swap(&mut out_line_above, &mut out_line);
            out_line.fill(0);
        }

        out_line_above
            .iter()
            .fold(0, |acc, v| acc + v)
    }
}

fn parse(input: &str) -> <Day07 as AocDay>::Input {
    input.lines().map(|line| line.bytes().collect()).collect()
}
