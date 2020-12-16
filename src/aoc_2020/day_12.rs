use crate::aoc_2020::Aoc2020;
use crate::files::Res;
use std::fs::read_to_string;

pub struct Day12;

impl Aoc2020 for Day12 {
    type Input = Vec<Action>;
    type Result1 = i32;
    type Result2 = i32;

    fn day() -> usize {
        12
    }
    fn load() -> Res<Self::Input> {
        Ok(read_to_string("data/2020/day_12.in")?
            .lines()
            .map(Action::from)
            .collect())
    }

    /// Move the ship around, nothing fancy here.
    fn part_1(input: &Self::Input) -> Self::Result1 {
        let mut dir = Direction::Right;
        let mut pos = (0, 0);

        for &command in input {
            match command {
                N(amount) => pos = Direction::Up.move_along(pos, amount),
                S(amount) => pos = Direction::Down.move_along(pos, amount),
                W(amount) => pos = Direction::Left.move_along(pos, amount),
                E(amount) => pos = Direction::Right.move_along(pos, amount),
                F(amount) => pos = dir.move_along(pos, amount),
                R(amount) => dir = dir.rotate_cw(amount),
                L(amount) => dir = dir.rotate_acw(amount),
            }
        }

        pos.0.abs() + pos.1.abs()
    }

    /// Move the ship around, with some fun rotation of the waypoint.
    fn part_2(input: &Self::Input) -> Self::Result2 {
        let mut pos = (0, 0);
        let mut way = (10, -1);

        for &command in input {
            match command {
                N(amount) => way = Direction::Up.move_along(way, amount),
                S(amount) => way = Direction::Down.move_along(way, amount),
                W(amount) => way = Direction::Left.move_along(way, amount),
                E(amount) => way = Direction::Right.move_along(way, amount),
                F(amount) => pos = move_to_waypoint(pos, &way, amount),
                R(amount) => way = rotate_cw(way, amount),
                L(amount) => way = rotate_acw(way, amount),
            }
        }

        pos.0.abs() + pos.1.abs()
    }
}

fn move_to_waypoint((x, y): Point, &(wx, wy): &Point, amount: i32) -> Point {
    (x + wx * amount, y + wy * amount)
}

fn rotate_cw((x, y): Point, deg: i32) -> Point {
    match deg {
        1 => (-y, x),
        2 => (-x, -y),
        3 => (y, -x),
        _ => unreachable!("Impossible: {}", deg),
    }
}

fn rotate_acw((x, y): Point, deg: i32) -> Point {
    match deg {
        1 => (y, -x),
        2 => (-x, -y),
        3 => (-y, x),
        _ => unreachable!("Impossible: {}", deg),
    }
}

type Point = (i32, i32);

use Action::*;
#[derive(Copy, Clone)]
pub enum Action {
    N(i32),
    E(i32),
    S(i32),
    W(i32),
    R(i32),
    L(i32),
    F(i32),
}

impl Action {
    fn from(s: &str) -> Self {
        let command = s.chars().next().unwrap();
        let amount = s
            .chars()
            .skip(1)
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        match command {
            'N' => N(amount),
            'S' => S(amount),
            'W' => W(amount),
            'E' => E(amount),
            'F' => F(amount),
            'R' => R(amount / 90),
            'L' => L(amount / 90),
            c => unreachable!("WHAT! {}", c),
        }
    }
}

// TODO this is the third or so time I've written this!
#[derive(Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}
impl Direction {
    fn from(v: i32) -> Self {
        match v {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!("Unknown direction {}", v),
        }
    }
    fn to_int(&self) -> i32 {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
    fn rotate_cw(&self, dir: i32) -> Self {
        Self::from((self.to_int() + dir) % 4)
    }
    fn rotate_acw(&self, dir: i32) -> Self {
        Self::from((self.to_int() + 4 - dir) % 4)
    }
    fn move_along(&self, (x, y): (i32, i32), amount: i32) -> (i32, i32) {
        match self {
            Direction::Up => (x, y - amount),
            Direction::Right => (x + amount, y),
            Direction::Down => (x, y + amount),
            Direction::Left => (x - amount, y),
        }
    }
}
