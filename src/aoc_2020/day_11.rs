use crate::aoc_2020::Aoc2020;
use crate::files::Res;
use std::{fs::read_to_string, mem::swap};

pub struct Day11;

impl Aoc2020 for Day11 {
    type Input = (Vec<char>, usize /*width*/, usize /*height*/);
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        11
    }
    fn load() -> Res<Self::Input> {
        let s = read_to_string("data/2020/day_11.in")?;
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        let input = s.chars().filter(|c| *c != '\n').collect::<Vec<_>>();
        Ok((input, width, height))
    }

    /// Model the input as cellular automata and run it until it is stable and count the number of seats.
    fn part_1((input, width, height): &Self::Input) -> Self::Result1 {
        // double buffer, to cut down on allocation and freeing
        let mut ca = input.clone();
        let mut cb = input.clone();
        loop {
            run_step_vec(&ca, &mut cb, *width, *height, 4, false);

            if ca == cb {
                return cb.iter().filter(|&c| *c == '#').count();
            } else {
                swap(&mut ca, &mut cb);
            }
        }
    }

    /// As part 1, but using a "sight" based neighbourhood.
    fn part_2((input, width, height): &Self::Input) -> Self::Result2 {
        // double buffer, to cut down on allocation and freeing
        let mut ca = input.clone();
        let mut cb = input.clone();
        loop {
            run_step_vec(&ca, &mut cb, *width, *height, 5, true);

            if ca == cb {
                return cb.iter().filter(|&c| *c == '#').count();
            } else {
                swap(&mut ca, &mut cb);
            }
        }
    }
}

fn run_step_vec(
    ca: &Vec<char>,
    new: &mut Vec<char>,
    width: usize,
    height: usize,
    crowd: usize,
    sight: bool,
) {
    for (i, c) in ca.iter().enumerate() {
        new[i] = match c {
            '#' => {
                let neighbours = get_neighbours_vec(ca, i, width, height, sight);
                if neighbours.iter().filter(|&n| *n == '#').count() >= crowd {
                    'L'
                } else {
                    '#'
                }
            }
            'L' => {
                let neighbours = get_neighbours_vec(ca, i, width, height, sight);
                if neighbours.iter().filter(|&n| *n == '#').count() == 0 {
                    '#'
                } else {
                    'L'
                }
            }
            c => *c,
        }
    }

    // new
}

/// an empty string means off-grid
fn get(ca: &Vec<char>, column: i32, row: i32, width: usize, height: usize) -> char {
    if (0..width as i32).contains(&column) && (0..height as i32).contains(&row) {
        ca[(column as usize + width * row as usize) as usize]
    } else {
        ' '
    }
}

/// get the first non-empty cell value along the vector (checking the point first)
fn get_vec(
    ca: &Vec<char>,
    column: i32,
    row: i32,
    c_vec: i32,
    r_vec: i32,
    width: usize,
    height: usize,
    sight: bool,
) -> char {
    let cn = column + c_vec;
    let rn = row + r_vec;
    let hit = get(ca, cn, rn, width, height);
    if hit == '.' && sight {
        get_vec(ca, cn, rn, c_vec, r_vec, width, height, sight)
    } else {
        hit
    }
}

fn get_neighbours_vec(
    ca: &Vec<char>,
    i: usize,
    width: usize,
    height: usize,
    sight: bool,
) -> [char; 8] {
    let row = (i / width) as i32;
    let col = (i % width) as i32;

    let mut res = [' '; 8];

    for (i, &(r, c)) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .enumerate()
    {
        res[i] = get_vec(ca, col, row, c, r, width, height, sight);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "L.LL.LL.LL\n\
        LLLLLLL.LL\n\
        L.L.L..L..\n\
        LLLL.LL.LL\n\
        L.LL.LL.LL\n\
        L.LLLLL.LL\n\
        ..L.L.....\n\
        LLLLLLLLLL\n\
        L.LLLLLL.L\n\
        L.LLLLL.LL"
            .chars()
            .filter(|c| *c != '\n')
            .collect::<Vec<_>>();
        let res = Day11::part_1(&(input, 10, 10));
        assert_eq!(res, 37);
    }

    #[test]
    fn part_2() {
        let input = "L.LL.LL.LL\n\
        LLLLLLL.LL\n\
        L.L.L..L..\n\
        LLLL.LL.LL\n\
        L.LL.LL.LL\n\
        L.LLLLL.LL\n\
        ..L.L.....\n\
        LLLLLLLLLL\n\
        L.LLLLLL.L\n\
        L.LLLLL.LL"
            .chars()
            .filter(|c| *c != '\n')
            .collect::<Vec<_>>();
        let res = Day11::part_2(&(input, 10, 10));
        assert_eq!(res, 26);
    }
}
