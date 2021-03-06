use aoc_common::files::{read_better, Res};
use aoc_common::geometry::{self, Direction};
use std::collections::{HashMap, HashSet};

struct Permutations {
    base: [i64; 5],
    end: i64,
    current: i64,
}

impl Permutations {
    fn val_to_arr(value: i64) -> [i64; 5] {
        [
            value / 10000,
            value / 1000 % 10,
            value / 100 % 10,
            value / 10 % 10,
            value % 10,
        ]
    }

    fn permute(start: i64, end: i64) -> Self {
        Permutations {
            base: Permutations::val_to_arr(start),
            end,
            current: start,
        }
    }
}

impl Iterator for Permutations {
    type Item = [i64; 5];

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        loop {
            let next_arr = Permutations::val_to_arr(self.current);
            self.current += 1;
            let mut sorted = next_arr;
            sorted.sort_unstable();
            if sorted == self.base {
                break Some(next_arr);
            }
            if self.current > self.end {
                break None;
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Machine {
    pub mem: Vec<i64>,
    pub op_index: usize,
    pub input: Vec<i64>,
    pub input_index: usize,
    pub output: Vec<i64>,
    pub relative_base: i64,
}

#[derive(Debug)]
enum Mode {
    Immediate,
    Address,
    Relative,
}
impl Mode {
    fn from(val: i64) -> Self {
        match val {
            0 => Mode::Address,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            x => panic!("Unknown mode - {}", x),
        }
    }
}

impl Machine {
    pub fn new(mem: &[i64], input: Vec<i64>) -> Self {
        Machine {
            mem: mem.to_owned(),
            input,
            ..Machine::default()
        }
    }

    fn ck_inst(&self, val: i64) -> usize {
        if (0..=9).contains(&val) || val == 99 {
            val as usize
        } else {
            panic!("Invalid instruction {}!\n{:?}", val, self)
        }
    }

    fn ck_addr(&mut self, val: i64) -> usize {
        if 0 <= val && val < self.mem.len() as i64 {
            val as usize
        } else {
            self.mem.resize(val as usize + 1, 0);
            val as usize
        }
    }

    pub fn run_to_completion(&mut self) {
        while self.compute_step().is_some() {}
    }

    pub fn run_to_output(&mut self, input: Option<i64>) -> Option<i64> {
        if let Some(in_val) = input {
            self.input.push(in_val);
        }
        let out_length = self.output.len();
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

    #[allow(dead_code)]
    pub fn run_to_next_input(&mut self, input: Option<i64>) -> Option<usize> {
        if let Some(in_val) = input {
            self.input.push(in_val);
        }
        loop {
            if self.mem[self.op_index] == 99 {
                break None;
            }
            let next = self.compute_step();
            if next.is_none() {
                break None;
            }
            if self.mem[self.op_index] == 3 {
                break Some(self.op_index);
            }
        }
    }

    fn read_op_to_value(&mut self, op: i64, mode: Mode) -> i64 {
        match mode {
            Mode::Address => {
                let addr = self.ck_addr(op);
                self.mem[addr]
            }
            Mode::Immediate => op as i64,
            Mode::Relative => {
                let addr = self.ck_addr(self.relative_base + op);
                self.mem[addr]
            }
        }
    }

    fn write_op_to_addr(&mut self, op: i64, mode: Mode) -> usize {
        self.ck_addr(match mode {
            Mode::Address => op,
            Mode::Immediate => panic!("Writing in Immediate mode is not supported"),
            Mode::Relative => self.relative_base + op,
        })
    }

    fn read_op_and_modes(&self) -> (usize, Mode, Mode, Mode) {
        let raw_val = self.mem[self.op_index];
        (
            self.ck_inst(raw_val % 100),
            Mode::from(raw_val / 100 % 10),
            Mode::from(raw_val / 1000 % 10),
            Mode::from(raw_val / 10000),
        )
    }

    fn read_operands_1(&mut self, mode: Mode) -> (i64, usize) {
        (
            self.read_op_to_value(self.mem[self.op_index + 1], mode),
            self.op_index + 2,
        )
    }
    fn read_operands_2(&mut self, mode_1: Mode, mode_2: Mode) -> (i64, i64, usize) {
        (
            self.read_op_to_value(self.mem[self.op_index + 1], mode_1),
            self.read_op_to_value(self.mem[self.op_index + 2], mode_2),
            self.op_index + 3,
        )
    }
    fn read_operands_3(
        &mut self,
        mode_1: Mode,
        mode_2: Mode,
        mode_3: Mode,
    ) -> (i64, i64, i64, usize) {
        (
            self.read_op_to_value(self.mem[self.op_index + 1], mode_1),
            self.read_op_to_value(self.mem[self.op_index + 2], mode_2),
            self.read_op_to_value(self.mem[self.op_index + 3], mode_3),
            self.op_index + 4,
        )
    }

    fn compute_step(&mut self) -> Option<usize> {
        let (instruction, mode_1, mode_2, mode_3) = self.read_op_and_modes();
        match instruction {
            1 => {
                let (val_1, val_2, dest, next) =
                    self.read_operands_3(mode_1, mode_2, Mode::Immediate);
                let dest = self.write_op_to_addr(dest, mode_3);
                self.mem[dest] = val_1 + val_2;
                self.op_index = next;
                Some(self.op_index)
            }
            2 => {
                let (val_1, val_2, dest, next) =
                    self.read_operands_3(mode_1, mode_2, Mode::Immediate);
                let dest = self.write_op_to_addr(dest, mode_3);
                self.mem[dest] = val_1 * val_2;
                self.op_index = next;
                Some(self.op_index)
            }
            3 => {
                let (dest, next) = self.read_operands_1(Mode::Immediate);
                let dest = self.write_op_to_addr(dest, mode_1);
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
                let dest = self.write_op_to_addr(dest, mode_3);
                self.mem[dest] = if val_1 < val_2 { 1 } else { 0 };
                self.op_index = next;
                Some(self.op_index)
            }
            8 => {
                let (val_1, val_2, dest, next) =
                    self.read_operands_3(mode_1, mode_2, Mode::Immediate);
                let dest = self.write_op_to_addr(dest, mode_3);
                self.mem[dest] = if val_1 == val_2 { 1 } else { 0 };
                self.op_index = next;
                Some(self.op_index)
            }
            9 => {
                let (val_1, next) = self.read_operands_1(mode_1);
                self.relative_base += val_1;
                self.op_index = next;
                Some(self.op_index)
            }
            99 => Option::None,
            x => panic!("Invalid instruction {}", x),
        }
    }

    pub fn load_tape_from_file(file: &str) -> Res<Vec<i64>> {
        Ok(read_better(file, &|s| s.parse::<i64>().unwrap())?
            .next()
            .unwrap())
    }
}

pub fn day_5() -> Res<(i64, i64)> {
    println!("Day 5");

    let mem: Vec<i64> = read_better("data/2019/day_5.in", &|s| s.parse::<i64>().unwrap())?
        .next()
        .unwrap();

    let mut machine = Machine::new(&mem, vec![1]);
    machine.run_to_completion();
    println!("  part 1 {:?}", machine.output);

    let mut machine_2 = Machine::new(&mem, vec![5]);
    machine_2.run_to_completion();
    println!("  part 2 {:?}", machine_2.output);
    Ok((
        *machine.output.last().unwrap(),
        *machine_2.output.last().unwrap(),
    ))
}

pub fn day_7() -> Res<(i64, i64)> {
    println!("Day 7");

    let mem: Vec<i64> = read_better("data/2019/day_7.in", &|s| s.parse::<i64>().unwrap())?
        .next()
        .unwrap();
    let memref = &mem;

    let thruster = Permutations::permute(1234, 43210)
        .map(move |config| {
            config.iter().fold(0, |acc, &v| {
                let mut machine_a = Machine::new(memref, vec![v]);
                machine_a.run_to_output(Some(acc));
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
                let output = m.run_to_output(Some(last_output));
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

    Ok((thruster.unwrap(), thruster_2.unwrap()))
}

pub fn day_9() -> Res<(i64, i64)> {
    println!("Day 9");

    let mem: Vec<i64> = read_better("data/2019/day_9.in", &|s| s.parse::<i64>().unwrap())?
        .next()
        .unwrap();

    let mut machine = Machine::new(&mem, vec![1]);
    machine.run_to_completion();
    println!("  part 1 {:?}", machine.output);
    println!("    max mem content {:?}", machine.mem.iter().max());
    println!("    machine mem size {:?}", machine.mem.len());

    let mut machine_2 = Machine::new(&mem, vec![2]);
    machine_2.run_to_completion();
    println!("  part 2 {:?}", machine_2.output);
    println!("    max mem content {:?}", machine_2.mem.iter().max());
    println!("    machine mem size {:?}", machine_2.mem.len());
    Ok((
        *machine.output.last().unwrap(),
        *machine_2.output.last().unwrap(),
    ))
}

fn painter(machine: &mut Machine, start: bool) -> i64 {
    let mut surface: HashMap<(i64, i64), bool> = HashMap::new();
    let mut pos = (0, 0);
    surface.insert(pos, start);
    let mut dir = Direction::North;

    loop {
        let current = if *surface.get(&pos).unwrap_or(&false) {
            1
        } else {
            0
        };
        if let Some(out_val) = machine.run_to_output(Some(current)) {
            surface.insert(pos, out_val > 0);
        } else {
            break;
        }

        let out_2 = machine.run_to_output(None);
        if let Some(out_val) = out_2 {
            dir = dir.rotate_cw_amount(out_val);
            pos = dir.next_point(pos);
        } else {
            break;
        }
    }

    geometry::output_map(&surface);
    surface.len() as i64
}

pub fn day_11() -> Res<(i64, i64)> {
    println!("Day 11");

    let mem: Vec<i64> = read_better("data/2019/day_11.in", &|s| s.parse::<i64>().unwrap())?
        .next()
        .unwrap();

    let mut machine = Machine::new(&mem, vec![]);

    let res_1 = painter(&mut machine, false);
    println!("  part 1 {:?}", res_1);

    let mut machine = Machine::new(&mem, vec![]);

    let res_2 = painter(&mut machine, true);
    println!("  part 2 {:?}", res_2);

    Ok((res_1, res_2))
}

pub fn day_13() -> Res<i64> {
    println!("Day 13");

    let mem: Vec<i64> = read_better("data/2019/day_13.in", &|s| s.parse::<i64>().unwrap())?
        .next()
        .unwrap();

    let mut machine = Machine::new(&mem, vec![]);

    machine.run_to_completion();

    let squares = machine
        .output
        .chunks_exact(3)
        .map(|chunk| (chunk[0], chunk[1], chunk[2]))
        .collect::<Vec<_>>();
    dbg!(squares.len());

    let mut set: HashSet<(i64, i64)> = HashSet::new();
    let mut map: HashMap<(i64, i64), bool> = HashMap::new();
    for (x, y, ty) in squares {
        map.insert((x, y), ty != 0);
        if ty != 2 {
            continue;
        }
        set.insert((x, y));
    }
    geometry::output_map(&map);

    println!("  part 1 {:?}", set.len());

    let mut machine = Machine::new(&mem, vec![]);
    machine.mem[0] = 2;
    // let mut map: HashMap<(i64, i64), i64> = HashMap::new();
    // let mut count = 0;
    let mut finish = false;

    machine.run_to_next_input(None);
    loop {
        // for i in inputs {
        // count += 1;
        let squares = machine
            .output
            .chunks_exact(3)
            .map(|chunk| (chunk[0], chunk[1], chunk[2]))
            .collect::<Vec<_>>();
        // Uncomment for visualisation.
        // for &(x, y, ty) in &squares {
        //     if x == -1 {
        //         // this is the score
        //         // dbg!(ty);
        //     } else {
        //         map.insert((x, y), ty);
        //     }
        // }
        // geometry::output_map(&map);

        let ball = squares.iter().rev().find(|s| s.2 == 4).unwrap();
        let paddle = squares.iter().rev().find(|s| s.2 == 3).unwrap();
        let score = squares
            .iter()
            .rev()
            .find(|s| s.0 == -1 && s.1 == 0)
            .unwrap()
            .2;

        let next = match (ball.0, paddle.0) {
            (b, p) if b > p => 1,
            (b, p) if b < p => -1,
            _ => 0,
        };
        // dbg!((count, next, score));
        if finish {
            println!("  part 2 {:?}", score);
            return Ok(score);
        }
        if None == machine.run_to_next_input(Some(next)) {
            finish = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produce_copy() {
        let input = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut machine = Machine::new(&input, vec![]);
        machine.run_to_completion();
        assert_eq!(machine.output, input);
    }

    #[test]
    fn output_long_number() {
        let input = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut machine = Machine::new(&input, vec![]);
        machine.run_to_completion();
        let length = format!("{}", machine.output[0]).len();
        assert_eq!(length, 16);
    }

    #[test]
    fn output_middle() {
        let input = vec![104, 1125899906842624, 99];
        let mut machine = Machine::new(&input, vec![]);
        machine.run_to_completion();
        assert_eq!(machine.output[0], input[1]);
    }

    #[test]
    fn increases_relative_base() {
        let input = vec![109, 19];
        let mut machine = Machine {
            mem: input,
            relative_base: 2000,
            ..Machine::default()
        };
        machine.compute_step();
        assert_eq!(machine.relative_base, 2019);
    }

    #[test]
    fn day_5_test() -> Res<()> {
        let res = day_5()?;
        assert_eq!(res.0, 15314507);
        assert_eq!(res.1, 652726);
        Ok(())
    }

    #[test]
    fn day_7_test() -> Res<()> {
        let res = day_7()?;
        assert_eq!(res.0, 38500);
        assert_eq!(res.1, 33660560);
        Ok(())
    }

    #[test]
    fn day_9_test() -> Res<()> {
        let res = day_9()?;
        assert_eq!(res.0, 4006117640);
        assert_eq!(res.1, 88231);
        Ok(())
    }
}
