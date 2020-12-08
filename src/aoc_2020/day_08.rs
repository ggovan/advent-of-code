use super::Aoc2020;
use crate::files::{read_lines, Res};
use std::collections::HashSet;
use std::str::FromStr;

pub struct Day08;

impl Aoc2020 for Day08 {
    type Input = Vec<Instruction>;
    type Result1 = i64;
    type Result2 = i64;

    fn day() -> usize {
        8
    }

    fn load() -> Res<Self::Input> {
        Ok(read_lines("data/2020/day_08.in")?
            .map(|l| l.unwrap().parse::<Instruction>().unwrap())
            .collect())
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        let mut computer = Computer::new(input.clone());

        halts(&mut computer);

        computer.acc
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        let mut input = input.clone();

        for i in 0..input.len() {
            let code = input[i];
            match code {
                Instruction::Jmp(v) => {
                    input[i] = Instruction::Nop(v);
                    let mut c = Computer::new(input);
                    if halts(&mut c) {
                        return c.acc;
                    }
                    // since the program isn't mutated, we can move it back to use on the next iteration
                    input = c.program;
                    input[i] = Instruction::Jmp(v);
                }
                Instruction::Nop(v) => {
                    input[i] = Instruction::Jmp(v);
                    let mut c = Computer::new(input);
                    if halts(&mut c) {
                        return c.acc;
                    }
                    // since the program isn't mutated, we can move it back to use on the next iteration
                    input = c.program;
                    input[i] = Instruction::Nop(v);
                }
                _ => (),
            }
        }

        panic!("Should have found a solution");
    }
}

fn halts(c: &mut Computer) -> bool {
    let mut used_instructions: HashSet<usize> = HashSet::new();

    while !used_instructions.contains(&c.ptr) {
        if c.ptr == c.program.len() {
            return true;
        }
        used_instructions.insert(c.ptr);
        c.run_step();
    }

    false
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

impl FromStr for Instruction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(input: &str) -> Res<Self> {
        use Instruction::*;
        let mut i = input.split(' ');
        let code = i.next().unwrap();
        let value = i.next().unwrap().parse().unwrap();
        Ok(match code {
            "acc" => Acc(value),
            "jmp" => Jmp(value),
            "nop" => Nop(value),
            x => panic!(format!("Unknown instruction {} {}", x, value)),
        })
    }
}

pub struct Computer {
    program: Vec<Instruction>,
    acc: i64,
    ptr: usize,
}

impl Computer {
    fn new(program: Vec<Instruction>) -> Self {
        Computer {
            program,
            acc: 0,
            ptr: 0,
        }
    }

    fn run_step(&mut self) {
        use Instruction::*;
        match self.program[self.ptr] {
            Nop(_) => self.ptr += 1,
            Jmp(value) => self.ptr = (self.ptr as i64 + value) as usize,
            Acc(value) => {
                self.acc += value;
                self.ptr += 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        use Instruction::*;
        assert_eq!("acc +3".parse::<Instruction>().unwrap(), Acc(3));
        assert_eq!("jmp -99".parse::<Instruction>().unwrap(), Jmp(-99));
        assert_eq!("nop +0".parse::<Instruction>().unwrap(), Nop(0));
    }

    #[test]
    fn test_part_1() {
        let input = "nop +0\n\
                     acc +1\n\
                     jmp +4\n\
                     acc +3\n\
                     jmp -3\n\
                     acc -99\n\
                     acc +1\n\
                     jmp -4\n\
                     acc +6";
        let program = input
            .split('\n')
            .map(|l| l.parse::<Instruction>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(Day08::part_1(&program), 5);
    }
}
