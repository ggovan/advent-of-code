use crate::aoc_2020::Aoc2020;
use crate::files::Res;
use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

pub struct Day22;

impl Aoc2020 for Day22 {
    type Input = (VecDeque<u8>, VecDeque<u8>);
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        22
    }
    fn load() -> Res<Self::Input> {
        let input = read_to_string("data/2020/day_22.in")?;
        let (player, crab) = input.split_once("\n\n").unwrap();
        Ok((read_hand(player), read_hand(crab)))
    }

    fn part_1((player, crab): &Self::Input) -> Self::Result1 {
        let mut player = player.clone();
        let mut crab = crab.clone();

        while !player.is_empty() && !crab.is_empty() {
            let p = player.pop_front().unwrap();
            let c = crab.pop_front().unwrap();
            if p > c {
                player.push_back(p);
                player.push_back(c);
            } else {
                if c > p {
                    crab.push_back(c);
                    crab.push_back(p);
                }
            }
        }

        let winner = if !player.is_empty() { player } else { crab };
        winner
            .iter()
            .enumerate()
            .map(|(i, v)| usize::from(*v) * (winner.len() - i))
            .sum()
    }

    fn part_2((player, crab): &Self::Input) -> Self::Result2 {
        let (_, winner) = game((player.clone(), crab.clone()));

        winner
            .iter()
            .enumerate()
            .map(|(i, v)| usize::from(*v) * (winner.len() - i))
            .sum()
    }
}

fn game((mut player, mut crab): (VecDeque<u8>, VecDeque<u8>)) -> (bool, VecDeque<u8>) {
    let mut previous_states: HashSet<(VecDeque<u8>, VecDeque<u8>)> = HashSet::new();
    let mut winner = None;

    if player.iter().max().unwrap() > crab.iter().max().unwrap() {
        // if the player has the highest card then they will win.
        return (true, player);
    }

    while !player.is_empty() && !crab.is_empty() {
        let cloned = (player.clone(), crab.clone());
        if !previous_states.insert(cloned) {
            winner = Some(true);
            break;
        }

        let p = player.pop_front().unwrap();
        let c = crab.pop_front().unwrap();

        if usize::from(p) <= player.len() && usize::from(c) <= crab.len() {
            let winner = game((
                player.iter().take(p as usize).cloned().collect(),
                crab.iter().take(c as usize).cloned().collect(),
            ))
            .0;
            if winner {
                player.push_back(p);
                player.push_back(c);
            } else {
                crab.push_back(c);
                crab.push_back(p);
            }
        } else if p > c {
            player.push_back(p);
            player.push_back(c);
        } else {
            crab.push_back(c);
            crab.push_back(p);
        }
    }

    let (winner, hand) = if winner.is_some() || crab.is_empty() {
        (true, player)
    } else {
        (false, crab)
    };

    return (winner, hand);
}

fn read_hand(s: &str) -> VecDeque<u8> {
    s.lines()
        .skip(1)
        .map(|s| s.parse::<u8>().unwrap())
        .collect()
}
