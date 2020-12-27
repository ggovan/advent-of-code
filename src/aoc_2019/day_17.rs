use super::intcode;
use crate::aoc_2020::Aoc2020;
use crate::common::geometry::{self, Direction, Point2D};
use crate::files::Res;
use std::collections::{HashMap, HashSet};

pub struct Day17;

type Map = HashMap<Point2D<i64>, char>;

impl Aoc2020 for Day17 {
    type Input = Vec<i64>;
    type Result1 = i64;
    type Result2 = i64;

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
        geometry::output_map(&map);

        let mut visited: HashSet<Point2D<i64>> = HashSet::new();
        let mut intersections: HashSet<Point2D<i64>> = HashSet::new();

        let mut pos = *map.iter().find(|&(_, v)| *v == '^').unwrap().0;
        let mut dir = Direction::West;

        let mut s: String = "L".to_string();

        let mut count = 0;

        while let Some((n_pos, n_dir)) = next_point_on_track(&map, pos, dir) {
            if n_dir != dir {
                s.push_str(&count.to_string());
                count = 0;
                s.push(if dir.rotate_cw() == n_dir { 'R' } else { 'L' });
            }
            count += 1;
            pos = n_pos;
            dir = n_dir;
            let is_intersected = !visited.insert(pos);
            if is_intersected {
                intersections.insert(pos);
            }
        }
        s.push_str(&count.to_string());

        println!("{}", s);

        intersections
            .iter()
            .map(|Point2D(x, y)| *x * *y)
            .sum::<i64>()
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        let mut mem_cloned = input.clone();
        mem_cloned[0] = 2;
        let input_string = r#"A,B,A,C,B,C,A,C,B,C
L,8,R,10,L,10
R,10,L,8,L,8,L,10
L,4,L,6,L,8,L,8
n
"#;
        let mut machine = intcode::Machine::new(
            &mem_cloned,
            input_string
                .as_bytes()
                .iter()
                .cloned()
                .map(i64::from)
                .collect(),
        );

        machine.run_to_completion();
        *machine.output.last().unwrap()
    }
}

fn next_point_on_track(
    map: &Map,
    pos: Point2D<i64>,
    dir: Direction,
) -> Option<(Point2D<i64>, Direction)> {
    for r in &[0, 1, 3] {
        let n_dir = dir.rotate_times(*r);
        let n_pos = n_dir.next_point(pos);
        if map.contains_key(&n_pos) {
            return Some((n_pos, n_dir));
        }
    }
    None
}

// L8R10L10R10L8L8L10L8R10L10L4L6L8L8R10L8L8L10L4L6L8L8L8R10L10L4L6L8L8R10L8L8L10L4L6L8L8
// L8R10L10          L8R10L10                          L8R10L10
//         R10L8L8L10                R10L8L8L10                        R10L8L8L10
//                           L4L6L8L8          L4L6L8L8        L4L6L8L8          L4L6L8L8

// abacbcacbc
// A,B,A,C,B,C,A,C,B,C
// L,8,R,10,L,10
// R,10,L,8,L,8,L,10
// L,4,L,6,L,8,L,8
