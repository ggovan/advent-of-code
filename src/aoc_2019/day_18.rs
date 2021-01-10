use crate::aoc_2020::Aoc2020;
use crate::common::bitset::Bitset;
use crate::common::geometry::{Direction, Point2D};
use crate::common::search::{search, HeapElem as ExHeapElem};
use crate::files::Res;
use std::collections::{HashMap, HashSet, VecDeque};
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

        let key_count = keys.len() as u64;

        let dc = &distance_cache;
        let kr = &keys_required;

        let start: (Bitset, u8) = (Bitset::empty(), b'@');
        search(
            start,
            |(collected, _)| collected.count() == key_count,
            |(collected, point), distance| {
                keys.iter().filter_map(move |&k| {
                    if collected.contains((k - b'a').into()) {
                        // visited in this route
                        return None;
                    }
                    if !collected.contains_all(kr[&k]) {
                        return None;
                    }
                    Some(ExHeapElem {
                        elem: (collected.set((k - b'a').into()), k),
                        distance: distance + dc[&(point, k)] as u64,
                        heuristic: key_count - collected.count(),
                    })
                })
            },
        )
        .1
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

        let key_count = keys.len() as u64;

        let dc = &distance_cache;
        let kr = &keys_required;

        let start: (Bitset, [u8; 4]) = (Bitset::empty(), [b'@', b'?', b'>', b'=']);
        search(
            start,
            |(collected, _)| collected.count() == key_count,
            |(collected, points), distance| {
                keys.iter().filter_map(move |&k| {
                    if collected.contains((k - b'a').into()) {
                        // visited in this route
                        return None;
                    }
                    if !collected.contains_all(kr[&k]) {
                        return None;
                    }
                    let (i, d) = points
                        .iter()
                        .enumerate()
                        .filter_map(|(i, s)| dc.get(&(*s, k)).map(|d| (i, d)))
                        .next()
                        .unwrap();
                    #[allow(clippy::clone_on_copy)]
                    let mut new_point = points.clone();
                    new_point[i] = k;
                    Some(ExHeapElem {
                        elem: (collected.set((k - b'a').into()), new_point),
                        distance: distance + *d as u64,
                        heuristic: key_count - collected.count(),
                    })
                })
            },
        )
        .1
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
