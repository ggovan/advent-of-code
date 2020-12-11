pub mod day_02;
pub mod day_1;
pub mod day_10;
mod day_12;
pub mod day_14;
pub mod day_15;
mod day_3;
mod intcode;
use crate::aoc_2020::Aoc2020;
use crate::files::{read_lines, Res};
use std::collections::HashMap;

pub fn main(day: Option<usize>) -> Res<()> {
    println!("Advent of code!!!");

    day_1::Day01::run_me_maybe(day)?;
    day_02::Day02::run_me_maybe(day)?;

    if day.is_none() {
        day_3::day_3()?;
        day_4();
        intcode::day_5()?;
        day_6()?;
        intcode::day_7()?;
        day_8()?;
        intcode::day_9()?;
        day_10::day_10()?;
        intcode::day_11()?;
        day_12::day_12()?;
        intcode::day_13()?;
    }

    day_14::Day14::run_me_maybe(day)?;
    day_15::Day15::run_me_maybe(day)?;

    Ok(())
}

fn day_4() {
    println!("Day 4");
    let from = 171_309;
    let until = 643_063;
    {
        let mut res_count = 0;

        for test in from..=until {
            if has_adjacent_digits(test) && all_digits_incrementing(test) {
                res_count += 1;
            }
        }

        println!("  part 1 {}", res_count);
    }
    {
        let mut res_count = 0;

        for test in from..=until {
            if has_only_adjacent_digits(test) && all_digits_incrementing(test) {
                res_count += 1;
            }
        }

        println!("  part 2 {}", res_count);
    }
}

fn has_adjacent_digits(test: usize) -> bool {
    let d1 = test / 100_000;
    let d2 = test / 10_000 % 10;
    let d3 = test / 1_000 % 10;
    let d4 = test / 100 % 10;
    let d5 = test / 10 % 10;
    let d6 = test % 10;

    (d1 == d2) || (d2 == d3) || (d3 == d4) || (d4 == d5) || (d5 == d6)
}

fn has_only_adjacent_digits(test: usize) -> bool {
    let d1 = test / 100_000;
    let d2 = test / 10_000 % 10;
    let d3 = test / 1_000 % 10;
    let d4 = test / 100 % 10;
    let d5 = test / 10 % 10;
    let d6 = test % 10;

    (d1 == d2 && d2 != d3)
        || (d1 != d2 && d2 == d3 && d3 != d4)
        || (d2 != d3 && d3 == d4 && d4 != d5)
        || (d3 != d4 && d4 == d5 && d5 != d6)
        || (d4 != d5 && d5 == d6)
}

fn all_digits_incrementing(test: usize) -> bool {
    let d1 = test / 100_000;
    let d2 = test / 10000 % 10;
    let d3 = test / 1000 % 10;
    let d4 = test / 100 % 10;
    let d5 = test / 10 % 10;
    let d6 = test % 10;

    d1 <= d2 && d2 <= d3 && d3 <= d4 && d4 <= d5 && d5 <= d6
}

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn day_6() -> Res<()> {
    let input = read_lines("data/2019/day_6.in")?
        .map(|x| {
            let unwrapped = x.unwrap();
            let parts = unwrapped
                .split(')')
                .map(|x| x.to_string())
                .collect::<Vec<_>>();
            (parts[0].clone(), parts[1].clone())
        })
        .collect::<Vec<_>>();

    let mut node_map: Graph = HashMap::new();

    for (a, b) in &input {
        let new_val: Vec<&str> =
            node_map
                .remove(&a.as_ref())
                .map_or(vec![&b.as_ref()], move |mut v| {
                    v.push(b);
                    v
                });
        node_map.insert(a, new_val);
    }

    println!("Day 6");
    println!("  part 1 {}", count_orbits(&node_map, &"COM".to_owned(), 0));
    println!("  part 2 {:?}", shift_orbits(&node_map, &"COM".to_owned()));
    Ok(())
}

fn count_orbits(graph: &Graph, node: &str, depth: usize) -> usize {
    let empty_vec = vec![];
    graph
        .get(node)
        .unwrap_or(&empty_vec)
        .iter()
        .map(move |n| count_orbits(graph, n, depth + 1))
        .sum::<usize>()
        + depth
}

fn shift_orbits(graph: &Graph, node: &str) -> OrbitDiff {
    use OrbitDiff::*;
    let empty_vec = vec![];
    match node {
        "SAN" => San(0),
        "YOU" => You(0),
        _ => {
            let (both, san, you): (OrbitDiff, OrbitDiff, OrbitDiff) = graph
                .get(node)
                .unwrap_or(&empty_vec)
                .iter()
                .map(move |n| shift_orbits(graph, n))
                .filter(|&d| d != Neither)
                .fold((Neither, Neither, Neither), |(b, s, y), c| match c {
                    Both(_) => (c, Neither, Neither),
                    San(_) => (Neither, c, y),
                    You(_) => (Neither, s, c),
                    _ => (b, s, y),
                });

            match (both, san, you) {
                (Both(x), _, _) => Both(x),
                (_, San(x), You(y)) => Both(x + y),
                (_, San(x), _) => San(x + 1),
                (_, _, You(x)) => You(x + 1),
                _ => OrbitDiff::Neither,
            }
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum OrbitDiff {
    San(usize),
    You(usize),
    Both(usize),
    Neither,
}

fn day_8() -> Res<()> {
    let width = 25;
    let height = 6;
    let layer_size = width * height;

    let input = read_lines("data/2019/day_8.in")?
        .next()
        .unwrap()?
        .chars()
        .collect::<Vec<_>>();
    let layer_count = input.len() / width / height;

    let (_zeros, ones, twos) = (0..layer_count)
        .map(|layer| {
            input[layer * layer_size..(layer + 1) * layer_size]
                .iter()
                .fold((0, 0, 0), |(z, o, t), c| match c {
                    '0' => (z + 1, o, t),
                    '1' => (z, o + 1, t),
                    '2' => (z, o, t + 1),
                    _ => (z, o, t),
                })
        })
        .min_by_key(|acc| acc.0)
        .unwrap();

    println!("Day 8");
    println!("  part 1 {}", ones * twos);

    let mut output = (0..height)
        .map(|_| (0..width).map(|_| '2').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (i, &c) in input.iter().enumerate() {
        let row = (i % layer_size) / width;
        let col = i % width;
        if output[row][col] == '2' {
            output[row][col] = c;
        }
    }

    let out_string = output
        .iter()
        .map(|row| {
            row.iter()
                .map(|&v| if v == '1' { 'x' } else { ' ' })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");
    println!("  part 2:");
    println!("{}", out_string);

    Ok(())
}
