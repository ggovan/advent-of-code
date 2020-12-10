use super::intcode::Machine;
use crate::aoc_2020::Aoc2020;
use crate::files::Res;

pub struct Day02;

impl Aoc2020 for Day02 {
    type Input = Vec<i64>;
    type Result1 = i64;
    type Result2 = i64;

    fn day() -> usize {
        02
    }
    fn load() -> Res<Self::Input> {
        Machine::load_tape_from_file("data/2019/day_2.in")
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        let mut mem: Vec<i64> = input.clone();
        mem[1] = 12;
        mem[2] = 2;

        let mut machine = Machine {
            mem,
            ..Machine::default()
        };

        machine.run_to_completion();
        machine.mem[0]
    }

    fn part_2(input: &Vec<Self::Result2>) -> Self::Result2 {
        let mut pairs: Vec<(i64, i64)> = vec![];

        for noun in 0..100 {
            for verb in 0..100 {
                let mut mem = input.clone();
                mem[1] = noun;
                mem[2] = verb;

                let mut machine = Machine {
                    mem,
                    ..Machine::default()
                };

                machine.run_to_completion();
                if machine.mem[0] == 19_690_720 {
                    pairs.push((noun, verb))
                }
            }
        }

        pairs[0].0 * 100 + pairs[0].1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_2_test() {
        let input = Day02::load().unwrap();
        assert_eq!(Day02::part_1(&input), 9706670);
        assert_eq!(Day02::part_2(&input), 2552);
    }
}
