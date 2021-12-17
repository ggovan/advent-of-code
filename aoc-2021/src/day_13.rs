use aoc_common::aoc_day::AocDay;
use aoc_common::files::Res;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::fmt::Error;
use std::fs::read_to_string;
use std::iter::FromIterator;
use std::str::FromStr;

pub struct Day13;

#[derive(std::fmt::Debug, Copy, Clone, Hash, Eq, PartialEq)]

pub enum Dir {
    X,
    Y,
}

impl FromStr for Dir {
    type Err = Error;

    fn from_str(s: &str) -> Result<Dir, std::fmt::Error> {
        match s {
            "x" => Ok(Dir::X),
            "y" => Ok(Dir::Y),
            _ => unreachable!("Not valid {}", s),
        }
    }
}

#[derive(std::fmt::Debug, Copy, Clone, Hash, Eq, PartialEq)]

enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<A, B> Either<A, B> {
    fn as_left(self) -> Option<A> {
        if let Either::Left(a) = self {
            Some(a)
        } else {
            None
        }
    }

    fn as_right(self) -> Option<B> {
        if let Either::Right(b) = self {
            Some(b)
        } else {
            None
        }
    }
}

impl AocDay for Day13 {
    type Input = (Vec<(i32, i32)>, Vec<(Dir, i32)>);
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        13
    }
    fn load() -> Res<Self::Input> {
        let input = read_to_string("data/2021/day_13.in")?
            .lines()
            .filter(|l| !l.is_empty())
            .flat_map(parse)
            .collect::<Vec<_>>();
        // dbg!(&input);
        Ok((
            input.iter().filter_map(|either| either.as_left()).collect(),
            input
                .iter()
                .filter_map(|either| either.as_right())
                .collect(),
        ))
    }

    fn part_1((page, folds): &Self::Input) -> Self::Result1 {
        let mut currentPage = page.clone();

        for (dir, pos) in folds.iter().take(1) {
            let nextPage = currentPage
                .iter()
                .map(|(x, y)| match dir {
                    Dir::X if x >= pos => (pos - (x - pos), *y),
                    Dir::Y if y >= pos => (*x, pos - (y - pos)),
                    _ => (*x, *y),
                })
                .collect();
            currentPage = nextPage;
        }

        let set: HashSet<_, RandomState> = HashSet::from_iter(currentPage.into_iter());
        set.len()
    }

    fn part_2((page, folds): &Self::Input) -> Self::Result2 {
        let mut currentPage = page.clone();

        for (dir, pos) in folds.iter() {
            let nextPage = currentPage
                .iter()
                .map(|(x, y)| match dir {
                    Dir::X if x >= pos => (pos - (x - pos), *y),
                    Dir::Y if y >= pos => (*x, pos - (y - pos)),
                    _ => (*x, *y),
                })
                .collect();
            currentPage = nextPage;
        }

        let set: HashSet<_, RandomState> = HashSet::from_iter(currentPage.into_iter());

        println!("Day 13:");
        for y in 0..=(*set.iter().map(|(_, y)| y).max().unwrap()) {
            for x in 0..=(*set.iter().map(|(x, _)| x).max().unwrap()) {
                if set.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }

        set.len()
    }
}

fn parse(line: &str) -> Option<Either<(i32, i32), (Dir, i32)>> {
    if line.chars().nth(0)? != 'f' {
        let (left, right) = line.split_once(",")?;
        Some(Either::Left((left.parse().ok()?, right.parse().ok()?)))
    } else {
        let (dir, pos) = line
            .split_once("along ")
            .unwrap()
            .1
            .split_once("=")
            .unwrap();
        Some(Either::Right((dir.parse().ok()?, pos.parse().ok()?)))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_parse_input() {
        let res = parse(
            r#"start-A
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"#,
        );
        assert_eq!(res, ("start".to_owned(), "b".to_owned()));
    }
}
