use aoc_common::aoc_day::AocDay;
use aoc_common::files::Res;
use std::fs::read_to_string;

pub struct Day16;

impl AocDay for Day16 {
    type Input = Vec<i32>;
    type Result1 = i32;
    type Result2 = i32;

    fn day() -> usize {
        16
    }
    fn load() -> Res<Self::Input> {
        Ok(read_to_string("data/2019/day_16.in")?
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| (c as i32 - '0' as i32))
            .collect::<Vec<_>>())
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        let mut input = input.to_owned();

        for _ in 0..100 {
            input = calc_phase(&input, 1);
        }

        input.iter().take(8).fold(0, |acc, v| acc * 10 + *v)
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        let src = input;
        let mut input = input.to_owned();

        let offset: usize = src.iter().take(7).fold(0, |acc, v| acc * 10 + *v as usize);

        for _ in 1..10_000 {
            input.extend(src)
        }

        assert!(
            offset as f64 / input.len() as f64 > 0.5,
            "Otherwise the algorithm is invalid.\
             i.e. we support a bunch of zeros then a bunch of ones."
        );

        // These are never used (always *0)
        input = input.into_iter().skip(offset).collect();

        for _ in 0..100 {
            input = calc_phase_2(&input);
        }

        input.iter().take(8).fold(0, |acc, v| acc * 10 + *v)
    }
}

fn calc_phase_2(input: &[i32]) -> Vec<i32> {
    let mut next = input.to_owned();

    for i in (0..input.len() - 1).rev() {
        next[i] = (next[i + 1] + input[i]) % 10;
    }

    next
}

fn calc_phase(input: &[i32], skip: usize) -> Vec<i32> {
    let pattern = [0, 1, 0, -1];
    (0..input.len())
        .map(|row| {
            let ps = row + 1;
            (input
                .iter()
                .enumerate()
                .map(|(row, x)| x * pattern[((row + skip) / ps) % 4])
                .sum::<i32>()
                % 10)
                .abs()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_phase() {
        let input = "12345678"
            .chars()
            .map(|c| c as i32 - '0' as i32)
            .collect::<Vec<_>>();
        let out = calc_phase(&input, 1);
        let res: i32 = out.iter().fold(0, |acc, v| acc * 10 + *v);
        assert_eq!(res, 48226158);
    }

    // These take a long time, ~200ms
    // #[test]
    // fn part_1() {
    //     let input = Day16::load().unwrap();
    //     let res = Day16::part_1(&input);
    //     assert_eq!(res, 63483758);
    // }

    // #[test]
    // fn part_2() {
    //     let input = Day16::load().unwrap();
    //     let res = Day16::part_2(&input);
    //     assert_eq!(res, 96099551);
    // }

    #[test]
    fn part_1_example_1() {
        let input = "80871224585914546619083218645595"
            .chars()
            .map(|c| c as i32 - '0' as i32)
            .collect::<Vec<_>>();
        let res = Day16::part_1(&input);
        assert_eq!(res, 24176176);
    }

    #[test]
    fn part_2_example_1() {
        let input = "03036732577212944063491565474664"
            .chars()
            .map(|c| c as i32 - '0' as i32)
            .collect::<Vec<_>>();
        let res = Day16::part_2(&input);
        assert_eq!(res, 84462026);
    }

    #[test]
    fn part_2_example_2() {
        let input = "02935109699940807407585447034323"
            .chars()
            .map(|c| c as i32 - '0' as i32)
            .collect::<Vec<_>>();
        let res = Day16::part_2(&input);
        assert_eq!(res, 78725270);
    }
}
