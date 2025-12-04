use aoc_common::files::Res;
use aoc_common::aoc_day::AocDay;
use std::{fs::read_to_string, mem::swap};

pub struct Day04;

impl AocDay for Day04 {
    type Input = (Vec<char>, usize /*width*/, usize /*height*/);
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        11
    }
    fn load() -> Res<Self::Input> {
        let s = read_to_string("data/2025/day_04.in")?;
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        let input = s.chars().filter(|c| *c != '\n').collect::<Vec<_>>();
        Ok((input, width, height))
    }

    /// Model the input as cellular automata and run it once.
    fn part_1((input, width, height): &Self::Input) -> Self::Result1 {
        let mut new = input.clone();

        run_step_vec(&input, &mut new, *width, *height, 4, false);
        return new.iter().filter(|&c| *c == 'x').count();
    }

    /// Model the input as cellular automata and run it until it is stable and count the number of seats.
    fn part_2((input, width, height): &Self::Input) -> Self::Result2 {
        // double buffer, to cut down on allocation and freeing
        let mut ca = input.clone();
        let mut cb = input.clone();
        loop {
            run_step_vec(&ca, &mut cb, *width, *height, 4, false);

            if ca == cb {
                return cb.iter().filter(|&c| *c == 'x').count();
            } else {
                swap(&mut ca, &mut cb);
            }
        }
    }
}

fn run_step_vec(
    old: &[char],
    new: &mut [char],
    width: usize,
    height: usize,
    crowd: usize,
    sight: bool,
) {
    for (i, c) in old.iter().enumerate() {
        new[i] = match c {
            '@' => {
                let occupied_neighbours =
                    get_occupied_neighbours_vec(old, i, (width as i32, height as i32), sight);
                if occupied_neighbours < crowd {
                    'x'
                } else {
                    '@'
                }
            }

            c => *c,
        }
    }

}

/// an empty string means off-grid
fn get(ca: &[char], column: i32, row: i32, width: i32, height: i32) -> char {
    // faster without this!
    // if (0..width as i32).contains(&column) && (0..height as i32).contains(&row) {
    if column >= 0 && row >= 0 && column < width && row < height {
        ca[(column + width * row) as usize]
    } else {
        ' '
    }
}

type Point = (i32, i32);

/// get the first non-empty cell value along the vector (checking the point first)
fn get_vec(
    grid: &[char],
    (column, row): Point,
    (c_vec, r_vec): Point,
    (width, height): Point,
    sight: bool,
) -> char {
    let cn = column + c_vec;
    let rn = row + r_vec;
    let hit = get(grid, cn, rn, width, height);
    if hit == '.' && sight {
        get_vec(grid, (cn, rn), (c_vec, r_vec), (width, height), sight)
    } else {
        hit
    }
}

fn get_occupied_neighbours_vec(
    grid: &[char],
    i: usize,
    (width, height): Point,
    sight: bool,
) -> usize {
    const NEIGHBOURS: [Point; 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let i = i as i32;
    let row = i / width;
    let col = i - (width * row);

    NEIGHBOURS
        .iter()
        .filter(|(r, c)| '@' == get_vec(grid, (col, row), (*c, *r), (width, height), sight))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            .chars()
            .filter(|c| *c != '\n')
            .collect::<Vec<_>>();
        let res = Day04::part_1(&(input, 10, 10));
        assert_eq!(res, 13);
    }

    #[test]
    fn part_2() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            .chars()
            .filter(|c| *c != '\n')
            .collect::<Vec<_>>();
        let res = Day04::part_2(&(input, 10, 10));
        assert_eq!(res, 43);
    }
}
