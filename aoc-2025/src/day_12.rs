use aoc_common::aoc_day::AocDay;
use aoc_common::files::Res;
use std::fs::read_to_string;
use tinyvec::ArrayVec;

pub struct Day12;

type TV = tinyvec::ArrayVec<[bool; 9]>;
type MapCounts = tinyvec::ArrayVec<[usize; 6]>;

type Map = TV;

impl AocDay for Day12 {
    type Input = (Vec<Map>, Vec<((i64, i64), MapCounts)>);
    type Result1 = i64;
    type Result2 = i64;

    fn day() -> usize {
        12
    }

    fn load() -> Res<Self::Input> {
        Ok(parse(&read_to_string("data/2025/day_12.in")?))
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        // Turns out the problem doesn't need to be solved fully.
        // Just filter out all the arrangements that are impossible based on total occupied space alone.
        let (maps, regions) = input;
        let population: ArrayVec<[i64; 6]> = maps
            .iter()
            .map(|m| m.iter().filter(|&v| *v).count() as i64)
            .collect();
        regions
            .iter()
            .filter(|((width, height), map_indices)| {
                let mut area = width * height;
                for (idx, &count) in map_indices.iter().enumerate() {
                    area -= count as i64 * population[idx];
                }
                area > 0
            })
            .count() as i64
    }

    fn part_2(_input: &Self::Input) -> Self::Result2 {
        12
    }
}

fn parse(input: &str) -> <Day12 as AocDay>::Input {
    let sections: Vec<&str> = input.split("\n\n").collect();

    let maps: Vec<Map> = sections
        .iter()
        .filter_map(|s| {
            if s.lines().next()?.contains('x') {
                None
            } else {
                Some(s)
            }
        })
        .map(|map_str| {
            let mut map = TV::default();
            for line in map_str.lines() {
                for ch in line.chars() {
                    match ch {
                        '#' => map.push(true),
                        '.' => map.push(false),
                        _ => {}
                    }
                }
            }
            map
        })
        .collect();

    let mut arrangements: Vec<((i64, i64), MapCounts)> = Vec::new();

    for line in sections.last().unwrap().lines() {
        let (dimensions, map_counts) = line.split_once(":").unwrap();
        let (width, height) = dimensions.split_once("x").unwrap();
        let width: i64 = width.trim().parse().unwrap();
        let height: i64 = height.trim().parse().unwrap();

        let map_counts: MapCounts = map_counts
            .split_whitespace()
            .map(|idx_str| idx_str.parse().unwrap())
            .collect();

        arrangements.push(((width, height), map_counts));
    }

    (maps, arrangements)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tinyvec::array_vec;

    const INPUT: &str = r"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]

    fn part_1() {
        let input = parse(INPUT);
        assert_eq!(
            input,
            (
                vec![
                    array_vec!([bool; 9] => true, true, true, true, true, false, true, true, false),
                    array_vec!([bool; 9] => true, true, true, true, true, false, false, true, true),
                    array_vec!([bool; 9] => false, true, true, true, true, true, true, true, false),
                    array_vec!([bool; 9] => true, true, false, true, true, true, true, true, false),
                    array_vec!([bool; 9] => true, true, true, true, false, false, true, true, true),
                    array_vec!([bool; 9] => true, true, true, false, true, false, true, true, true),
                ],
                vec![
                    ((4, 4), array_vec!([usize; 6] => 0, 0, 0, 0, 2, 0)),
                    ((12, 5), array_vec!([usize; 6] => 1, 0, 1, 0, 2, 2)),
                    ((12, 5), array_vec!([usize; 6] => 1, 0, 1, 0, 3, 2)),
                ]
            )
        );
    }
}
