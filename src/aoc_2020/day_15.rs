use crate::aoc_2020::Aoc2020;
use crate::files::Res;

pub struct Day15;

impl Aoc2020 for Day15 {
    type Input = Vec<usize>;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        15
    }
    fn load() -> Res<Self::Input> {
        Ok(vec![19, 0, 5, 1, 10, 13])
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        do_it(input, 2020)
    }

    // using a HashMap ~2.3s
    // using a pre-sized 30M elem vec ~900ms
    fn part_2(input: &Self::Input) -> Self::Result2 {
        do_it(input, 30_000_000)
    }
}

fn do_it(input: &[usize], size: usize) -> usize {
    let mut seen = Vec::new();
    seen.resize(size, None);
    let mut prev = 0;

    for (i, v) in input.iter().enumerate() {
        seen[*v] = Some(i);
    }

    for i in input.len()..(size - 1) {
        let next = if let Some(last) = seen[prev] {
            i - last
        } else {
            0
        };
        seen[prev] = Some(i);
        prev = next;
    }

    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = vec![0, 3, 6];
        let res = Day15::part_1(&input);
        assert_eq!(res, 436);
    }
}
