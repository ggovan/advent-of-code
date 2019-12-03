use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Res<T> = Result<T, Box<dyn Error>>;

fn main() -> Res<()> {
    println!("Advent of code!!!");
    println!("Day 1");
    let day_1_in: Vec<_> = read_lines("day_1.in")?
        .map(|l| l.unwrap().trim().parse::<i32>().unwrap())
        .collect();
    println!("  part 1 {}", day_1(day_1_in.iter()));
    println!("  part 2 {}", day_1_part_2(day_1_in.iter()));

    day_2_part_1()?;
    day_2_part_2()?;
    day_3_part_1()?;

    Ok(())
}

fn day_1<'a>(mass: impl Iterator<Item = &'a i32>) -> i32 {
    mass.map(|m| m / 3 - 2).sum()
}

fn day_1_part_2<'a>(mass: impl Iterator<Item = &'a i32>) -> i32 {
    mass.map(|m| m / 3 - 2)
        .map(|m| m + day_1_part_2_fuel(m))
        .sum()
}

fn day_1_part_2_fuel(mass: i32) -> i32 {
    let more_fuel = mass / 3 - 2;
    if more_fuel <= 0 {
        0
    } else {
        more_fuel + day_1_part_2_fuel(more_fuel)
    }
}

fn read_better<P, R, F>(filename: P, fff: &'static F) -> io::Result<impl Iterator<Item = Vec<R>>>
where
    P: AsRef<Path>,
    F: Fn(&str) -> R,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(move |l| l.unwrap().split(",").map(fff).collect::<Vec<R>>()))
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn day_2_part_1() -> Res<()> {
    println!("Day 2");

    let mut mem: Vec<usize> = read_better("day_2.in", &|s| s.parse::<usize>().unwrap())?
        .nth(0)
        .unwrap();

    mem[1] = 12;
    mem[2] = 2;

    run_to_completion(&mut mem, 0);

    println!("  part 1 {}", mem[0]);
    Ok(())
}

fn day_2_part_2() -> Res<()> {
    let input: Vec<usize> = read_lines("day_2.in")?
        .nth(0)
        .unwrap()?
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut pairs: Vec<(usize, usize)> = vec![];

    for noun in 0..100 {
        for verb in 0..100 {
            let mut mem = input.clone();
            mem[1] = noun;
            mem[2] = verb;
            run_to_completion(&mut mem, 0);
            if mem[0] == 19690720 {
                pairs.push((noun, verb))
            }
        }
    }

    println!("  part 2 {:?}", pairs);
    Ok(())
}

fn run_to_completion(mem: &mut Vec<usize>, start_op_index: usize) {
    let mut next = Some(start_op_index);
    while let Some(i) = next {
        next = compute_step(mem, i)
    }
}

fn compute_step(input: &mut Vec<usize>, op_index: usize) -> Option<usize> {
    let next = match input[op_index] {
        1 => {
            let op_1 = input[op_index + 1];
            let op_2 = input[op_index + 2];
            let dest = input[op_index + 3];
            input[dest] = input[op_1] + input[op_2];
            Some(op_index + 4)
        }
        2 => {
            let op_1 = input[op_index + 1];
            let op_2 = input[op_index + 2];
            let dest = input[op_index + 3];
            input[dest] = input[op_1] * input[op_2];
            Some(op_index + 4)
        }
        99 => Option::None,
        x => panic!("Invalid instruction {}", x),
    };
    next
}

fn day_3_part_1() -> Res<()> {
    let input = read_better("day_3.in", &LineSeg::parse)?;
    let wires = input.map(|ss| Wire::from_segments(&ss)).collect::<Vec<_>>();

    let intersections = Wire::intersection_points(&wires[0], &wires[1]);
    let min = intersections
        .iter()
        .map(|Point(x, y)| x.abs() + y.abs())
        .filter(|x| *x > 0)
        .min()
        .unwrap();
    println!("Day 3 - part 1 {}", min);

    let min_dist = intersections
        .iter()
        .map(|p| wires[0].distance_to(p) + wires[1].distance_to(p))
        .filter(|x| *x > 0)
        .min()
        .unwrap();
    println!("        part 2 {}", min_dist);
    Ok(())
}

#[derive(Debug)]
struct LineSeg {
    start: Point,
    length: i32,
    direction: Direction,
}

impl LineSeg {
    fn parse(input: &str) -> Self {
        let (direction, multiplier) = match input.chars().nth(0).unwrap() {
            'R' => (Direction::H, 1),
            'L' => (Direction::H, -1),
            'U' => (Direction::V, 1),
            'D' => (Direction::V, -1),
            x => panic!("Unexpected direction '{}'", x),
        };
        let length: i32 = input.get(1..).unwrap().parse().unwrap();
        LineSeg {
            start: Point(0, 0),
            length: length * multiplier,
            direction,
        }
    }
    fn end_point(&self) -> Point {
        let Point(x, y) = self.start;
        match self.direction {
            Direction::H => Point(x + self.length, y),
            Direction::V => Point(x, y + self.length),
        }
    }

    fn is_point_on_line(&self, point: &Point) -> bool {
        match self.direction {
            Direction::H => {
                let mut xs = [self.start.0, self.end_point().0];
                xs.sort();
                point.0 >= xs[0] && point.0 <= xs[1] && point.1 == self.start.1
            }
            Direction::V => {
                let mut ys = [self.start.1, self.end_point().1];
                ys.sort();
                point.1 >= ys[0] && point.1 <= ys[1] && point.0 == self.start.0
            }
        }
    }

    fn distance_to_point_on_line(&self, point: &Point) -> i32 {
        match self.direction {
            Direction::H => (point.0 - self.start.0).abs(),
            Direction::V => (point.1 - self.start.1).abs(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);
#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    H,
    V,
}

#[derive(Debug)]
struct Wire {
    segments: Vec<LineSeg>,
}
impl Wire {
    fn from_segments(segs: &Vec<LineSeg>) -> Self {
        let mut pos = Point(0, 0);
        let mut positioned_segs: Vec<LineSeg> = Vec::new();

        for seg in segs {
            let new_seg = LineSeg { start: pos, ..*seg };
            pos = new_seg.end_point();
            positioned_segs.push(new_seg);
        }
        Wire {
            segments: positioned_segs,
        }
    }

    fn intersection_points(w1: &Wire, w2: &Wire) -> Vec<Point> {
        let mut res = Vec::new();
        for seg1 in &w1.segments {
            for seg2 in &w2.segments {
                if let Some(point) = find_intersection(seg1, seg2) {
                    res.push(point)
                }
            }
        }
        res
    }

    fn distance_to(&self, point: &Point) -> i32 {
        let mut dist = 0;
        for seg in &self.segments {
            if seg.is_point_on_line(point) {
                dist += seg.distance_to_point_on_line(point);
                break;
            } else {
                dist += seg.length.abs()
            }
        }
        dist
    }
}

fn find_intersection(seg1: &LineSeg, seg2: &LineSeg) -> Option<Point> {
    match (seg1.direction, seg2.direction) {
        (Direction::H, Direction::V) => {
            let point = Point(seg2.start.0, seg1.start.1);
            if seg1.is_point_on_line(&point) && seg2.is_point_on_line(&point) {
                Some(point)
            } else {
                None
            }
        }
        (Direction::V, Direction::H) => {
            let point = Point(seg1.start.0, seg2.start.1);
            if seg1.is_point_on_line(&point) && seg2.is_point_on_line(&point) {
                Some(point)
            } else {
                None
            }
        }
        _ => None,
    }
}
