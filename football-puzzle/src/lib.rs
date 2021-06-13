use aoc_common::files::Res;
use aoc_common::{time, time_block};

use std::collections::{HashMap, VecDeque};

pub fn run_all() -> Res<()> {
    // let input = vec![2,3,5, 7, 11, 13, 17, 19, 23, 29, 31];
    // these _must_ be coprime
    let input = vec![7, 17, 13, 10]; //, 19, 23, 29, 31, 33, 37];
    let input = vec![17, 13, 10, 19, 23, 29, 31, 33, 37]; // 7,
    let input = vec![37, 33, 31, 29, 23, 19, 17, 13, 10, 7, 101];
    // let input = vec![37, 33, 31, 19, 23, 29, 17, 13, 10, 7];

    let input = [101, 17, 31, 7, 13, 37, 33, 23, 10, 19, 29];

    // let input = [3, 5];
    let speed = 10;

    let _total_time = time_block("Part 6: solve within");

    let (s, t): (_, _) = time(|| part_1(&input));
    println!("Part 1 - {} in {:?}", s, t);

    let (s, t): (_, _) = time(|| part_2(&input, speed));
    println!("Part 2 - {} in {:?}", s, t);

    let mut with_klinski = input.clone();
    with_klinski[0] = 7919;
    let (s, t): (_, _) = time(|| part_2(&with_klinski, speed));
    println!("Part 3 - {} in {:?}", s, t);

    let (s, t): (_, _) = time(|| part_4(&input, speed));
    println!("Part 4 - {} in {:?}", s, t);

    let (s, t): (_, _) = time(|| part_5(&input));
    println!("Part 5 - {:?} in {:?}", s, t);

    // println!("Total time: {:?}", t);

    Ok(())
}

fn part_1(input: &[i64]) -> i64 {
    let mut known_primes = vec![2, 3];

    let all_prime_factors = input
        .iter()
        .map(|x| get_prime_factors(*x, &mut known_primes))
        .collect::<Vec<_>>();

    let mut product = 1;
    for p in known_primes.iter() {
        let max_occur = all_prime_factors
            .iter()
            .map(|pfs| pfs.get(p).unwrap_or(&0))
            .max()
            .unwrap_or(&0);
        product *= p.pow(*max_occur as u32);
    }

    product
}

fn get_prime_factors(mut n: i64, known_primes: &mut Vec<i64>) -> HashMap<i64, i64> {
    let mut prime_factors: HashMap<i64, i64> = HashMap::new();
    let mut d = 2;
    loop {
        if n == 1 {
            return prime_factors;
        } else if n % d == 0 {
            *prime_factors.entry(d).or_insert(0) += 1;
            n = n / d
        } else {
            d = get_next_prime(d, known_primes)
        }
    }
}

fn get_next_prime(mut p: i64, known_primes: &mut Vec<i64>) -> i64 {
    for &known_p in known_primes.iter() {
        if known_p > p {
            return known_p;
        }
    }

    loop {
        p += 2;
        if known_primes.iter().all(|x| p % x != 0) {
            known_primes.push(p);
            return p;
        }
    }
}

/// Fold over all the players, and at each step find an `offset` and `period` that can be used to find occurrences for the matching pattern.
/// i.e. for players 7,13:
///   for 7 the offset is 0 (the pattern starts from position 0), and the period is 7 (when this player next blinks)
///   for 13 the offset is 42 (the first time we see a pattern that we accept) and the period is 91 (the number of steps until it appears again).
/// By always jumping by the period, we quickly find a start value which produces a valid pattern.
fn part_2(players: &[i64], interval: i64) -> i64 {
    players
        .iter()
        .enumerate()
        // .filter_map(|(i, b)| b.map(|b| (i, b)))
        .fold((1, 1), |(offset, period), (player_index, player_speed)| {
            // dbg!((offset * interval, period, player_index, player_speed));
            find_pattern((player_index, *player_speed), offset, period, interval)
        })
        .0
}

fn find_pattern(
    (player_index, player_speed): (usize, i64),
    offset: i64,
    period: i64,
    running_interval: i64,
) -> (i64, i64) {
    // Keep iterating by the period until we find the new offset
    let mut new_offset = offset;
    loop {
        if (new_offset + (player_index as i64) * running_interval) % player_speed == 0 {
            break;
        }
        new_offset += period;
    }

    // The new period will be a multiple of the old period.
    // Keep looping until you find it.
    // dbg!("get period");
    let mut new_period = period;
    loop {
        if (new_offset + new_period + (player_index as i64) * running_interval) % player_speed == 0
        {
            break;
        }
        new_period += period;
    }

    (new_offset, new_period)
}

fn part_4(players: &[i64], running_interval: i64) -> i64 {
    // The current permuted list of players
    let mut permut_store: VecDeque<usize> = VecDeque::new();
    // A stack of (offset, period) - could be an array
    let mut score_stack: VecDeque<(i64, i64)> = VecDeque::new();

    // the team order and score
    let mut min: Option<(VecDeque<usize>, i64)> = None;

    dive(
        players,
        &mut permut_store,
        &mut score_stack,
        &mut min,
        running_interval,
    );

    let min = min.unwrap();

    dbg!(min.0.iter().map(|p| players[*p]).collect::<Vec<_>>());
    min.1
}

fn dive(
    players: &[i64],
    permut_store: &mut VecDeque<usize>,
    score_stack: &mut VecDeque<(i64, i64)>,
    min: &mut Option<(VecDeque<usize>, i64)>,
    running_interval: i64,
) {
    if permut_store.len() == players.len() {
        match min {
            Some((_, v)) if *v <= score_stack.back().unwrap().0 => (),
            _ => {
                println!("{:?} - {:?}", permut_store, score_stack.back().unwrap().0);
                min.replace((permut_store.clone(), score_stack.back().unwrap().0));
                ()
            }
        }
    } else if score_stack
        .back()
        .map(|(o, _)| o)
        .zip(min.as_ref().map(|(_, s)| s))
        .map(|(o, s)| *o >= *s)
        .unwrap_or(false)
    {
        return;
    } else {
        for p in 0..players.len() {
            if permut_store.iter().any(|p2| *p2 == p) {
                continue;
            } else {
                let &(p_offset, p_period) = score_stack.back().unwrap_or(&(1, 1));

                permut_store.push_back(p);
                score_stack.push_back(find_pattern(
                    (permut_store.len() - 1, players[p]),
                    p_offset,
                    p_period,
                    running_interval,
                ));

                // dbg!(&permut_store);
                // dbg!(&score_stack);

                dive(players, permut_store, score_stack, min, running_interval);

                score_stack.pop_back();
                permut_store.pop_back();
            }
        }
    }
}

fn part_5(players: &[i64]) -> (i64, i64) {
    (0..1000)
        .map(|speed| (speed, part_2(players, speed)))
        .min_by_key(|(_, t)| *t)
        .unwrap()
}
