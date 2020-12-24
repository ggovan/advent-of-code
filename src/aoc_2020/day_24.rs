use crate::aoc_2020::Aoc2020;
use crate::files::Res;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub struct Day24;

impl Aoc2020 for Day24 {
    type Input = Vec<Vec<char>>;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        24
    }
    fn load() -> Res<Self::Input> {
        let input = read_to_string("data/2020/day_24.in")?;
        Ok(input.lines().map(|l| l.chars().collect()).collect())
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        get_starting_tiles(input).len()
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        const NEIGHBOURS: [Point; 6] = [(0, 1), (-1, 1), (1, -1), (0, -1), (1, 0), (-1, 0)];
        let mut populated_tiles: HashSet<Point> = get_starting_tiles(input).into_iter().collect();
        let mut neighbours: HashMap<Point, usize> = HashMap::new();

        for _generation in 0..100 {
            for (n, ne) in populated_tiles.iter() {
                for neighbour in NEIGHBOURS.iter().map(|(n2, ne2)| (n + n2, ne + ne2)) {
                    *neighbours.entry(neighbour).or_insert(0) += 1;
                }
            }

            populated_tiles = neighbours
                .drain()
                .filter(|(k, v)| match (populated_tiles.contains(k), v) {
                    (true, &x) if x == 1 || x == 2 => true,
                    (false, 2) => true,
                    _ => false,
                })
                .map(|(k, _)| k)
                .collect();
        }

        populated_tiles.len()
    }
}

type Point = (i32, i32);

fn get_starting_tiles(input: &[Vec<char>]) -> Vec<Point> {
    let mut reached_tiles = input
        .iter()
        .map(|path| {
            path.iter()
                .batching(|it| match it.next() {
                    Some('w') => Some(Dir::W),
                    Some('e') => Some(Dir::E),
                    Some('n') => match it.next() {
                        Some('w') => Some(Dir::NW),
                        Some('e') => Some(Dir::NE),
                        x => unreachable!("{:?}", x),
                    },
                    Some('s') => match it.next() {
                        Some('w') => Some(Dir::SW),
                        Some('e') => Some(Dir::SE),
                        x => unreachable!("{:?}", x),
                    },
                    None => None,
                    x => unreachable!("{:?}", x),
                })
                .fold((0, 0), |(e, ne), d| match d {
                    Dir::NE => (e, ne + 1),
                    Dir::NW => (e - 1, ne + 1),
                    Dir::SE => (e + 1, ne - 1),
                    Dir::SW => (e, ne - 1),
                    Dir::E => (e + 1, ne),
                    Dir::W => (e - 1, ne),
                })
        })
        .collect::<Vec<_>>();

    reached_tiles.sort_by(|(e1, ne1), (e2, ne2)| match e1.cmp(e2) {
        Ordering::Equal => ne1.cmp(ne2),
        x => x,
    });

    reached_tiles
        .into_iter()
        .map(Some)
        .coalesce(|a, b| match (a, b) {
            (Some(a), Some(b)) if a == b => Ok(None),
            (Some(a), Some(b)) => Err((Some(a), Some(b))),
            (None, Some(a)) => Ok(Some(a)),
            _ => unreachable!(),
        })
        .filter_map(|p| p)
        .collect()
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    NE,
    NW,
    W,
    E,
    SW,
    SE,
}
