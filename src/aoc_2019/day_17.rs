use super::day_3::{self, *};
use super::intcode::*;
use crate::aoc_2020::Aoc2020;
use crate::files::Res;
use std::collections::HashMap;

pub struct Day17;

type Map = HashMap<(i64, i64), char>;

impl Aoc2020 for Day17 {
    type Input = Vec<i64>;
    type Result1 = u64;
    type Result2 = u64;

    fn day() -> usize {
        17
    }
    fn load() -> Res<Self::Input> {
        Machine::load_tape_from_file("data/2019/day_17.in")
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        let mut machine = Machine::new(input, vec![]);
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
                map.insert((col, row), c);
            }
            col += 1;
        }
        output_map(&map);

        let (w1, w2) = map_to_line_segs(&map);
        let points = day_3::Wire::intersection_points(&w1, &w2, false);

        points.iter().map(|Point(x, y)| *x * *y).sum::<i32>() as u64
    }

    fn part_2(_input: &Self::Input) -> Self::Result2 {
        17
    }
}

enum State {
    Empty,
    Wire(i64),
}

fn map_to_line_segs(map: &Map) -> (day_3::Wire, day_3::Wire) {
    use State::*;

    let (x_min, x_max, y_min, y_max) = map_bounds(map);

    let mut line_segs = Vec::new();

    for x in x_min..=x_max {
        let mut state = Empty;
        // go 1 pass so we close lines that end on the last row.
        for y in y_min..=y_max + 1 {
            state = match (state, map.get(&(x, y))) {
                (Empty, None) => Empty,
                (Empty, Some(_)) => Wire(y),
                (Wire(start), Some(_)) => Wire(start),
                (Wire(start), None) if y - 1 == start => Empty, // crossing over a wire in the other direction
                (Wire(start), None) => {
                    line_segs.push(LineSeg {
                        start: Point(x as i32, start as i32),
                        direction: Direction::V,
                        length: (y - start - 1) as i32,
                    });
                    Empty
                }
            }
        }
    }

    let verticals = day_3::Wire {
        segments: line_segs,
    };

    let mut line_segs = Vec::new();

    for y in y_min..=y_max {
        let mut state = Empty;
        // go 1 pass so we close lines that end on the last row.
        for x in x_min..=x_max + 1 {
            state = match (state, map.get(&(x, y))) {
                (Empty, None) => Empty,
                (Empty, Some(_)) => Wire(x),
                (Wire(start), Some(_)) => Wire(start),
                (Wire(start), None) if x - 1 == start => Empty, // crossing over a wire in the other direction
                (Wire(start), None) => {
                    line_segs.push(LineSeg {
                        start: Point(start as i32, y as i32),
                        direction: Direction::H,
                        length: (x - start - 1) as i32,
                    });
                    Empty
                }
            }
        }
    }

    let horizontals = day_3::Wire {
        segments: line_segs,
    };

    (verticals, horizontals)
}
