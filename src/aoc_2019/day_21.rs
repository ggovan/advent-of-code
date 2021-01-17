use crate::aoc_2020::Aoc2020;
use crate::files::Res;
use itertools::Itertools;
use std::convert::TryFrom;

use super::intcode::Machine;

pub struct Day21;

impl Aoc2020 for Day21 {
    type Input = Vec<i64>;
    type Result1 = i64;
    type Result2 = u64;

    fn day() -> usize {
        21
    }
    fn load() -> Res<Self::Input> {
        Machine::load_tape_from_file("data/2019/day_21.in")
    }

    fn part_1(code: &Self::Input) -> Self::Result1 {
        let input = [
            "NOT C J", // if there is hole at C
            "AND D J", // only jump if you can land
            "NOT A T", // if not A then we must jump
            "OR T J",  // OR the above two rules
            "WALK",    //
            "",        // add the new line at the end
        ]
        .iter()
        .join("\n");

        let input: Vec<i64> = input.bytes().map(|b| b as i64).collect();
        let mut machine = Machine::new(code, input);

        machine.run_to_completion();

        let result = machine.output.iter().find(|&&b| u8::try_from(b).is_err());

        if let Some(res) = result {
            *res
        } else {
            let output: String = machine.output.iter().map(|&b| b as u8 as char).collect();

            println!("Output: {}", output);
            -1
        }
    }

    fn part_2(_input: &Self::Input) -> Self::Result2 {
        21
    }
}
