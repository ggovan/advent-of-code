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
            let code = input[i].code;
            match code {
                InstructionCode::Jmp => {
                    input[i].code = InstructionCode::Nop;
                    let mut c = Computer::new(input);
                    if halts(&mut c) {
                        return c.acc;
                    }
                    // since the program isn't mutated, we can move it back to use on the next iteration
                    input = c.program;
                    input[i].code = InstructionCode::Jmp;
                }
                InstructionCode::Nop => {
                    input[i].code = InstructionCode::Jmp;
                    let mut c = Computer::new(input);
                    if halts(&mut c) {
                        return c.acc;
                    }
                    // since the program isn't mutated, we can move it back to use on the next iteration
                    input = c.program;
                    input[i].code = InstructionCode::Nop;
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
pub enum InstructionCode {
    Acc,
    Jmp,
    Nop,
}

impl FromStr for InstructionCode {
    type Err = Box<dyn std::error::Error>;

    fn from_str(input: &str) -> Res<Self> {
        use InstructionCode::*;
        Ok(match input {
            "acc" => Acc,
            "jmp" => Jmp,
            "nop" => Nop,
            x => panic!(format!("Unknown instruction {}", x)),
        })
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Instruction {
    code: InstructionCode,
    value: i64,
}

impl FromStr for Instruction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(input: &str) -> Res<Self> {
        let mut i = input.split(' ');
        Ok(Instruction {
            code: i.next().unwrap().parse().unwrap(),
            value: i.next().unwrap().parse().unwrap(),
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
        use InstructionCode::*;
        match self.program[self.ptr] {
            Instruction {
                code: Nop,
                value: _,
            } => self.ptr += 1,
            Instruction { code: Jmp, value } => self.ptr = (self.ptr as i64 + value) as usize,
            Instruction { code: Acc, value } => {
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
        assert_eq!(
            "acc +3".parse::<Instruction>().unwrap(),
            Instruction {
                code: InstructionCode::Acc,
                value: 3,
            }
        );
        assert_eq!(
            "jmp -99".parse::<Instruction>().unwrap(),
            Instruction {
                code: InstructionCode::Jmp,
                value: -99,
            }
        );
        assert_eq!(
            "nop +0".parse::<Instruction>().unwrap(),
            Instruction {
                code: InstructionCode::Nop,
                value: 0,
            }
        );
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
