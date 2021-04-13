use aoc_common::aoc_day::AocDay;
use aoc_common::files::Res;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub struct Day17;

type Point4d = (i64, i64, i64, i64);
type Points = HashSet<Point4d>;

impl AocDay for Day17 {
    type Input = Points;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        17
    }
    fn load() -> Res<Self::Input> {
        let input = read_to_string("data/2020/day_17.in")?;
        let mut points = Points::new();
        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if c == '#' {
                    points.insert((x as i64, y as i64, 0, 0));
                }
            }
        }
        Ok(points)
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        let mut ca = input.clone();
        for _ in 0..6 {
            ca = run_generation(&ca, false);
        }

        ca.len()
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        let mut ca = input.clone();
        for _ in 0..6 {
            ca = run_generation(&ca, true);
        }

        ca.len()
    }
}

fn run_generation(points: &Points, is_4d: bool) -> Points {
    let mut adjacents: HashMap<Point4d, usize> = HashMap::new();

    for p in points {
        if points.contains(p) {
            add_adjacents(&mut adjacents, *p, is_4d);
        }
    }

    let mut next = Points::new();

    for (p, v) in adjacents {
        if v == 3 || (v == 2 && points.contains(&p)) {
            next.insert(p);
        }
    }

    next
}

fn add_adjacents(adjacents: &mut HashMap<Point4d, usize>, p: Point4d, is_4d: bool) {
    (-1..=1)
        .flat_map(|x| {
            (-1..=1).flat_map(move |y| {
                (-1..=1).flat_map(move |z| {
                    (if is_4d { -1..=1 } else { 0..=0 })
                        .map(move |w| (p.0 + x, p.1 + y, p.2 + z, p.3 + w))
                })
            })
        })
        .filter(|&n| n != p)
        .for_each(|p| *adjacents.entry(p).or_insert(0) += 1);
}
