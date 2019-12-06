use std::collections::HashMap;
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
    println!("  part 1 {}", day_1(&day_1_in));
    println!("  part 2 {}", day_1_part_2(&day_1_in));

    day_2_part_1()?;
    day_2_part_2()?;
    day_3()?;

    day_4();

    day_5()?;
    day_6()?;

    Ok(())
}

fn day_1(mass: &[i32]) -> i32 {
    mass.iter().map(|m| m / 3 - 2).sum()
}

fn day_1_part_2(mass: &[i32]) -> i32 {
    mass.iter()
        .map(|m| m / 3 - 2)
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

fn read_better<P, R, F>(
    filename: P,
    item_parser: &'static F,
) -> io::Result<impl Iterator<Item = Vec<R>>>
where
    P: AsRef<Path>,
    F: Fn(&str) -> R,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(move |l| l.unwrap().split(',').map(item_parser).collect::<Vec<R>>()))
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

    let mut mem: Vec<i32> = read_better("day_2.in", &|s| s.parse::<i32>().unwrap())?
        .nth(0)
        .unwrap();

    mem[1] = 12;
    mem[2] = 2;

    let mut machine = Machine {
        mem,
        op_index: 0,
        input: vec![],
        input_index: 0,
        output: vec![],
    };

    machine.run_to_completion(0);

    println!("  part 1 {}", machine.mem[0]);
    Ok(())
}

fn day_5() -> Res<()> {
    println!("Day 5");

    let mem: Vec<i32> = read_better("day_5.in", &|s| s.parse::<i32>().unwrap())?
        .nth(0)
        .unwrap();

    let mut machine = Machine {
        mem: mem.clone(),
        op_index: 0,
        input: vec![1],
        input_index: 0,
        output: vec![],
    };
    machine.run_to_completion(0);
    println!("  part 1 {:?}", machine.output);

    let mut machine_2 = Machine {
        mem,
        op_index: 0,
        input: vec![5],
        input_index: 0,
        output: vec![],
    };
    machine_2.run_to_completion(0);
    println!("  part 2 {:?}", machine_2.output);
    Ok(())
}

#[derive(Debug)]
struct Machine {
    mem: Vec<i32>,
    op_index: usize,
    input: Vec<i32>,
    input_index: usize,
    output: Vec<i32>,
}

#[derive(Debug)]
enum Mode {
    Immediate,
    Address,
}
impl Mode {
    fn from(val: i32) -> Self {
        match val {
            0 => Mode::Address,
            _ => Mode::Immediate,
        }
    }
}

impl Machine {
    fn ck_addr(&self, val: i32) -> usize {
        if 0 <= val && val < self.mem.len() as i32 {
            val as usize
        } else {
            panic!("Address is out of range {}!\n{:?}", val, self)
        }
    }

    fn run_to_completion(&mut self, start_op_index: usize) {
        let mut next = Some(start_op_index);
        while let Some(_i) = next {
            next = self.compute_step();
        }
    }

    fn op_to_value(&self, op: i32, mode: Mode) -> i32 {
        match mode {
            Mode::Immediate => op as i32,
            Mode::Address => self.mem[self.ck_addr(op)],
        }
    }

    fn read_op_and_modes(&self) -> (usize, Mode, Mode, Mode) {
        let raw_val = self.mem[self.op_index];
        (
            self.ck_addr(raw_val % 100),
            Mode::from(raw_val / 100 % 10),
            Mode::from(raw_val / 1000 % 10),
            Mode::from(raw_val / 10000),
        )
    }

    fn read_operands_1(&self, mode: Mode) -> (i32, usize) {
        (
            self.op_to_value(self.mem[self.op_index + 1], mode),
            self.op_index + 2,
        )
    }
    fn read_operands_2(&self, mode_1: Mode, mode_2: Mode) -> (i32, i32, usize) {
        (
            self.op_to_value(self.mem[self.op_index + 1], mode_1),
            self.op_to_value(self.mem[self.op_index + 2], mode_2),
            self.op_index + 3,
        )
    }
    fn read_operands_3(&self, mode_1: Mode, mode_2: Mode, mode_3: Mode) -> (i32, i32, i32, usize) {
        (
            self.op_to_value(self.mem[self.op_index + 1], mode_1),
            self.op_to_value(self.mem[self.op_index + 2], mode_2),
            self.op_to_value(self.mem[self.op_index + 3], mode_3),
            self.op_index + 4,
        )
    }

    fn compute_step(&mut self) -> Option<usize> {
        let (instruction, mode_1, mode_2, _mode_3) = self.read_op_and_modes();
        match instruction {
            1 => {
                let (val_1, val_2, dest, next) =
                    self.read_operands_3(mode_1, mode_2, Mode::Immediate);
                let dest = self.ck_addr(dest);
                self.mem[dest] = val_1 + val_2;
                self.op_index = next;
                Some(self.op_index)
            }
            2 => {
                let (val_1, val_2, dest, next) =
                    self.read_operands_3(mode_1, mode_2, Mode::Immediate);
                let dest = self.ck_addr(dest);
                self.mem[dest] = val_1 * val_2;
                self.op_index = next;
                Some(self.op_index)
            }
            3 => {
                let (dest, next) = self.read_operands_1(Mode::Immediate);
                let dest = self.ck_addr(dest);
                self.mem[dest] = self.input[self.input_index];
                self.input_index += 1;
                self.op_index = next;
                Some(self.op_index)
            }
            4 => {
                let (val_1, next) = self.read_operands_1(mode_1);
                self.output.push(val_1);
                self.op_index = next;
                Some(self.op_index)
            }
            5 => {
                let (val_1, val_2, maybe_next) = self.read_operands_2(mode_1, mode_2);
                let next = if val_1 != 0 {
                    self.ck_addr(val_2)
                } else {
                    maybe_next
                };
                self.op_index = next;
                Some(self.op_index)
            }
            6 => {
                let (val_1, val_2, maybe_next) = self.read_operands_2(mode_1, mode_2);
                let next = if val_1 == 0 {
                    self.ck_addr(val_2)
                } else {
                    maybe_next
                };
                self.op_index = next;
                Some(self.op_index)
            }
            7 => {
                let (val_1, val_2, dest, next) =
                    self.read_operands_3(mode_1, mode_2, Mode::Immediate);
                let dest = self.ck_addr(dest);
                self.mem[dest] = if val_1 < val_2 { 1 } else { 0 };
                self.op_index = next;
                Some(self.op_index)
            }
            8 => {
                let (val_1, val_2, dest, next) =
                    self.read_operands_3(mode_1, mode_2, Mode::Immediate);
                let dest = self.ck_addr(dest);
                self.mem[dest] = if val_1 == val_2 { 1 } else { 0 };
                self.op_index = next;
                Some(self.op_index)
            }
            99 => Option::None,
            x => panic!("Invalid instruction {}", x),
        }
    }
}

fn day_2_part_2() -> Res<()> {
    let input: Vec<i32> = read_lines("day_2.in")?
        .nth(0)
        .unwrap()?
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut pairs: Vec<(i32, i32)> = vec![];

    for noun in 0..100 {
        for verb in 0..100 {
            let mut mem = input.clone();
            mem[1] = noun;
            mem[2] = verb;

            let mut machine = Machine {
                mem,
                op_index: 0,
                input: vec![],
                input_index: 0,
                output: vec![],
            };

            machine.run_to_completion(0);
            if machine.mem[0] == 19_690_720 {
                pairs.push((noun, verb))
            }
        }
    }

    println!("  part 2 {:?}", pairs);
    Ok(())
}

fn day_3() -> Res<()> {
    println!("Day 3");
    let input = read_better("day_3.in", &LineSeg::parse)?;
    let wires = input.map(|ss| Wire::from_segments(&ss)).collect::<Vec<_>>();

    let intersections = Wire::intersection_points(&wires[0], &wires[1]);
    let min = intersections
        .iter()
        .map(|Point(x, y)| x.abs() + y.abs())
        .filter(|x| *x > 0)
        .min()
        .unwrap();
    println!("  part 1 {}", min);

    let min_dist = intersections
        .iter()
        .map(|&p| wires[0].distance_to(p) + wires[1].distance_to(p))
        .filter(|x| *x > 0)
        .min()
        .unwrap();
    println!("  part 2 {}", min_dist);
    Ok(())
}

#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    H,
    V,
}

#[derive(Debug)]
struct LineSeg {
    start: Point,
    length: i32,
    direction: Direction,
}

#[derive(Debug)]
struct Wire {
    segments: Vec<LineSeg>,
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

    fn is_point_on_line(&self, point: Point) -> bool {
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

    fn distance_to_point_on_line(&self, point: Point) -> i32 {
        match self.direction {
            Direction::H => (point.0 - self.start.0).abs(),
            Direction::V => (point.1 - self.start.1).abs(),
        }
    }

    fn find_intersection(seg1: &LineSeg, seg2: &LineSeg) -> Option<Point> {
        match (seg1.direction, seg2.direction) {
            (Direction::H, Direction::V) => {
                let point = Point(seg2.start.0, seg1.start.1);
                if seg1.is_point_on_line(point) && seg2.is_point_on_line(point) {
                    Some(point)
                } else {
                    None
                }
            }
            (Direction::V, Direction::H) => {
                let point = Point(seg1.start.0, seg2.start.1);
                if seg1.is_point_on_line(point) && seg2.is_point_on_line(point) {
                    Some(point)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Wire {
    fn from_segments(segs: &[LineSeg]) -> Self {
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
                if let Some(point) = LineSeg::find_intersection(seg1, seg2) {
                    res.push(point)
                }
            }
        }
        res
    }

    fn distance_to(&self, point: Point) -> i32 {
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

fn day_4() {
    println!("Day 4");
    let from = 171_309;
    let until = 643_063;
    {
        let mut res_count = 0;

        for test in from..=until {
            if has_adjacent_digits(test) && all_digits_incrementing(test) {
                res_count += 1;
            }
        }

        println!("  part 1 {}", res_count);
    }
    {
        let mut res_count = 0;

        for test in from..=until {
            if has_only_adjacent_digits(test) && all_digits_incrementing(test) {
                res_count += 1;
            }
        }

        println!("  part 2 {}", res_count);
    }
}

fn has_adjacent_digits(test: usize) -> bool {
    let d1 = test / 100_000;
    let d2 = test / 10000 % 10;
    let d3 = test / 1000 % 10;
    let d4 = test / 100 % 10;
    let d5 = test / 10 % 10;
    let d6 = test % 10;

    (d1 == d2) || (d2 == d3) || (d3 == d4) || (d4 == d5) || (d5 == d6)
}

fn has_only_adjacent_digits(test: usize) -> bool {
    let d1 = test / 100_000;
    let d2 = test / 10000 % 10;
    let d3 = test / 1000 % 10;
    let d4 = test / 100 % 10;
    let d5 = test / 10 % 10;
    let d6 = test % 10;

    (d1 == d2 && d2 != d3)
        || (d1 != d2 && d2 == d3 && d3 != d4)
        || (d2 != d3 && d3 == d4 && d4 != d5)
        || (d3 != d4 && d4 == d5 && d5 != d6)
        || (d4 != d5 && d5 == d6)
}

fn all_digits_incrementing(test: usize) -> bool {
    let d1 = test / 100_000;
    let d2 = test / 10000 % 10;
    let d3 = test / 1000 % 10;
    let d4 = test / 100 % 10;
    let d5 = test / 10 % 10;
    let d6 = test % 10;

    d1 <= d2 && d2 <= d3 && d3 <= d4 && d4 <= d5 && d5 <= d6
}

struct Node {
    name: String,
    children: Vec<Node>,
}

fn day_6() -> Res<()> {
    let input = read_lines("day_6.in")?
        .map(|x| {
            let unwrapped = x.unwrap();
            let parts = unwrapped
                .split(')')
                .map(|x| x.to_string())
                .collect::<Vec<_>>();
            (parts[0].clone(), parts[1].clone())
        })
        .collect::<Vec<_>>();

    let mut node_map: HashMap<&String, Vec<&String>> = HashMap::new();

    for (a, b) in &input {
        let new_val = node_map.remove(a).map_or(vec![b], move |mut v| {
            v.push(b);
            v
        });
        node_map.insert(a, new_val);
    }

    let root = build_tree(&node_map, &"COM".to_owned());

    println!("Day 6");
    println!("  part 1 {}", count_orbits(&root, 0));
    Ok(())
}

fn build_tree(node_map: &HashMap<&String, Vec<&String>>, node_name: &String) -> Node {
    Node {
        name: node_name.clone(),
        children: node_map.get(node_name).map_or(vec![], move |v| {
            v.iter().map(|c| build_tree(node_map, c)).collect()
        }),
    }
}

fn count_orbits(node: &Node, depth: usize) -> usize {
    node.children
        .iter()
        .map(move |n| count_orbits(n, depth + 1))
        .sum::<usize>()
        + depth
}
