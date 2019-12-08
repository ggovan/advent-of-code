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
    day_7()?;
    day_8()?;

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

    machine.run_to_completion();

    println!("  part 1 {}", machine.mem[0]);
    Ok(())
}

fn day_5() -> Res<()> {
    println!("Day 5");

    let mem: Vec<i32> = read_better("day_5.in", &|s| s.parse::<i32>().unwrap())?
        .nth(0)
        .unwrap();

    let mut machine = Machine::new(&mem, vec![1]);
    machine.run_to_completion();
    println!("  part 1 {:?}", machine.output);

    let mut machine_2 = Machine::new(&mem, vec![5]);
    machine_2.run_to_completion();
    println!("  part 2 {:?}", machine_2.output);
    Ok(())
}

fn day_7() -> Res<()> {
    println!("Day 7");

    let mem: Vec<i32> = read_better("day_7.in", &|s| s.parse::<i32>().unwrap())?
        .nth(0)
        .unwrap();
    let memref = &mem;

    let thruster = Permutations::permute(01234, 43210)
        .map(move |config| {
            config.iter().fold(0, |acc, &v| {
                let mut machine_a = Machine::new(memref, vec![v]);
                machine_a.run_to_output(acc);
                machine_a.output[0]
            })
        })
        .max();
    println!("  part 1 {:?}", thruster);

    let thruster_2 = Permutations::permute(56789, 98765)
        .map(move |config| {
            let mut machines: Vec<Machine> = config
                .iter()
                .map(|&v| Machine::new(memref, vec![v]))
                .collect();

            let mut next_machine = 0;
            let mut last_output = 0;

            loop {
                let m: &mut Machine = &mut (machines[next_machine]);
                next_machine = (next_machine + 1) % 5;
                let output = m.run_to_output(last_output);
                if let Some(v) = output {
                    last_output = v;
                } else {
                    let e = &machines[4];
                    break e.output[e.output.len() - 1];
                }
            }
        })
        .max();
    println!("  part 2 {:?}", thruster_2);

    Ok(())
}

struct Permutations {
    base: [i32; 5],
    end: i32,
    current: i32,
}

impl Permutations {
    fn val_to_arr(value: i32) -> [i32; 5] {
        [
            value / 10000,
            value / 1000 % 10,
            value / 100 % 10,
            value / 10 % 10,
            value % 10,
        ]
    }

    fn permute(start: i32, end: i32) -> Self {
        Permutations {
            base: Permutations::val_to_arr(start),
            end,
            current: start,
        }
    }
}

impl Iterator for Permutations {
    type Item = [i32; 5];

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        loop {
            let next_arr = Permutations::val_to_arr(self.current);
            self.current += 1;
            let mut sorted = next_arr.clone();
            sorted.sort();
            if sorted == self.base {
                break Some(next_arr);
            }
            if self.current > self.end {
                break None;
            }
        }
    }
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
    fn new(mem: &Vec<i32>, input: Vec<i32>) -> Self {
        Machine {
            mem: mem.clone(),
            op_index: 0,
            input,
            input_index: 0,
            output: vec![],
        }
    }

    fn ck_addr(&self, val: i32) -> usize {
        if 0 <= val && val < self.mem.len() as i32 {
            val as usize
        } else {
            panic!("Address is out of range {}!\n{:?}", val, self)
        }
    }

    fn run_to_completion(&mut self) {
        while self.compute_step().is_some() {}
    }

    fn run_to_output(&mut self, input: i32) -> Option<i32> {
        let out_length = self.output.len();
        self.input.push(input);
        loop {
            let next = self.compute_step();
            if next.is_none() {
                break None;
            }
            if self.output.len() != out_length {
                break Some(self.output[out_length]);
            }
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

            machine.run_to_completion();
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
    let d2 = test / 10_000 % 10;
    let d3 = test / 1_000 % 10;
    let d4 = test / 100 % 10;
    let d5 = test / 10 % 10;
    let d6 = test % 10;

    (d1 == d2) || (d2 == d3) || (d3 == d4) || (d4 == d5) || (d5 == d6)
}

fn has_only_adjacent_digits(test: usize) -> bool {
    let d1 = test / 100_000;
    let d2 = test / 10_000 % 10;
    let d3 = test / 1_000 % 10;
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

type Graph<'a> = HashMap<&'a String, Vec<&'a String>>;

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

    let mut node_map: Graph = HashMap::new();

    for (a, b) in &input {
        let new_val = node_map.remove(a).map_or(vec![b], move |mut v| {
            v.push(b);
            v
        });
        node_map.insert(a, new_val);
    }

    println!("Day 6");
    println!("  part 1 {}", count_orbits(&node_map, &"COM".to_owned(), 0));
    println!("  part 2 {:?}", shift_orbits(&node_map, &"COM".to_owned()));
    Ok(())
}

fn count_orbits(graph: &Graph, node: &String, depth: usize) -> usize {
    let empty_vec = vec![];
    graph
        .get(node)
        .unwrap_or(&empty_vec)
        .iter()
        .map(move |n| count_orbits(graph, n, depth + 1))
        .sum::<usize>()
        + depth
}

fn shift_orbits(graph: &Graph, node: &String) -> OrbitDiff {
    use OrbitDiff::*;
    let empty_vec = vec![];
    match node.as_ref() {
        "SAN" => San(0),
        "YOU" => You(0),
        _ => {
            let (both, san, you): (OrbitDiff, OrbitDiff, OrbitDiff) = graph
                .get(node)
                .unwrap_or(&empty_vec)
                .iter()
                .map(move |n| shift_orbits(graph, n))
                .filter(|&d| d != Neither)
                .fold((Neither, Neither, Neither), |(b, s, y), c| match c {
                    Both(_) => (c, Neither, Neither),
                    San(_) => (Neither, c, y),
                    You(_) => (Neither, s, c),
                    _ => (b, s, y),
                });

            match (both, san, you) {
                (Both(x), _, _) => Both(x),
                (_, San(x), You(y)) => Both(x + y),
                (_, San(x), _) => San(x + 1),
                (_, _, You(x)) => You(x + 1),
                _ => OrbitDiff::Neither,
            }
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum OrbitDiff {
    San(usize),
    You(usize),
    Both(usize),
    Neither,
}

fn day_8() -> Res<()> {
    let width = 25;
    let height = 6;
    let layer_size = width * height;

    let input = read_lines("day_8.in")?
        .nth(0)
        .unwrap()?
        .chars()
        .collect::<Vec<_>>();
    let layer_count = input.len() / width / height;

    let (_zeros, ones, twos) = (0..layer_count)
        .map(|layer| {
            input[layer * layer_size..(layer + 1) * layer_size]
                .iter()
                .fold((0, 0, 0), |(z, o, t), c| match c {
                    '0' => (z + 1, o, t),
                    '1' => (z, o + 1, t),
                    '2' => (z, o, t + 1),
                    _ => (z, o, t),
                })
        })
        .min_by_key(|acc| acc.0)
        .unwrap();

    println!("Day 8");
    println!("  part 1 {}", ones * twos);

    let mut output = (0..height)
        .map(|_| (0..width).map(|_| '2').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (i, &c) in input.iter().enumerate() {
        let row = (i % layer_size) / width;
        let col = i % width;
        if output[row][col] == '2' {
            output[row][col] = c;
        }
    }

    let out_string = output
        .iter()
        .map(|row| {
            row.iter()
                .map(|&v| if v == '1' { 'x' } else { ' ' })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");
    println!("  part 2:");
    println!("{}", out_string);

    Ok(())
}
