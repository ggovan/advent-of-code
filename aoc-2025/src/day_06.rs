use aoc_common::aoc_day::AocDay;
use aoc_common::files::Res;
use std::fs::read_to_string;

pub struct Day06;

fn fast_parse(s: &str) -> i64 {
    let mut res: i64 = 0;
    for b in s.bytes() {
        if b >= b'0' && b <= b'9' {
            res = res * 10 + (b - b'0') as i64;
        }
    }
    res
}

impl AocDay for Day06 {
    type Input = String;
    type Result1 = i64;
    type Result2 = i64;

    fn day() -> usize {
        06
    }

    fn load() -> Res<Self::Input> {
        let _input = r"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
        let input = read_to_string("data/2025/day_06.in")?;
        Ok(input.to_string())
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        let input: Vec<&str> = input.split("\n").collect();

        let ops = input
            .last()
            .unwrap()
            .bytes()
            .filter(|b| *b != b' ')
            .collect::<Vec<u8>>();
        let len = ops.len();

        let input: Vec<Vec<i64>> = input
            .iter()
            .take(input.len() - 1)
            .map(|line| {
                line.split(" ")
                    .filter(|v| *v != "")
                    .map(|v| fast_parse(v))
                    .collect()
            })
            .collect();

        let mut sum: i64 = 0;
        for i in 0..len {
            let op = ops[i];
            let mut sum_single: i64 = if op == b'*' { 1 } else { 0 };
            for j in 0..(input.len()) {
                match op {
                    b'+' => {
                        let val = input[j][i];
                        sum_single += val;
                    }
                    b'*' => {
                        let val = input[j][i];
                        sum_single *= val;
                    }
                    _ => unreachable!(),
                }
            }

            sum += sum_single;
        }

        sum
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        let input: Vec<Vec<u8>> = input
            .split("\n")
            .map(|line| line.bytes().collect())
            .collect();
        let ops = input.last().unwrap();
        let len = ops.len();

        let mut sum: i64 = 0;
        let mut operands: Vec<i64> = vec![];
        for i in (0..len).rev() {
            let mut operand = 0;
            for j in 0..(input.len()) {
                let b = input[j][i];
                if b >= b'0' && b <= b'9' {
                    operand = operand * 10 + (b - b'0') as i64;
                }
            }

            if operand != 0 {
                operands.push(operand);
            }

            let op = ops[i];
            match op {
                b' ' => {
                    continue;
                }
                b'+' => {
                    sum += operands.iter().sum::<i64>();
                    operands.clear();
                }
                b'*' => {
                    sum += operands.iter().product::<i64>();
                    operands.clear();
                }
                _ => unreachable!(),
            }
        }

        sum
    }
}
