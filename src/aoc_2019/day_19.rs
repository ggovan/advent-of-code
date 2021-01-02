use super::intcode::Machine;
use crate::aoc_2020::Aoc2020;
use crate::files::Res;

pub struct Day19;

impl Aoc2020 for Day19 {
    type Input = Vec<i64>;
    type Result1 = usize;
    type Result2 = i64;

    fn day() -> usize {
        19
    }
    fn load() -> Res<Self::Input> {
        Machine::load_tape_from_file("data/2019/day_19.in")
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        (0..50)
            .map(|x| {
                (0..50)
                    .filter(|&y| {
                        let mut machine = Machine::new(input, vec![x, y]);
                        machine.run_to_output(None) == Some(1)
                    })
                    .count()
            })
            .sum()
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        let mut min_x = 0;

        let check = |x: i64, y: i64| -> bool {
            let mut machine = Machine::new(input, vec![x, y]);
            machine.run_to_output(None) == Some(1)
        };

        for y in 99.. {
            let mut seen_x = false;

            for x in min_x.. {
                let in_beam = check(x, y);
                if !seen_x && in_beam {
                    seen_x = true;
                    min_x = x
                }
                if seen_x && !in_beam {
                    break;
                }

                let check_x = check(x + 99, y);
                let check_y = check(x, y + 99);

                if check_x && check_y {
                    return x * 10000 + y;
                }
            }
        }

        unreachable!();
    }
}
