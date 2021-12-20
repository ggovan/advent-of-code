use aoc_common::aoc_day::AocDay;
use aoc_common::files::Res;
use aoc_common::geometry::Direction;
use aoc_common::search::{search, HeapElem};
use std::collections::HashMap;
use std::fs::read_to_string;

pub struct Day15;

impl AocDay for Day15 {
    type Input = Vec<Vec<i64>>;
    type Result1 = i64;
    type Result2 = i64;

    fn day() -> usize {
        15
    }
    fn load() -> Res<Self::Input> {
        let input_str = read_to_string("data/2021/day_15.in")?;

        Ok(input_str
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i64).collect())
            .collect())
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        let end = (input[0].len() as i64 - 1, input.len() as i64 - 1);
        search(
            (0i64, 0i64),
            |p| *p == end,
            |p, distance| {
                Direction::array().iter().filter_map(move |direction| {
                    let successor = direction.next_point(p);
                    match successor {
                        (x, y) if x < 0 || y < 0 || x > end.0 || y > end.1 => None,
                        successor => Some(HeapElem {
                            elem: successor,
                            distance: distance
                                + input[successor.1 as usize][successor.0 as usize] as u64,
                            heuristic: ((end.0 - successor.0).abs() + (end.1 - successor.1).abs())
                                as u64,
                        }),
                    }
                })
            },
        )
        .1 as i64
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        let w_mod = input[0].len() as i64;
        let h_mod = input.len() as i64;
        let width = w_mod * 5;
        let height = h_mod * 5;
        let end = (width - 1, height - 1);
        search(
            (0i64, 0i64),
            |p| *p == end,
            |p, distance| {
                Direction::array().iter().filter_map(move |direction| {
                    let successor = direction.next_point(p);
                    match successor {
                        (x, y) if x < 0 || y < 0 || x == width || y == height => None,
                        successor => {
                            let w_wrap = successor.0 / w_mod;
                            let h_wrap = successor.1 / h_mod;
                            let o_v = input[(successor.1 % h_mod) as usize]
                                [(successor.0 % w_mod) as usize];
                            let n_v = o_v + w_wrap + h_wrap;
                            // I feel like this shouldn't work for 9+5+5
                            let v = if n_v > 9 { n_v - 9 } else { n_v };
                            Some(HeapElem {
                                elem: successor,
                                distance: distance + v as u64,
                                heuristic: ((end.0 - successor.0).abs()
                                    + (end.1 - successor.1).abs())
                                    as u64,
                            })
                        }
                    }
                })
            },
        )
        .1 as i64
    }
}
