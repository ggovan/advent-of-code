use super::Aoc2020;
use crate::files::{read_lines, Res};
use std::cmp;

pub struct Day09;

impl Aoc2020 for Day09 {
    type Input = Vec<i64>;
    type Result1 = i64;
    type Result2 = i64;

    fn day() -> usize {
        9
    }

    fn load() -> Res<Self::Input> {
        Ok(read_lines("data/2020/day_09.in")?
            .map(|l| l.unwrap().parse::<i64>().unwrap())
            .collect())
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        has_match(input, 25)
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        contiguous_match(input, 18272118)
    }
}

fn has_match(ns: &[i64], preamble: usize) -> i64 {
    assert!(preamble <= 25);
    let mut thing: Vec<[i64; 24]> = Vec::with_capacity(ns.len());

    for i in 0..(ns.len() - 1) {
        let mut arr = [0; 24];
        for j in (i + 1)..cmp::min(i + preamble, ns.len()) {
            arr[j - i - 1] = ns[i] + ns[j]
        }
        thing.push(arr);
    }

    'outer: for (i, &ni) in ns.iter().enumerate().skip(preamble) {
        // clippy prefered this line to: `for j in (i - preamble)..(i - 1)`
        for (j, &tj) in thing.iter().enumerate().take(i - 1).skip(i - preamble) {
            let t = tj;
            for &tk in t.iter().take(i - j - 1) {
                if ni == tk {
                    continue 'outer;
                }
            }
        }
        return ns[i];
    }

    panic!("not found")
}

fn contiguous_match(ns: &[i64], target: i64) -> i64 {
    let mut head = 0;
    let mut tail = 1;
    let mut sum = ns[head] + ns[tail];

    loop {
        match sum.cmp(&target) {
            cmp::Ordering::Equal => {
                let slice = &ns[head..=tail];
                let min = slice.iter().min().unwrap();
                let max = slice.iter().max().unwrap();
                return min + max;
            }
            cmp::Ordering::Greater => {
                sum -= ns[head];
                head += 1;
            }
            cmp::Ordering::Less => {
                tail += 1;
                sum += ns[tail];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(has_match(&input, 5), 127);
    }

    #[test]
    fn test_example_2() {
        let input = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(contiguous_match(&input, 127), 62);
    }
}
