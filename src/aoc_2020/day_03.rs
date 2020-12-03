use crate::files::Res;
use std::fs::read_to_string;

pub fn day_03() -> Res<()> {
    println!("Day 3");

    let input = load()?;

    println!("  part 1: {}", part_1(&input));
    println!("  part 2: {}", part_2(&input));

    Ok(())
}

pub fn load() -> Res<Vec<Vec<char>>> {
    let input: String = read_to_string("data/2020/day_03.in")?;
    Ok(input.lines().map(|l| l.chars().collect()).collect())
}

pub fn part_1(input: &Vec<Vec<char>>) -> i64 {
    run_slope(input, 3, 1)
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

pub fn part_2(input: &Vec<Vec<char>>) -> i64 {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(move |(x, y)| run_slope(input, *x as usize, *y as usize))
        .product()
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
        let res = part_1(&input);
        assert_eq!(res, 7);
    }
}
