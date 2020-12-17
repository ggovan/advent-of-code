use crate::aoc_2020::Aoc2020;
use crate::files::Res;
use std::collections::HashSet;
use std::fs::read_to_string;

pub struct Day17;

type Point4d = (i64, i64, i64, i64);
type Points = HashSet<Point4d>;

impl Aoc2020 for Day17 {
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
            // print_points(&ca);
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
    let (min, max) = get_bounds(points);
    let mut next = Points::new();

    for x in (min.0 - 1)..=(max.0 + 1) {
        for y in (min.1 - 1)..=(max.1 + 1) {
            for z in (min.2 - 1)..=(max.2 + 1) {
                for w in (min.3 - 1)..=(max.3 + 1) {
                    let active_count = get_populated_neighbours(points, (x, y, z, w), is_4d);
                    match points.contains(&(x, y, z, w)) {
                        true if active_count == 2 || active_count == 3 => {
                            next.insert((x, y, z, w));
                        }
                        false if active_count == 3 => {
                            next.insert((x, y, z, w));
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    next
}

fn get_bounds(points: &Points) -> (Point4d, Point4d) {
    // TODO - fold over this just once!
    (
        (
            points.iter().min_by_key(|p| p.0).unwrap().0,
            points.iter().min_by_key(|p| p.1).unwrap().1,
            points.iter().min_by_key(|p| p.2).unwrap().2,
            points.iter().min_by_key(|p| p.3).unwrap().3,
        ),
        (
            points.iter().max_by_key(|p| p.0).unwrap().0,
            points.iter().max_by_key(|p| p.1).unwrap().1,
            points.iter().max_by_key(|p| p.2).unwrap().2,
            points.iter().max_by_key(|p| p.3).unwrap().3,
        ),
    )
}

fn get_populated_neighbours(points: &Points, p: Point4d, is_4d: bool) -> usize {
    (-1..=1)
        .flat_map(|x| {
            (-1..=1).flat_map(move |y| {
                (-1..=1).flat_map(move |z| {
                    (if is_4d { -1..=1 } else { 0..=0 })
                        .map(move |w| (p.0 + x, p.1 + y, p.2 + z, p.3 + w))
                })
            })
        })
        .filter(|&n| n != p && points.contains(&n))
        .count()
}

fn print_points(points: &Points) {
    let (min, max) = get_bounds(points);

    for y in (min.1)..=max.1 {
        for x in (min.0)..=max.0 {
            match points.contains(&(x, y, 0, 0)) {
                true => print!("#"),
                false => print!("."),
            }
        }
        print!("\n");
    }
}
