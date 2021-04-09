use aoc_common::aoc_day::AocDay;
use aoc_common::files::Res;
use std::collections::HashMap;
use std::fs::read_to_string;

use Instruction::*;
pub struct Day14;

impl AocDay for Day14 {
    type Input = Vec<Instruction>;
    type Result1 = u64;
    type Result2 = u64;

    fn day() -> usize {
        14
    }
    fn load() -> Res<Self::Input> {
        let input = read_to_string("data/2020/day_14.in")?;

        let instructions = input.lines().map(|l| Instruction::parse(l)).collect();

        Ok(instructions)
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        let mut trues: u64 = 0;
        let mut falses: u64 = 0;
        let mut mem: HashMap<u64, u64> = HashMap::new();

        for i in input {
            match i {
                Mask {
                    val_trues,
                    val_falses,
                    ..
                } => {
                    trues = *val_trues;
                    falses = *val_falses;
                }
                SetMem(addr, val) => {
                    mem.insert(*addr, (val | trues) & falses);
                }
            }
        }

        mem.iter().map(|(_, v)| v).sum()
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        let mut trues: u64 = 0;
        let mut floating = 0;
        let mut mem: HashMap<u64, u64> = HashMap::new();

        for i in input {
            match i {
                Mask {
                    val_trues,
                    mem_floating,
                    ..
                } => {
                    trues = *val_trues;
                    floating = *mem_floating;
                }
                SetMem(addr, val) => {
                    // mem.insert(addr, (val | trues) & falses);
                    // let value = (val | trues) & falses;
                    let addr = addr | trues;
                    set_floating_mem(&mut mem, *val, addr, floating)
                }
            }
        }

        // dbg!(&mem);

        mem.iter().map(|(_, v)| v).sum()
    }
}

fn set_floating_mem(mem: &mut HashMap<u64, u64>, value: u64, addr: u64, floating: u64) {
    if floating == 0 {
        mem.insert(addr, value);
    } else {
        let msb = 63 - floating.leading_zeros();
        let new_floating = floating ^ (1 << msb);
        let one_addr = addr | (1 << msb);
        let zero_addr = addr & (0xf_ffff_ffff - (1 << msb));

        set_floating_mem(mem, value, one_addr, new_floating);
        set_floating_mem(mem, value, zero_addr, new_floating);
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Instruction {
    Mask {
        val_trues: u64,
        val_falses: u64,
        mem_floating: u64,
    },
    SetMem(u64, u64), // location, value
}

impl Instruction {
    fn parse(s: &str) -> Self {
        match s.split_once(" = ") {
            Some(("mask", mask)) => {
                let (trues, falses, floating) = mask.chars().fold(
                    (0_u64, 0_u64, 0_u64),
                    |(trues, falses, floating), c| match c {
                        '1' => ((trues << 1) | 1, (falses << 1) | 1, floating << 1),
                        '0' => ((trues << 1), (falses << 1), floating << 1),
                        _ => ((trues << 1), (falses << 1) | 1, (floating << 1) | 1),
                    },
                );
                Instruction::Mask {
                    val_trues: trues,
                    val_falses: falses,
                    mem_floating: floating,
                }
            }
            Some((mem, val)) => {
                let mem: String = mem.split(']').next().unwrap().chars().skip(4).collect();
                Instruction::SetMem(mem.parse().unwrap(), val.parse().unwrap())
            }
            _ => unreachable!("Invalid value {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Instruction::parse("mask = xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"),
            Mask {
                val_trues: 0,
                val_falses: 0xf_ffff_ffff,
                mem_floating: 0xf_ffff_ffff
            }
        );
        assert_eq!(
            Instruction::parse("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
            Mask {
                val_trues: 64,
                val_falses: 0xf_ffff_fffd,
                mem_floating: 0xf_ffff_ffbd
            }
        );
    }

    #[test]
    fn test_msb() {
        if let Mask { mem_floating, .. } =
            Instruction::parse("mask = 000000000000000000000000000000000X0X")
        {
            assert_eq!(mem_floating, 5);
            let msb = 63 - mem_floating.leading_zeros();
            assert_eq!(msb, 2);
            let new_floating = mem_floating ^ (1 << msb);
            assert_eq!(new_floating, 1);
        }
    }

    #[test]
    fn test_part_2() {
        let input = "mask = 000000000000000000000000000000X1001X\n\
        mem[42] = 100\n\
        mask = 00000000000000000000000000000000X0XX\n\
        mem[26] = 1"
            .lines()
            .map(Instruction::parse)
            .collect::<Vec<_>>();
        assert_eq!(Day14::part_2(&input), 208);
    }
}
