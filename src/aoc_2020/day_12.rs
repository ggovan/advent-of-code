use crate::aoc_2020::Aoc2020;
use crate::files::Res;
use std::fs::read_to_string;

pub struct Day12;

impl Aoc2020 for Day12 {
    type Input = Vec<String>;
    type Result1 = i64;
    type Result2 = i64;

    fn day() -> usize {
        12
    }
    fn load() -> Res<Self::Input> {
        Ok(read_to_string("data/2020/day_12.in")?
            .lines()
            .map(|l| l.to_owned())
            .collect())
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        let mut dir = Direction::Right;
        let mut pos = (0, 0);

        for i in input {
            let command = i.chars().next().unwrap();
            let amount = i
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<i64>()
                .unwrap();

            match command {
                'N' => pos = Direction::Up.move_along(pos, amount),
                'S' => pos = Direction::Down.move_along(pos, amount),
                'W' => pos = Direction::Left.move_along(pos, amount),
                'E' => pos = Direction::Right.move_along(pos, amount),
                'F' => pos = dir.move_along(pos, amount),
                'R' => dir = dir.rotate_cw(amount / 90),
                'L' => dir = dir.rotate_acw(amount / 90),
                c => unreachable!("WHAT! {}", c),
            }

            // println!("{} {}, {:?} {:?}", command, amount, dir, pos);
        }

        pos.0.abs() + pos.1.abs()
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        let mut pos = (0, 0);
        let mut way = (10, -1);

        for i in input {
            let command = i.chars().next().unwrap();
            let amount = i
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<i64>()
                .unwrap();

            match command {
                'N' => way = Direction::Up.move_along(way, amount),
                'S' => way = Direction::Down.move_along(way, amount),
                'W' => way = Direction::Left.move_along(way, amount),
                'E' => way = Direction::Right.move_along(way, amount),
                'F' => pos = move_to_waypoint(pos, &way, amount),
                'R' => way = rotate_cw(way, amount / 90),
                'L' => way = rotate_acw(way, amount / 90),
                c => unreachable!("WHAT! {}", c),
            }
        }

        pos.0.abs() + pos.1.abs()
    }
}

fn move_to_waypoint((x, y): Point, &(wx, wy): &Point, amount: i64) -> Point {
    (x + wx * amount, y + wy * amount)
}

fn rotate_cw((x, y): Point, deg: i64) -> Point {
    match deg {
        1 => (-y, x),
        2 => (-x, -y),
        3 => (y, -x),
        _ => unreachable!("Impossible: {}", deg),
    }
}

fn rotate_acw((x, y): Point, deg: i64) -> Point {
    match deg {
        1 => (y, -x),
        2 => (-x, -y),
        3 => (-y, x),
        _ => unreachable!("Impossible: {}", deg),
    }
}

type Point = (i64, i64);

// TODO this is the third or so time I've written this!
#[derive(Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}
impl Direction {
    fn from(v: i64) -> Self {
        match v {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!("Unknown direction {}", v),
        }
    }
    fn to_int(&self) -> i64 {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
    fn rotate_cw(&self, dir: i64) -> Self {
        Self::from((self.to_int() + dir) % 4)
    }
    fn rotate_acw(&self, dir: i64) -> Self {
        Self::from((self.to_int() + 4 - dir) % 4)
    }
    fn move_along(&self, (x, y): (i64, i64), amount: i64) -> (i64, i64) {
        match self {
            Direction::Up => (x, y - amount),
            Direction::Right => (x + amount, y),
            Direction::Down => (x, y + amount),
            Direction::Left => (x - amount, y),
        }
    }
}
