use crate::aoc_2020::Aoc2020;
use crate::files::Res;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub struct Day23;

impl Aoc2020 for Day23 {
    type Input = Vec<usize>;
    type Result1 = String;
    type Result2 = u64;

    fn day() -> usize {
        23
    }
    fn load() -> Res<Self::Input> {
        // Ok(vec![3, 8, 9, 1, 2, 5, 4, 6, 7])
        Ok(vec![6, 8, 5, 9, 7, 4, 2, 1, 3])
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        let mut cups = input.clone();

        for _ in 0..100 {
            println!("({}) {:?}", cups[0], &cups[1..]);
            let current_value = cups[0];
            let next_cups = [cups[1], cups[2], cups[3]];

            let next = cups
                .iter()
                .enumerate()
                .skip(4)
                .filter(|(_, v)| **v < current_value)
                .map(|(i, v)| (i, *v))
                .max_by_key(|(_, v)| *v);
            // wrap around
            let next = next.unwrap_or_else(|| {
                cups.iter()
                    .enumerate()
                    .skip(4)
                    .map(|(i, v)| (i, *v))
                    .max_by_key(|(_, v)| *v)
                    .unwrap()
            });

            (&mut cups[0..=next.0]).rotate_left(4);
            cups[next.0 - 3] = next_cups[0];
            cups[next.0 - 2] = next_cups[1];
            cups[next.0 - 1] = next_cups[2];
            (&mut cups[next.0..]).rotate_left(1);
            let len = cups.len();
            cups[len - 1] = current_value;
        }
        println!("({}) {:?}", cups[0], &cups[1..]);
        let (one_index, _) = cups.iter().enumerate().find(|(i, v)| **v == 1).unwrap();
        cups.rotate_left(one_index);
        cups.iter().skip(1).map(|v| v.to_string()).collect()
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        let mut succs = Vec::with_capacity(1_000_000);
        succs.resize(9, 0);

        let mut prev = 1;
        for i in 0..9 {
            let cup = input[i];
            succs[prev - 1] = cup;
            prev = cup;
        }

        succs.resize(1_000_000, 0);
        for i in 10..=1_000_000 {
            succs[prev - 1] = i;
            prev = i;
        }

        // let len = succs.len();
        succs[prev - 1] = input[0];

        // println!("{:?}", succs);

        let mut current_val = input[0];

        for _ in 0..10_000_000 {
            let first_to_move = succs[current_val - 1];
            // println!("{} {}", current_val, first_to_move);
            let second_to_move = succs[first_to_move - 1];
            let last_to_move = succs[second_to_move - 1];
            let after_moved = succs[last_to_move - 1];

            let invalid = [current_val, first_to_move, second_to_move, last_to_move];

            let mut destination = current_val;
            while invalid.contains(&destination) {
                destination = if destination > 1 {
                    destination - 1
                } else {
                    succs.len()
                };
            }

            let after_destination = succs[destination - 1];

            succs[current_val - 1] = after_moved;
            succs[destination - 1] = first_to_move;
            succs[last_to_move - 1] = after_destination;
            current_val = after_moved;
        }

        let second = succs[0];
        let third = succs[second - 1];

        dbg!(second) as u64 * dbg!(third) as u64
    }
}
