use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Res<T> = Result<T, Box<dyn Error>>;

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
            let mut sorted = next_arr;
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
    fn new(mem: &[i32], input: Vec<i32>) -> Self {
        Machine {
            mem: mem.to_owned(),
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

pub fn day_2_part_1() -> Res<()> {
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

pub fn day_2_part_2() -> Res<()> {
    let input: Vec<i32> = read_better("day_2.in", &|s| s.parse::<i32>().unwrap())?
        .nth(0)
        .unwrap();

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

pub fn day_5() -> Res<()> {
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

pub fn day_7() -> Res<()> {
    println!("Day 7");

    let mem: Vec<i32> = read_better("day_7.in", &|s| s.parse::<i32>().unwrap())?
        .nth(0)
        .unwrap();
    let memref = &mem;

    let thruster = Permutations::permute(1234, 43210)
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
