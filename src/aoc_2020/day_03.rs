use super::Aoc2020;
use crate::files::Res;
use std::fs::read_to_string;

pub struct Day03;

impl Aoc2020 for Day03 {
    type Input = Vec<Vec<char>>;
    type Result1 = i64;
    type Result2 = i64;

    fn day() -> usize {
        3
    }

    fn load() -> Res<Vec<Vec<char>>> {
        let input: String = read_to_string("data/2020/day_03.in")?;
        Ok(input.lines().map(|l| l.chars().collect()).collect())
    }

    fn part_1(input: &Vec<Vec<char>>) -> i64 {
        run_slope(input, 3, 1)
    }

    fn part_2(input: &Vec<Vec<char>>) -> i64 {
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(move |(x, y)| run_slope(input, *x as usize, *y as usize))
            .product()
    }
}

fn run_slope(input: &Vec<Vec<char>>, dx: usize, dy: usize) -> i64 {
    let width = input[0].len();
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    while y < input.len() {
        if input[y][x % width] == '#' {
            trees += 1;
        }
        x += dx;
        y += dy;
    }
    trees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "..##.......\n\
                     #...#...#..\n\
                     .#....#..#.\n\
                     ..#.#...#.#\n\
                     .#...##..#.\n\
                     ..#.##.....\n\
                     .#.#.#....#\n\
                     .#........#\n\
                     #.##...#...\n\
                     #...##....#\n\
                     .#..#...#.#";
        let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let res = run_slope(&input, 3, 1);
        assert_eq!(res, 7);
    }
}
