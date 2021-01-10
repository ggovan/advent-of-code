use crate::aoc_2020::Aoc2020;
use crate::common::geometry::Direction;
use crate::common::search::{search, HeapElem};
use crate::files::Res;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

pub struct Day20;

impl Aoc2020 for Day20 {
    type Input = Vec<Vec<MapElem>>;
    type Result1 = u64;
    type Result2 = u64;

    fn day() -> usize {
        20
    }
    fn load() -> Res<Self::Input> {
        let input = read_to_string("data/2019/day_20.in")?;
        Ok(input
            .lines()
            .map(|l| {
                l.bytes()
                    .map(|b| match b {
                        b' ' => MapElem::Space,
                        b'.' => MapElem::Path,
                        b'#' => MapElem::Wall,
                        x if (b'A'..=b'Z').contains(&x) => MapElem::PortalPart(x),
                        x => unreachable!("Unexpected symbol in input: {:?}", x),
                    })
                    .collect()
            })
            .collect())
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        let mut input = input.to_owned();
        setup_portals(&mut input);
        let map = &input;
        let graph = setup_graph(map);

        search(
            Portal('A', 'A', false),
            |p| matches!(p, Portal('Z', 'Z', _)),
            |portal, distance| {
                graph[&portal].iter().filter_map(move |(p, d)| {
                    if p.0 == 'A' && p.1 == 'A' {
                        None
                    } else {
                        Some(HeapElem {
                            elem: Portal(p.0, p.1, !p.2),
                            distance: *d as u64 + distance + 1,
                            heuristic: 0,
                        })
                    }
                })
            },
        )
        .1 - 1
    }

    fn part_2(_input: &Self::Input) -> Self::Result2 {
        20
    }
}

struct BfsIterator<'a, T> {
    visited: HashSet<T>,
    queue: VecDeque<T>,
    map: &'a [Vec<MapElem>],
}

fn setup_graph(map: &[Vec<MapElem>]) -> HashMap<Portal, Vec<(Portal, i64)>> {
    let mut graph = HashMap::new();

    let mut queue: VecDeque<(i64, i64, i64)> = VecDeque::new();
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    for (y, row) in map.iter().enumerate() {
        for (x, p) in row.iter().enumerate() {
            if let MapElem::Portal(start) = p {
                queue.clear();
                visited.clear();

                let mut successors: Vec<(Portal, i64)> = Vec::new();
                // TODO, start distance is -2
                queue.push_back((x as i64, y as i64, -2));
                // visited.insert((x as i64, y as i64));

                while let Some((x, y, dist)) = queue.pop_front() {
                    if visited.contains(&(x, y)) {
                        continue;
                    }
                    let (x, y) = if (0..(row.len() as i64)).contains(&x)
                        && (0..(map.len() as i64)).contains(&y)
                    {
                        (x, y)
                    } else {
                        continue;
                    };
                    visited.insert((x as i64, y as i64));
                    if let Some(MapElem::Portal(succ)) =
                        map.get(y as usize).and_then(|r| r.get(x as usize))
                    {
                        if succ != start {
                            successors.push((*succ, dist));
                            continue;
                        }
                        for dir in Direction::array() {
                            let (x, y) = dir.next_point((x, y));
                            queue.push_back((x, y, dist + 1));
                        }
                    }
                    if let Some(MapElem::Path) = map.get(y as usize).and_then(|r| r.get(x as usize))
                    {
                        for dir in Direction::array() {
                            let (x, y) = dir.next_point((x, y));
                            queue.push_back((x, y, dist + 1));
                        }
                    }
                }

                graph.insert(*start, successors);
            }
        }
    }

    // graph.iter().for_each(|(k, v)| println!("{:?}: {:?}", k, v));
    graph
}

#[derive(std::fmt::Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum MapElem {
    Space,
    Path,
    Wall,
    PortalPart(u8),
    Portal(Portal),
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Portal(char, char, bool);

impl std::fmt::Debug for Portal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Portal")
            .field(&format!("{}{}", self.0, self.1))
            .field(&self.2)
            .finish()
    }
}

impl MapElem {
    fn create_portal(p1: u8, p2: u8, portals: &HashSet<Portal>) -> Portal {
        let p1 = p1 as char;
        let p2 = p2 as char;
        if p1 < p2 {
            Portal(p1, p2, portals.contains(&Portal(p1, p2, false)))
        } else {
            Portal(p2, p1, portals.contains(&Portal(p2, p1, false)))
        }
    }
}

fn setup_portals(map: &mut [Vec<MapElem>]) {
    let row_length = map[0].len();
    let mut portals = HashSet::new();
    for y in 0..map.len() {
        for x in 0..row_length {
            match map[y][x] {
                MapElem::PortalPart(p1) => {
                    if let Some(MapElem::PortalPart(p2)) = map[y].get(x + 1) {
                        let new_portal = MapElem::create_portal(p1, *p2, &portals);
                        portals.insert(new_portal);
                        let (wall, portal) = if let Some(MapElem::Path) = map[y].get(x + 2) {
                            (x, x + 1)
                        } else if let Some(MapElem::Path) = map[y].get(x - 1) {
                            (x + 1, x)
                        } else {
                            unreachable!("There should be a path next to this portal")
                        };
                        map[y][wall] = MapElem::Wall;
                        map[y][portal] = MapElem::Portal(new_portal);
                    } else if let Some(MapElem::PortalPart(p2)) = map.get(y + 1).map(|r| r[x]) {
                        let new_portal = MapElem::create_portal(p1, p2, &portals);
                        portals.insert(new_portal);
                        let (wall, portal) =
                            if let Some(MapElem::Path) = map.get(y + 2).map(|r| r[x]) {
                                (y, y + 1)
                            } else if let Some(MapElem::Path) = map.get(y - 1).map(|r| r[x]) {
                                (y + 1, y)
                            } else {
                                unreachable!("There should be a path next to this portal")
                            };
                        map[wall][x] = MapElem::Wall;
                        map[portal][x] = MapElem::Portal(new_portal);
                    }
                }

                _ => {}
            }
        }
    }

    // just check that we cleared up all the portal parts.
    assert!(map
        .iter()
        .all(|r| r.iter().all(|e| !matches!(e, MapElem::PortalPart(_)))));
}
