use aoc_common::aoc_day::AocDay;
use aoc_common::bitset::Bitset;
use aoc_common::files::Res;
use itertools::Itertools;
use std::convert::TryFrom;

use super::intcode::Machine;

pub struct Day21;

impl AocDay for Day21 {
    type Input = Vec<i64>;
    type Result1 = i64;
    type Result2 = i64;

    fn day() -> usize {
        21
    }
    fn load() -> Res<Self::Input> {
        Machine::load_tape_from_file("data/2019/day_21.in")
    }

    fn part_1(code: &Self::Input) -> Self::Result1 {
        use Op::*;
        let input = [
            Rule(NOT, 'C', 'J'), // if there is hole at C
            Rule(AND, 'D', 'J'), // only jump if you can land
            Rule(NOT, 'A', 'T'), // if not A then we must jump
            Rule(OR, 'T', 'J'),  // OR the above two rules
        ];

        let mut input = input.iter().map(Rule::to_string).join("\n");
        input.push_str("\nWALK\n");
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

    fn part_2(code: &Self::Input) -> Self::Result2 {
        use Op::*;
        let input = [
            // is_gap -> J
            Rule(NOT, 'B', 'J'), // no B
            Rule(NOT, 'C', 'T'), // np C
            Rule(OR, 'T', 'J'),  // either b and c are holes
            // is gap && D -> J
            Rule(AND, 'D', 'J'), //
            // is gap && D && (E || H) -> J (can_jump)
            Rule(NOT, 'A', 'T'), // clear T to false
            Rule(OR, 'E', 'T'),  // if e
            Rule(OR, 'H', 'T'),  // ... or h
            Rule(AND, 'T', 'J'), // && is_gap && D
            // must_jump
            Rule(NOT, 'A', 'T'), //
            // can_jump || must_jump
            Rule(OR, 'T', 'J'), //
        ];

        let mut input = input.iter().map(Rule::to_string).join("\n");
        input.push_str("\nRUN\n");
        let input: Vec<i64> = input.bytes().map(|b| b as i64).collect();

        let mut machine = Machine::new(code, input);
        machine.run_to_completion();

        let result = machine.output.iter().find(|&&b| u8::try_from(b).is_err());

        if let Some(res) = result {
            *res
        } else {
            let output: String = machine.output.iter().map(|&b| b as u8 as char).collect();
            let output = output.lines().skip(7).join("\n");

            println!("Output: \n{}", output);
            println!("-ABCDEFGHI");
            -1
        }
    }
}

#[allow(unused)]
fn run_springstep<const N: usize>(rules: &[Rule], input: Bitset, log: bool) -> bool {
    use Op::*;

    let mut j = false;
    let mut t = false;

    for r in rules {
        let p1 = if ('A'..='I').contains(&r.1) {
            input.contains((r.1 as u8 - b'A') as usize)
        } else if r.1 == 'T' {
            t
        } else {
            j
        };

        let p2 = if r.2 == 'T' { &mut t } else { &mut j };
        let tp = (p1, *p2);

        match r.0 {
            AND => *p2 &= p1,
            OR => *p2 |= p1,
            NOT => *p2 = !p1,
        };
        if log {
            println!("{:?}, (p1, p2): {:?}, T: {}, J: {}", r, tp, t, j);
        }
    }

    if log {
        println!("@\n#{} : {}", format_path::<N>(input), j);
    }

    j
}

#[derive(Copy, Clone, std::fmt::Debug, Eq, PartialEq)]
enum Op {
    AND,
    OR,
    NOT,
}

#[derive(Copy, Clone, std::fmt::Debug, Eq, PartialEq)]
struct Rule(Op, char, char);

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {}", self.0, self.1, self.2)
    }
}

fn format_path<const N: usize>(input: Bitset) -> String {
    (0..N)
        .map(|i| if input.contains(i) { '#' } else { '.' })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jumping() {
        use Op::*;

        let input = [
            // is_gap -> J
            Rule(NOT, 'B', 'J'), // no B
            Rule(NOT, 'C', 'T'), // np C
            Rule(OR, 'T', 'J'),  // either b and c are holes
            // is gap && D -> J
            Rule(AND, 'D', 'J'), // t iff should jump from b
            // is gap && D && (E || H) -> J
            Rule(NOT, 'A', 'T'),
            Rule(OR, 'E', 'T'), // t if we shouldn't jump then
            Rule(OR, 'H', 'T'), // we jump now
            Rule(AND, 'T', 'J'),
            // must_jump
            Rule(NOT, 'A', 'T'), //
            // jump
            Rule(OR, 'T', 'J'), //
        ];

        let failed_jumps = test_all::<9>(
            &input,
            &[
                //ABCD EFGH I
                0b0111_1000_0, //
                0b1001_1000_0, //
                0b1001_0001_0, //
                0b1101_0011_1, //
                0b1101_1101_0, //
            ],
            true,
        );

        let failed_walks = test_all::<9>(
            &input,
            &[
                //ABCD EFGH I
                0b1111_0101_1, //
                0b1110_1011_0, //
                0b1101_0110_1, //
                0b1101_0100_1, //
            ],
            false,
        );

        assert!(!failed_walks && !failed_jumps);
    }

    fn test_all<const N: usize>(prog: &[Rule], paths: &[u64], is_jumping: bool) -> bool {
        let fails = paths
            .iter()
            .map(|&i| Bitset::from(i).flip_start(N))
            .filter(|&i| run_springstep::<N>(prog, i, false) != is_jumping)
            .map(format_path::<N>)
            .collect::<Vec<_>>();

        if !fails.is_empty() {
            println!(
                "Should have {}:\n{}",
                if is_jumping { "jumped" } else { "walked" },
                fails.iter().join("\n")
            );
            true
        } else {
            false
        }
    }

    #[test]
    fn test_is_gap() {
        use Op::*;
        let input = [
            // is_gap (J)
            Rule(NOT, 'B', 'J'), // no B
            Rule(NOT, 'C', 'T'), // np C
            Rule(OR, 'T', 'J'),  // either b and c are holes
        ];

        let failed_jumps = test_all::<4>(
            &input,
            &[
                // 0b0111, // this is covered by not_a
                0b1001, //
                0b1101, //
                0b1011, //
            ],
            true,
        );

        let failed_walks = test_all::<4>(
            &input,
            &[
                0b0111, //
                0b1111, //
            ],
            false,
        );

        assert!(!failed_walks && !failed_jumps);
    }
}
