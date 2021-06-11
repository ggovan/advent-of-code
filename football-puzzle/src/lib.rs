use aoc_common::files::Res;
use aoc_common::{time, time_block};

use std::collections::HashMap;

pub fn run_all() -> Res<()> {
    let input = vec![2, 4, 5, 7, 11, 13, 17, 19, 23, 29, 31];

    let _total_time = time_block("Part 6: solve within");

    let (s, t): (_, _) = time(|| part1(&input));
    println!("Part 1 - {} in {:?}", s, t);

    // println!("Total time: {:?}", t);

    Ok(())
}

fn part1(input: &[i64]) -> i64 {
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

// 200560490130
// 534827973680
// 133706993420

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
