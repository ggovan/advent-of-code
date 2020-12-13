use crate::aoc_2020::Aoc2020;
use crate::files::Res;
use std::fs::read_to_string;

pub struct Day13;

impl Aoc2020 for Day13 {
    type Input = (i64, Vec<Option<i64>>);
    type Result1 = i64;
    type Result2 = i64;

    fn day() -> usize {
        13
    }
    fn load() -> Res<Self::Input> {
        let input = read_to_string("data/2020/day_13.in")?;
        let time: i64 = input.lines().next().unwrap().parse().unwrap();
        let busses: Vec<Option<i64>> = input
            .lines()
            .nth(1)
            .unwrap()
            .split(',')
            .map(|n| n.parse().ok())
            .collect();
        Ok((time, busses))
    }

    fn part_1((time, busses): &Self::Input) -> Self::Result1 {
        let (bus, wait) = busses
            .iter()
            .filter_map(|b| *b)
            .map(|b| {
                let prev = time / b;
                let next = b * (prev + 1);
                (b, next - time)
            })
            .min_by(|(_, t1), (_, t2)| t1.cmp(t2))
            .unwrap();
        bus * wait
    }

    fn part_2((_, busses): &Self::Input) -> Self::Result2 {
        busses
            .iter()
            .enumerate()
            .filter_map(|(i, b)| b.map(|b| (i, b)))
            .fold((0, 1), |(offset, period), bus| {
                find_pattern(bus, offset, period)
            })
            .0
    }
}

fn find_pattern((o, b): (usize, i64), offset: i64, period: i64) -> (i64, i64) {
    let mut new_offset = offset;
    loop {
        if (new_offset + o as i64) % b == 0 {
            break;
        }
        new_offset += period;
    }

    let mut new_period = period;
    loop {
        if (new_offset + new_period + o as i64) % b == 0 {
            break;
        }
        new_period += period;
    }

    (new_offset, new_period)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_pattern() {
        assert_eq!(find_pattern((0, 7), 0, 1), (0, 7));
        assert_eq!(find_pattern((1, 13), 0, 7), (77, 91));
        assert_eq!(find_pattern((4, 59), 77, 91), (350, 5369));
        assert_eq!(find_pattern((6, 31), 350, 5369), (70147, 166439));
        assert_eq!(find_pattern((7, 19), 70147, 166439), (1068781, 3162341));
    }

    #[test]
    fn part_1_example() {
        let input = "7,13,x,x,59,x,31,19"
            .split(',')
            .map(|n| n.parse().ok())
            .collect();
        let res = Day13::part_1(&(939, input));
        assert_eq!(res, 295);
    }

    #[test]
    fn part_2_example() {
        let input = "7,13,x,x,59,x,31,19"
            .split(',')
            .map(|n| n.parse().ok())
            .collect();
        let res = Day13::part_2(&(0, input));
        assert_eq!(res, 1068781);
    }
}
