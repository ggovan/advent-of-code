use crate::aoc_2020::Aoc2020;
use crate::common::bitset::Bitset;
use crate::common::geometry::{Direction, Point2D};
use crate::files::Res;
use std::cmp::{Eq, Ordering, PartialEq};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

pub struct Day18;

impl Aoc2020 for Day18 {
    type Input = Vec<Vec<u8>>;
    type Result1 = u64;
    type Result2 = u64;

    fn day() -> usize {
        18
    }

    fn load() -> Res<Self::Input> {
        let input = read_to_string("data/2019/day_18.in")?;
        Ok(input.lines().map(|l| l.bytes().collect()).collect())
    }

    fn part_1(map: &Self::Input) -> Self::Result1 {
        let mut keys_required: HashMap<u8, Bitset> = HashMap::new();
        let mut distance_cache: HashMap<(u8, u8), i64> = HashMap::new();
        let keys: HashSet<u8> = map
            .iter()
            .flat_map(|r| r.iter().filter(|c| (b'a'..=b'z').contains(*c)))
            .cloned()
            .collect();

        let starting_position = map
            .iter()
            .enumerate()
            .filter_map(|(y, r)| {
                r.iter()
                    .enumerate()
                    .find(|(_, c)| **c == b'@')
                    .map(|(x, _)| Point2D(x as i64, y as i64))
            })
            .next()
            .unwrap();

        bfs(
            &mut keys_required,
            &mut distance_cache,
            map,
            starting_position,
            b'@',
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

        search(&keys_required, &distance_cache, b'@', &keys)
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        let mut map: Vec<Vec<u8>> = input.clone();
        let mut keys_required: HashMap<u8, Bitset> = HashMap::new();
        let mut distance_cache: HashMap<(u8, u8), i64> = HashMap::new();
        let keys: HashSet<u8> = map
            .iter()
            .flat_map(|r| r.iter().filter(|c| (b'a'..=b'z').contains(*c)))
            .cloned()
            .collect();

        let orig_starting_position = map
            .iter()
            .enumerate()
            .filter_map(|(y, r)| {
                r.iter()
                    .enumerate()
                    .find(|(_, c)| **c == b'@')
                    .map(|(x, _)| Point2D(x as i64, y as i64))
            })
            .next()
            .unwrap();

        {
            let Point2D(x, y) = orig_starting_position;
            let x = x as usize;
            let y = y as usize;
            map[y - 1][x - 1] = b'@';
            map[y - 1][x + 1] = b'@' - 1;
            map[y + 1][x - 1] = b'@' - 2;
            map[y + 1][x + 1] = b'@' - 3;
            map[y][x - 1] = b'#';
            map[y][x + 1] = b'#';
            map[y - 1][x] = b'#';
            map[y + 1][x] = b'#';
        }
        for i in 0..4 {
            let s_sym = b'@' - i;
            let starting_position = map
                .iter()
                .enumerate()
                .filter_map(|(y, r)| {
                    r.iter()
                        .enumerate()
                        .find(|(_, c)| **c == s_sym)
                        .map(|(x, _)| Point2D(x as i64, y as i64))
                })
                .next()
                .unwrap();
            bfs(
                &mut keys_required,
                &mut distance_cache,
                &map,
                starting_position,
                s_sym,
                true,
            );
        }

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
                &map,
                starting_position,
                *k,
                false,
            );
        }

        search_4(
            &keys_required,
            &distance_cache,
            [b'@', b'?', b'>', b'='],
            &keys,
        )
    }
}

/// Do a breadth first search to find the keys required to reach each key/door.
/// Also calc how long it takes to get from the start to each point of interest.
fn bfs(
    keys_required: &mut HashMap<u8, Bitset>,
    distance_cache: &mut HashMap<(u8, u8), i64>,
    map: &[Vec<u8>],
    pos: Point2D<i64>,
    start_symbol: u8,
    fill_keys: bool,
) {
    let mut queue: VecDeque<(Point2D<i64>, Bitset, usize)> = VecDeque::new();
    queue.push_back((pos, Bitset::empty(), 0));
    let mut visited: HashSet<Point2D<i64>> = HashSet::new();

    while let Some((pos, keys, steps)) = queue.pop_front() {
        // dbg!(&queue);
        if visited.contains(&pos)
            || !(0..=map[0].len() as i64).contains(&pos.0)
            || !(0..=map.len() as i64).contains(&pos.1)
            || map[pos.1 as usize][pos.0 as usize] == b'#'
        {
            continue;
        }
        visited.insert(pos);

        let symbol = map[pos.1 as usize][pos.0 as usize];

        let new_keys = if (b'A'..=b'Z').contains(&symbol) && fill_keys {
            keys.set(((symbol + 32) - b'a').into())
        } else {
            keys
        };
        if (b'a'..=b'z').contains(&symbol) {
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
    point: u8,
    keys: Bitset,
    distance: u64,
    heuristic: u64,
}

fn create_he(point: u8, keys: Bitset, distance: u64, key_count: u64) -> HeapElem {
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
    required_keys: &HashMap<u8, Bitset>,
    distance_cache: &HashMap<(u8, u8), i64>,
    start: u8,
    all_keys: &HashSet<u8>,
) -> u64 {
    let key_count = all_keys.len() as u64;
    let mut queue: BinaryHeap<HeapElem> = BinaryHeap::new();
    let mut visited: HashSet<(u8, Bitset)> = HashSet::new();
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
            if keys.contains((k - b'a').into()) {
                // visited in this route
                continue;
            }
            if !keys.contains_all(required_keys[&k]) {
                continue;
            }
            queue.push(create_he(
                k,
                keys.set((k - b'a').into()),
                distance + distance_cache[&(point, k)] as u64,
                key_count,
            ));
        }

        visited.insert((point, keys));
    }

    queue.peek().unwrap().distance
}

type Arr4 = [u8; 4];

#[derive(PartialEq, Copy, Clone, Eq, std::fmt::Debug)]
struct HeapElem4 {
    point: Arr4,
    keys: Bitset,
    distance: u64,
    heuristic: u64,
}

fn create_he4(point: Arr4, keys: Bitset, distance: u64, key_count: u64) -> HeapElem4 {
    // use the manhattan distance as a heuristic
    let heuristic = key_count - keys.count();
    HeapElem4 {
        point,
        keys,
        distance,
        heuristic,
    }
}

impl Ord for HeapElem4 {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.distance + self.heuristic)
            .cmp(&(other.distance + other.heuristic))
            .reverse()
    }
}

impl PartialOrd for HeapElem4 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn search_4(
    required_keys: &HashMap<u8, Bitset>,
    distance_cache: &HashMap<(u8, u8), i64>,
    start: Arr4,
    all_keys: &HashSet<u8>,
) -> u64 {
    let key_count = all_keys.len() as u64;
    let mut queue: BinaryHeap<HeapElem4> = BinaryHeap::new();
    let mut visited: HashSet<(Arr4, Bitset)> = HashSet::new();
    queue.push(create_he4(start, Bitset::empty(), 0, key_count));

    while queue.peek().unwrap().keys.count() != key_count {
        let HeapElem4 {
            point,
            keys,
            distance,
            heuristic: _,
        } = queue.pop().unwrap();

        if visited.contains(&(point, keys)) {
            continue;
        }

        for &k in all_keys.iter() {
            if keys.contains((k - b'a').into()) {
                // visited in this route
                continue;
            }
            if !keys.contains_all(required_keys[&k]) {
                continue;
            }
            let (i, d) = point
                .iter()
                .enumerate()
                .filter_map(|(i, s)| distance_cache.get(&(*s, k)).map(|d| (i, d)))
                .next()
                .unwrap();
            let mut new_point = point;
            new_point[i] = k;
            queue.push(create_he4(
                new_point,
                keys.set((k - b'a').into()),
                distance + *d as u64,
                key_count,
            ));
        }

        visited.insert((point, keys));
    }

    queue.peek().unwrap().distance
}
