use super::Aoc2020;
use crate::files::{read_lines, Res};

pub struct Day01;

impl Aoc2020 for Day01 {
    type Input = Vec<i32>;
    type Result1 = i32;
    type Result2 = i32;

    fn day() -> usize {
        1
    }
    fn load() -> Res<Vec<i32>> {
        let mut input: Vec<_> = read_lines("data/2020/day_01.in")?
            .map(|l| l.unwrap().trim().parse::<i32>().unwrap())
            .collect();
        input.sort_unstable();
        Ok(input)
    }

    fn part_1(expenses: &Vec<i32>) -> i32 {
        find_matching_to_sum(expenses, 2020).unwrap()
    }

    fn part_2(expenses: &Vec<i32>) -> i32 {
        for i in 0..expenses.len() - 2 {
            if let Some(res) = find_matching_to_sum(&expenses[i + 1..], 2020 - expenses[i]) {
                return res * expenses[i];
            }
        }
        panic!("Impossible");
    }
}

fn find_matching_to_sum(expenses: &[i32], goal: i32) -> Option<i32> {
    let mut bottom = 0;
    let mut top = expenses.len() - 1;
    while top != bottom {
        let res = expenses[bottom] + expenses[top];
        match res {
            x if x == goal => return Some(expenses[bottom] * expenses[top]),
            x if x > goal => top -= 1,
            _ => bottom += 1,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let res = find_matching_to_sum(&input, 2020).unwrap();
        assert_eq!(res, 514579);
    }

    #[test]
    fn test_part_2() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let res = Day01::part_2(&input);
        assert_eq!(res, 241861950);
    }
}
