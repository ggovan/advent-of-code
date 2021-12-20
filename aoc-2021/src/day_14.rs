use aoc_common::aoc_day::AocDay;
use aoc_common::files::Res;
use std::collections::HashMap;
use std::fs::read_to_string;

pub struct Day14;

impl AocDay for Day14 {
    type Input = (Vec<char>, HashMap<(char, char), char>);
    type Result1 = i64;
    type Result2 = i64;

    fn day() -> usize {
        14
    }
    fn load() -> Res<Self::Input> {
        let input_str = read_to_string("data/2021/day_14.in")?;

        Ok((
            input_str.lines().next().unwrap().chars().collect(),
            input_str
                .lines()
                .skip(2)
                .map(|l| {
                    let (left, right) = l.split_once(" -> ").unwrap();
                    (
                        (left.chars().next().unwrap(), left.chars().nth(1).unwrap()),
                        right.chars().next().unwrap(),
                    )
                })
                .collect(),
        ))
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        do_it(input, 10)
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        do_it(input, 40)
    }
}

fn do_it((input, rules): &<Day14 as AocDay>::Input, iterations: usize) -> i64 {
    let paired: Vec<(char, char)> = input.windows(2).map(|arr| (arr[0], arr[1])).collect();

    let mut pairs: HashMap<(char, char), i64> = HashMap::new();
    for c in paired.iter() {
        *pairs.entry(*c).or_insert(0) += 1;
    }

    let mut next_pairs: HashMap<(char, char), i64> = HashMap::new();
    for _ in 0..iterations {
        next_pairs.clear();
        for (p, count) in pairs.drain() {
            let nc = rules[&p];
            let np1 = (p.0, nc);
            let np2 = (nc, p.1);
            *next_pairs.entry(np1).or_insert(0) += count;
            *next_pairs.entry(np2).or_insert(0) += count;
        }
        // swap to avoid alloc
        let t = pairs;
        pairs = next_pairs;
        next_pairs = t;
    }

    let mut freq_map: HashMap<char, i64> = HashMap::new();
    for ((l, _), c) in pairs.iter() {
        *freq_map.entry(*l).or_insert(0) += c;
    }
    *freq_map.entry(*input.last().unwrap()).or_insert(0) += 1;

    freq_map.values().max().unwrap() - freq_map.values().min().unwrap()
}
