use crate::aoc_2020::Aoc2020;
use crate::common::bitset::Bitset;
use crate::common::geometry::{Direction, Point2D};
use crate::files::Res;
use std::cmp::{Eq, Ordering, PartialEq};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

pub struct Day18;

impl Aoc2020 for Day18 {
    type Input = Vec<Vec<char>>;
    type Result1 = u64;
    type Result2 = u64;

    fn day() -> usize {
        18
    }
    fn load() -> Res<Self::Input> {
        let input = read_to_string("data/2019/day_18.in")?;
        Ok(input.lines().map(|l| l.chars().collect()).collect())
    }

    fn part_1(map: &Self::Input) -> Self::Result1 {
        let mut keys_required: HashMap<char, Bitset> = HashMap::new();
        let mut distance_cache: HashMap<(char, char), i64> = HashMap::new();
        let keys: HashSet<char> = map
            .iter()
            .flat_map(|r| r.iter().filter(|c| ('a'..='z').contains(*c)))
            .cloned()
            .collect();

        let starting_position = map
            .iter()
            .enumerate()
            .filter_map(|(y, r)| {
                r.iter()
                    .enumerate()
                    .find(|(_, c)| **c == '@')
                    .map(|(x, _)| Point2D(x as i64, y as i64))
            })
            .next()
            .unwrap();

        bfs(
            &mut keys_required,
            &mut distance_cache,
            map,
            starting_position,
            '@',
            true,
        );

        for k in keys.iter() {
            let starting_position = map
                .iter()
                .enumerate()
                .filter_map(|(y, r)| {
                    r.iter()
                        .enumerate()
                        .find(|(_, c)| **c == *k)
                        .map(|(x, _)| Point2D(x as i64, y as i64))
                })
                .next()
                .unwrap();

            bfs(
                &mut keys_required,
                &mut distance_cache,
                map,
                starting_position,
                *k,
                false,
            );
        }

        search(&keys_required, &distance_cache, '@', &keys)
    }

    fn part_2(_input: &Self::Input) -> Self::Result2 {
        18
    }
}

/// Do a breadth first search to find the keys required to reach each key/door.
/// Also calc how long it takes to get from the start to each point of interest.
fn bfs(
    keys_required: &mut HashMap<char, Bitset>,
    distance_cache: &mut HashMap<(char, char), i64>,
    map: &[Vec<char>],
    pos: Point2D<i64>,
    start_symbol: char,
    fill_keys: bool,
) -> () {
    let mut queue: VecDeque<(Point2D<i64>, Bitset, usize)> = VecDeque::new();
    queue.push_back((pos, Bitset::empty(), 0));
    let mut visited: HashSet<Point2D<i64>> = HashSet::new();

    while let Some((pos, keys, steps)) = queue.pop_front() {
        // dbg!(&queue);
        if visited.contains(&pos)
            || !(0..=map[0].len() as i64).contains(&pos.0)
            || !(0..=map.len() as i64).contains(&pos.1)
            || map[pos.1 as usize][pos.0 as usize] == '#'
        {
            continue;
        }
        visited.insert(pos);

        let symbol = map[pos.1 as usize][pos.0 as usize];

        let new_keys = if ('A'..='Z').contains(&symbol) && fill_keys {
            keys.set(((symbol as u8 + 32 as u8) - 'a' as u8).into())
        } else {
            keys
        };
        if ('a'..='z').contains(&symbol) {
            if fill_keys {
                keys_required.insert(symbol, new_keys);
            }
            distance_cache.insert((start_symbol, symbol), steps as i64);
            // we also know what doors we've passed through
            // could we add some distance cache entries here?
        }

        for d in Direction::array() {
            let pos = d.next_point(pos);
            queue.push_back((pos, new_keys, steps + 1))
        }
    }
}

// It's a graph where the nodes are {keys}.
// We want to get a path from {} to {ALL}
// But actually, it's ({keys}, location)
// Can we do an A*?
// I don't think that there's an admissible heuristic,
// - actually, just take the number of things still to visit
// - maybe the shortest distance between any things, or the longest of the shortest for each key
// So just do djikstras?

// ({A,B,C}, C) and ({C,B,A}, C) are the same
// ({A,B,C,D}, C) and ({C,B,A}, C) are separate
//

// There is an edge (C->D)

/// A point with a distance, and heuristic for use in A*
#[derive(PartialEq, Copy, Clone, Eq, std::fmt::Debug)]
struct HeapElem {
    point: char,
    keys: Bitset,
    distance: u64,
    heuristic: u64,
}

fn create_he(point: char, keys: Bitset, distance: u64, key_count: u64) -> HeapElem {
    // use the manhattan distance as a heuristic
    let heuristic = key_count - keys.count();
    HeapElem {
        point,
        keys,
        distance,
        heuristic,
    }
}

impl Ord for HeapElem {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.distance + self.heuristic)
            .cmp(&(other.distance + other.heuristic))
            .reverse()
    }
}

impl PartialOrd for HeapElem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn search(
    required_keys: &HashMap<char, Bitset>,
    distance_cache: &HashMap<(char, char), i64>,
    start: char,
    all_keys: &HashSet<char>,
) -> u64 {
    let key_count = all_keys.len() as u64;
    let mut queue: BinaryHeap<HeapElem> = BinaryHeap::new();
    let mut visited: HashSet<(char, Bitset)> = HashSet::new();
    queue.push(create_he(start, Bitset::empty(), 0, key_count));

    while queue.peek().unwrap().keys.count() != key_count {
        let HeapElem {
            point,
            keys,
            distance,
            heuristic: _,
        } = queue.pop().unwrap();

        if visited.contains(&(point, keys)) {
            continue;
        }

        for &k in all_keys.iter() {
            if keys.contains((k as u8 - 'a' as u8).into()) {
                // visited in this route
                continue;
            }
            if !keys.contains_all(required_keys[&k]) {
                continue;
            }
            queue.push(create_he(
                k,
                keys.set((k as u8 - 'a' as u8).into()),
                distance + distance_cache[&(point, k)] as u64,
                key_count,
            ));
        }

        visited.insert((point, keys));
    }

    queue.peek().unwrap().distance
}
