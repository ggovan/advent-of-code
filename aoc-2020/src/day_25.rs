use aoc_common::aoc_day::AocDay;
use aoc_common::files::Res;
use std::fs::read_to_string;

pub struct Day25;

impl AocDay for Day25 {
    type Input = (u64, u64);
    type Result1 = u64;
    type Result2 = u64;

    fn day() -> usize {
        25
    }
    fn load() -> Res<Self::Input> {
        let input = read_to_string("data/2020/day_25.in")?;
        let (left, right) = input.split_once('\n').unwrap();
        Ok((left.parse().unwrap(), right.trim().parse().unwrap()))
    }

    fn part_1(&(card_pub, door_pub): &Self::Input) -> Self::Result1 {
        let card_loop = inverse_transform(7, card_pub);
        transform(door_pub, card_loop)
    }

    fn part_2(_input: &Self::Input) -> Self::Result2 {
        25
    }
}

fn inverse_transform(subject: u64, public_key: u64) -> u64 {
    let mut value = 1;
    let mut loop_size = 0;
    while value != public_key {
        value = value * subject % 20201227;
        loop_size += 1;
    }
    loop_size
}

fn transform(subject: u64, mut loop_size: u64) -> u64 {
    let mut value = 1;
    while loop_size > 0 {
        value = value * subject % 20201227;
        loop_size -= 1;
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_example() {
        assert_eq!(transform(7, 8), 5764801);
    }

    #[test]
    fn test_transform_example_door() {
        assert_eq!(transform(7, 11), 17807724);
    }

    #[test]
    fn test_inverse_transform_example() {
        assert_eq!(inverse_transform(7, 5764801), 8);
    }

    #[test]
    fn test_inverse_transform_example_door() {
        assert_eq!(inverse_transform(7, 17807724), 11);
    }

    #[test]
    fn test_encrypt() {
        let card_pub = transform(7, 8);
        let door_pub = transform(7, 11);
        let encryption_key = transform(card_pub, 11);
        assert_eq!(encryption_key, 14897079);
        assert_eq!(transform(door_pub, 8), encryption_key);
    }
}
