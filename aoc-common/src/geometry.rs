use std::collections::HashMap;
use std::fmt::{Debug, Display};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Point2D<T>(pub T, pub T);

impl<T> Point2D<T> where T: Copy + Clone + Debug + Display {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn array() -> &'static [Self; 4] {
        use Direction::*;
        const ARRAY: [Direction; 4] = [North, East, South, West];
        &ARRAY
    }

    pub fn rotate_cw(&self) -> Self {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    pub fn rotate_acw(&self) -> Self {
        use Direction::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    fn from(v: i64) -> Self {
        match v {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => panic!("Unknown direction {}", v),
        }
    }
    fn to_int(&self) -> i64 {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }

    /// Don't use this
    pub fn rotate_cw_amount(&self, dir: i64) -> Self {
        Self::from((self.to_int() + if dir == 0 { 3 } else { 1 }) % 4)
    }

    pub fn rotate_times(&self, times: i64) -> Self {
        Self::from((self.to_int() + times) % 4)
    }

    pub fn rotate(&self, acw: bool) -> Self {
        if acw {
            self.rotate_acw()
        } else {
            self.rotate_cw()
        }
    }

    pub fn next_point<P, T>(&self, p: P) -> P
    where
        P: PointLike<T>,
        T: num::Integer + Copy,
    {
        use Direction::*;

        let (x, y) = p.get_xy();
        let (x, y) = match self {
            North => (x, y - T::one()),
            East => (x + T::one(), y),
            South => (x, y + T::one()),
            West => (x - T::one(), y),
        };
        P::from_xy(x, y)
    }
}

/// Transforms an element into a char to be displayed in a graphical map.
pub trait MapFmt {
    fn out(&self) -> char;
}

impl MapFmt for char {
    fn out(&self) -> char {
        *self
    }
}

impl MapFmt for bool {
    fn out(&self) -> char {
        match self {
            true => '#',
            false => ' ',
        }
    }
}

impl MapFmt for i64 {
    fn out(&self) -> char {
        match *self {
            0 => ' ',
            1 => '#',
            2 => '*',
            3 => '_',
            4 => '.',
            5 => '5',
            _ => ' ',
        }
    }
}

pub fn map_bounds<K: PointLike<T>, V: MapFmt, T>(map: &HashMap<K, V>) -> (T, T, T, T)
where
    T: Ord + Copy,
{
    (
        map.keys()
            .map(PointLike::get_xy)
            .min_by_key(|x| x.0)
            .unwrap()
            .0,
        map.keys()
            .map(PointLike::get_xy)
            .max_by_key(|x| x.0)
            .unwrap()
            .0,
        map.keys()
            .map(PointLike::get_xy)
            .min_by_key(|x| x.1)
            .unwrap()
            .1,
        map.keys()
            .map(PointLike::get_xy)
            .max_by_key(|x| x.1)
            .unwrap()
            .1,
    )
}

pub trait PointLike<T> {
    fn get_xy(&self) -> (T, T);
    fn from_xy(x: T, y: T) -> Self;
}

impl<T> PointLike<T> for Point2D<T>
where
    T: num::Integer + Copy,
{
    fn get_xy(&self) -> (T, T) {
        let Self(x, y) = self;
        (*x, *y)
    }

    fn from_xy(x: T, y: T) -> Self {
        Self(x, y)
    }
}

impl<T> PointLike<T> for (T, T)
where
    T: Copy,
{
    fn get_xy(&self) -> (T, T) {
        let (x, y) = self;
        (*x, *y)
    }

    fn from_xy(x: T, y: T) -> Self {
        (x, y)
    }
}

pub fn output_map<K, V: MapFmt, T>(map: &HashMap<K, V>)
where
    T: num::Integer + Copy + std::iter::Step,
    K: PointLike<T> + Eq + core::hash::Hash,
{
    let (x_min, x_max, y_min, y_max) = map_bounds(map);

    for r in y_min..=y_max {
        println!(
            "{}",
            (x_min..=x_max)
                .map(|c| map.get(&K::from_xy(c, r)).map_or(' ', V::out))
                .collect::<String>()
        );
    }
}
