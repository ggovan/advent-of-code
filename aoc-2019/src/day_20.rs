use aoc_common::aoc_day::AocDay;
use aoc_common::files::Res;
use aoc_common::geometry::Direction;
use aoc_common::search::{search, HeapElem};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

/// Note that most of the time here is spent in the "setup" phase.
/// Moving these to the `input` function would approx. half the total time taken
pub struct Day20;

impl AocDay for Day20 {
    type Input = HashMap<Portal, Vec<(Portal, i64)>>;
    type Result1 = u64;
    type Result2 = u64;

    fn day() -> usize {
        20
    }
    fn load() -> Res<Self::Input> {
        let input = read_to_string("data/2019/day_20.in")?;
        let mut input = input
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
            .collect::<Vec<_>>();
        setup_portals(&mut input);
        let graph = setup_graph(&input);
        Ok(graph)
    }

    /// Simply process the input, then find all the portals reachable from other portals, then do an A*.
    fn part_1(input: &Self::Input) -> Self::Result1 {
        let graph = &input;

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

    /// Simply process the input, then find all the portals reachable from other portals, then do an A*.
    /// But this time you need to keep track of the maze depth too.
    /// It will panic after going 50 mazes deep, just to stop an infinite search.
    fn part_2(input: &Self::Input) -> Self::Result2 {
        let graph = &input;

        search(
            (Portal('A', 'A', false), 0),
            |p| matches!(p, (Portal('Z', 'Z', _), -1)),
            move |(portal, depth), distance| {
                graph[&portal]
                    .iter()
                    .map(move |(p, d)| HeapElem {
                        elem: (Portal(p.0, p.1, !p.2), depth + if p.2 { 1 } else { -1 }),
                        distance: *d as u64 + distance + 1,
                        heuristic: 0,
                    })
                    .filter(|he| he.elem.1 >= -1)
                    .filter(|he| he.elem.0 .0 != 'A' || he.elem.0 .1 != 'A')
                    .filter(|he| !(he.elem.0 .0 == 'Z' && he.elem.0 .1 == 'Z') || he.elem.1 == -1)
                    // since it's recursive, just stop at 50 mazes deep
                    .filter(|he| he.elem.1 < 50)
            },
        )
        .1 - 1
    }
}

/// Setup a graph of `Portal -> [(Portal, distance)]`
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

/// two chars acting as the portal name, then an is_inside bool
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
    fn create_portal(p1: u8, p2: u8, is_inside: bool) -> Portal {
        let p1 = p1 as char;
        let p2 = p2 as char;
        if p1 < p2 {
            Portal(p1, p2, is_inside)
        } else {
            Portal(p2, p1, is_inside)
        }
    }
}

/// Replace pairs of "portal parts" with portals, e.g.
/// `'A','B','.' -> ' ',"AB",'.'`
/// and marks the portal as "inner" or "outer".
fn setup_portals(map: &mut [Vec<MapElem>]) {
    let row_length = map[0].len();
    let mut portals = HashSet::new();
    let mut is_inside_y = 0;
    for y in 0..map.len() {
        let mut is_inside_x = 0;
        if is_inside_y == 0
            && map[y]
                .iter()
                .any(|p| matches!(p, MapElem::Wall | MapElem::Path))
        {
            is_inside_y = 1;
        }
        if is_inside_y == 1
            && map[y]
                .iter()
                .all(|p| !matches!(p, MapElem::Wall | MapElem::Path))
        {
            is_inside_y = 2;
        }
        for x in 0..row_length {
            match map[y][x] {
                MapElem::PortalPart(p1) => {
                    if let Some(MapElem::PortalPart(p2)) = map[y].get(x + 1) {
                        let new_portal =
                            MapElem::create_portal(p1, *p2, (1..=2).contains(&is_inside_x));
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
                        let new_portal = MapElem::create_portal(p1, p2, is_inside_y == 1);
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

                MapElem::Wall | MapElem::Path if is_inside_x == 0 || is_inside_x == 2 => {
                    is_inside_x += 1;
                }

                MapElem::Space if is_inside_x == 1 => {
                    is_inside_x += 1;
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
