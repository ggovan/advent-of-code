use crate::aoc_2020::Aoc2020;
use crate::files::Res;
use std::collections::HashMap;
use std::fs::read_to_string;

pub struct Day20;

impl Aoc2020 for Day20 {
    type Input = Vec<Tile>;
    type Result1 = u64;
    type Result2 = u64;

    fn day() -> usize {
        20
    }
    fn load() -> Res<Self::Input> {
        let input = read_to_string("data/2020/day_20.in")?;
        Ok(input.split("\n\n").map(parse_tile).collect())
    }

    fn part_1(tiles: &Self::Input) -> Self::Result1 {
        let mut map: HashMap<u16, Vec<u64>> = HashMap::new();

        for t in tiles {
            // get the four edges
            // get their binary repr
            let entry = map.entry(t.top_binary()).or_insert(Vec::new());
            entry.push(t.id);
            let entry = map.entry(t.bottom_binary()).or_insert(Vec::new());
            entry.push(t.id);
            let entry = map.entry(t.left_binary()).or_insert(Vec::new());
            entry.push(t.id);
            let entry = map.entry(t.right_binary()).or_insert(Vec::new());
            entry.push(t.id);
        }

        tiles
            .iter()
            .filter(|t| {
                // A corner will have exactly 2 unmatched edges
                map.iter()
                    .filter(|(_, v)| v.len() == 1 && v.contains(&t.id))
                    .count()
                    > 1
            })
            .map(|t| t.id)
            .product()
    }

    fn part_2(_input: &Self::Input) -> Self::Result2 {
        20
    }
}

#[derive(Debug)]
pub struct Tile {
    id: u64,
    content: Vec<char>,
}

fn flip_bits(b: u16) -> u16 {
    let mut f_b = 0;
    for i in 0..10 {
        let bit = b & (1 << i);
        let bit = bit >> i;
        f_b |= bit << (9 - i)
    }
    f_b
}

impl Tile {
    fn top_binary(&self) -> u16 {
        let b = self
            .content
            .iter()
            .take(10)
            .fold(0_u16, |acc, x| (acc << 1) + if *x == '#' { 1 } else { 0 });
        let f_b = flip_bits(b);
        std::cmp::max(b, f_b)
    }

    fn bottom_binary(&self) -> u16 {
        let b = self
            .content
            .iter()
            .skip(90)
            .take(10)
            .fold(0_u16, |acc, x| (acc << 1) + if *x == '#' { 1 } else { 0 });
        let f_b = flip_bits(b);
        std::cmp::max(b, f_b)
    }

    fn left_binary(&self) -> u16 {
        let b = self
            .content
            .iter()
            .step_by(10)
            .take(10)
            .fold(0_u16, |acc, x| (acc << 1) + if *x == '#' { 1 } else { 0 });
        let f_b = flip_bits(b);
        std::cmp::max(b, f_b)
    }

    fn right_binary(&self) -> u16 {
        let b = self
            .content
            .iter()
            .skip(9)
            .step_by(10)
            .take(10)
            .fold(0_u16, |acc, x| (acc << 1) + if *x == '#' { 1 } else { 0 });
        let f_b = flip_bits(b);
        std::cmp::max(b, f_b)
    }
}

fn parse_tile(s: &str) -> Tile {
    let id = s
        .lines()
        .next()
        .unwrap()
        .chars()
        .skip(5)
        .take(4)
        .collect::<String>()
        .parse()
        .unwrap();
    let content = s
        .lines()
        .skip(1)
        .flat_map(|l| l.chars())
        .filter(|c| *c != '\n')
        .collect();

    Tile { id, content }
}
