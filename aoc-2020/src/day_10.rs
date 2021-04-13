use aoc_common::aoc_day::AocDay;
use aoc_common::files::{read_lines, Res};

pub struct Day10;

impl AocDay for Day10 {
    type Input = Vec<u64>;
    type Result1 = u64;
    type Result2 = u64;

    fn day() -> usize {
        10
    }

    fn load() -> Res<Self::Input> {
        let mut input = read_lines("data/2020/day_10.in")?
            .map(|l| l.unwrap().parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        input.sort_unstable();
        Ok(input)
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        // dbg!(input);
        let (ones, threes, _) =
            input
                .iter()
                .fold((0, 0, 0), |(ones, threes, previous), current| {
                    if previous + 3 == *current {
                        (ones, threes + 1, *current)
                    } else if previous + 1 == *current {
                        (ones + 1, threes, *current)
                    } else {
                        unreachable!(
                            "I have misunderstood the problem, {} {} {} {}",
                            ones, threes, previous, current
                        );
                    }
                });

        ones * (threes + 1)
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        let mut input = input.clone();
        input.insert(0, 0);
        let mut run = [1; 4];
        for i in (0..(input.len() - 2)).rev() {
            run.rotate_right(1);
            run[0] = 0;

            let max = input[i] + 3;

            for x in 1..=3 {
                if x + i >= input.len() {
                    continue;
                }
                if input[x + i] <= max {
                    run[0] += run[x];
                }
            }
        }

        run[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example_1() {
        let mut input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        input.sort_unstable();
        assert_eq!(Day10::part_1(&input), 35);
    }

    #[test]
    fn part_2_example_1() {
        let mut input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        input.sort_unstable();
        assert_eq!(Day10::part_2(&input), 8);
    }

    #[test]
    fn test_example_2() {
        let mut input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        input.sort_unstable();
        assert_eq!(Day10::part_1(&input), 220);
    }

    #[test]
    fn part_2_example_2() {
        let mut input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        input.sort_unstable();
        assert_eq!(Day10::part_2(&input), 19208);
    }
}
