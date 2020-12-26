use super::intcode;
use crate::aoc_2020::Aoc2020;
use crate::common::geometry::*;
use crate::files::Res;
use std::collections::HashMap;

pub struct Day17;

type Map = HashMap<Point2D<i64>, char>;

impl Aoc2020 for Day17 {
    type Input = Vec<i64>;
    type Result1 = u64;
    type Result2 = u64;

    fn day() -> usize {
        17
    }
    fn load() -> Res<Self::Input> {
        intcode::Machine::load_tape_from_file("data/2019/day_17.in")
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        let mut machine = intcode::Machine::new(input, vec![]);
        machine.run_to_completion();
        let mut map: Map = HashMap::new();

        let mut row = 0;
        let mut col = 0;
        for v in machine.output.iter() {
            let c = *v as u8 as char;
            if c == '\n' {
                row += 1;
                col = 0;
                continue;
            } else if c != '.' {
                map.insert(Point2D(col, row), c);
            }
            col += 1;
        }
        output_map(&map);

        // let (w1, w2) = map_to_line_segs(&map);
        // let points = day_3::Wire::intersection_points(&w1, &w2, false);

        let mut pos = map.iter().find(|&(_, v)| *v == '^').unwrap();
        // let mut dir = Dir

        // points.iter().map(|Point(x, y)| *x * *y).sum::<i32>() as u64
        17
    }

    fn part_2(_input: &Self::Input) -> Self::Result2 {
        17
    }
}
