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
        let mut cups = Vec::with_capacity(1_000_000);
        cups.resize_with(9, || Rc::new(Cup::new(0)));

        let mut first_cup = Rc::new(Cup::new(input[0]));
        let mut prev = first_cup.clone();
        cups[input[0] - 1] = first_cup.clone();

        for i in 1..9 {
            let cup = Rc::new(Cup::new(input[i]));
            cups[input[i] - 1] = cup.clone();
            prev.set_next(Some(cup.clone()));
            prev = cup;
        }

        for i in 10..=1_000_000 {
            let cup = Rc::new(Cup::new(i));
            cups.push(cup.clone());
            prev.set_next(Some(cup.clone()));
            prev = cup;
        }

        // let last_cup = cups.last().unwrap();
        // last_cup.set_next(Some(first_cup.clone()));
        prev.set_next(Some(first_cup.clone()));
        println!("{}", prev.value);

        for c in cups.iter().take(9) {
            println!("{}", c.value);
        }

        let mut current_val = first_cup.value;

        let mut p_node = first_cup.clone();
        for _ in 0..20 {
            print!("{} ", p_node.value);
            p_node = p_node.get_next();
        }
        println!();

        for _ in 0..10_000_000 {
            //     // println!("({}) {:?}", cups[0], &cups[1..]);
            let current_node = &cups[current_val - 1];
            assert_eq!(current_val, current_node.value);
            let mut p_node = current_node.clone();
            // for _ in 0..20 {
            //     print!("{} ", p_node.value);
            //     p_node = p_node.get_next();
            // }
            // println!();

            let current_node = &cups[current_val - 1];
            // dbg!(current_node.value);
            let first_to_move = current_node.get_next();
            let second_to_move = first_to_move.get_next();
            let last_to_move = second_to_move.get_next();
            let after_moved = last_to_move.get_next();

            let invalid = [
                current_val,
                first_to_move.value,
                second_to_move.value,
                last_to_move.value,
            ];

            let mut destination = current_val;
            while invalid.contains(&destination) {
                destination = if destination > 1 {
                    destination - 1
                } else {
                    cups.len()
                };
            }

            let destination_node = &cups[destination - 1];
            let after_destination = destination_node.get_next();

            current_val = after_moved.value;
            current_node.set_next(Some(after_moved));
            destination_node.set_next(Some(first_to_move));
            last_to_move.set_next(Some(after_destination));
        }

        let second = cups[0].get_next();
        let third = second.get_next();
        assert_eq!(cups[0].value, 1);
        let mut p_node = cups[current_val - 1].clone();
        for _ in 0..10 {
            print!("{} ", p_node.value);
            p_node = p_node.get_next();
        }
        println!();
        dbg!(second.value) as u64 * dbg!(third.value) as u64
    }
}

struct Cup {
    value: usize,
    link: RefCell<Link>,
}

struct Link {
    next: Option<Rc<Cup>>,
}

impl Cup {
    fn new(value: usize) -> Self {
        Self {
            value,
            link: RefCell::new(Link { next: None }),
        }
    }
    fn set_next(&self, cup: Option<Rc<Cup>>) {
        self.link.borrow_mut().next = cup;
    }
    fn get_next(&self) -> Rc<Cup> {
        self.link.borrow().next.clone().unwrap().clone()
    }
    fn has_next(&self) -> bool {
        self.link.borrow().next.is_some()
    }
}
