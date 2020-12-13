use crate::aoc_2020::Aoc2020;
use crate::files::Res;
use std::fs::read_to_string;

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
        let mut ca = input.clone();
        loop {
            let next = run_step(&ca, *width, *height);

            if ca == next {
                return next.iter().filter(|&c| *c == '#').count();
            } else {
                ca = next
            }
        }
    }

    /// As part 1, but using a "sight" based neighbourhood.
    fn part_2((input, width, height): &Self::Input) -> Self::Result2 {
        let mut ca = input.clone();
        loop {
            let next = run_step_vec(&ca, *width, *height);

            if ca == next {
                return next.iter().filter(|&c| *c == '#').count();
            } else {
                ca = next
            }
        }
    }
}

fn run_step(ca: &Vec<char>, width: usize, height: usize) -> Vec<char> {
    let mut new = ca.clone();

    for (i, c) in ca.iter().enumerate() {
        new[i] = match c {
            '#' => {
                let neighbours = get_neighbours(ca, i, width, height);
                if neighbours.iter().filter(|&n| *n == '#').count() >= 4 {
                    'L'
                } else {
                    '#'
                }
            }
            'L' => {
                let neighbours = get_neighbours(ca, i, width, height);
                if neighbours.iter().filter(|&n| *n == '#').count() == 0 {
                    '#'
                } else {
                    'L'
                }
            }
            c => *c,
        }
    }

    new
}

fn run_step_vec(ca: &Vec<char>, width: usize, height: usize) -> Vec<char> {
    let mut new = ca.clone();

    for (i, c) in ca.iter().enumerate() {
        new[i] = match c {
            '#' => {
                let neighbours = get_neighbours_vec(ca, i, width, height);
                if neighbours.iter().filter(|&n| *n == '#').count() >= 5 {
                    'L'
                } else {
                    '#'
                }
            }
            'L' => {
                let neighbours = get_neighbours_vec(ca, i, width, height);
                if neighbours.iter().filter(|&n| *n == '#').count() == 0 {
                    '#'
                } else {
                    'L'
                }
            }
            c => *c,
        }
    }

    new
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
) -> char {
    let hit = get(ca, column, row, width, height);
    if hit == '.' {
        get_vec(ca, column + c_vec, row + r_vec, c_vec, r_vec, width, height)
    } else {
        hit
    }
}

fn get_neighbours(ca: &Vec<char>, i: usize, width: usize, height: usize) -> [char; 8] {
    let row = (i / width) as i32;
    let col = (i % width) as i32;

    [
        get(ca, col - 1, row - 1, width, height),
        get(ca, col - 1, row, width, height),
        get(ca, col - 1, row + 1, width, height),
        get(ca, col, row - 1, width, height),
        get(ca, col, row + 1, width, height),
        get(ca, col + 1, row - 1, width, height),
        get(ca, col + 1, row, width, height),
        get(ca, col + 1, row + 1, width, height),
    ]
}

fn get_neighbours_vec(ca: &Vec<char>, i: usize, width: usize, height: usize) -> [char; 8] {
    let row = (i / width) as i32;
    let col = (i % width) as i32;

    [
        get_vec(ca, col - 1, row - 1, -1, -1, width, height),
        get_vec(ca, col - 1, row, -1, 0, width, height),
        get_vec(ca, col - 1, row + 1, -1, 1, width, height),
        get_vec(ca, col, row - 1, 0, -1, width, height),
        get_vec(ca, col, row + 1, 0, 1, width, height),
        get_vec(ca, col + 1, row - 1, 1, -1, width, height),
        get_vec(ca, col + 1, row, 1, 0, width, height),
        get_vec(ca, col + 1, row + 1, 1, 1, width, height),
    ]
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
