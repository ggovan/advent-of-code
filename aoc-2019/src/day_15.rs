use super::intcode;
use aoc_common::aoc_day::AocDay;
use aoc_common::files::Res;
use aoc_common::geometry::{self, Direction};
use aoc_common::search::{search, HeapElem};
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

pub struct Day15;

type Point = (i64, i64);

type Map = HashMap<Point, char>;

struct MazeSim {
    machine: intcode::Machine,
    map: Map,
    pos: Point,
}

fn input_from_direction(dir: &Direction) -> i64 {
    use Direction::*;
    match dir {
        North => 1,
        East => 4,
        South => 2,
        West => 3,
    }
}

impl MazeSim {
    fn new(input: &[i64], map: Option<Map>) -> Self {
        MazeSim {
            machine: intcode::Machine::new(input, vec![]),
            map: map.unwrap_or_default(),
            pos: (0, 0),
        }
    }

    /// try to move in the given direction, return the character that was there (or is there if you can't move).
    fn try_move(&mut self, dir: Direction) -> char {
        let mut res = ' ';
        if let Some(output) = self.machine.run_to_output(Some(input_from_direction(&dir))) {
            match output {
                0 => {
                    self.map.insert(dir.next_point(self.pos), '#');
                    res = '#'
                }
                1 => {
                    self.map.insert(self.pos, '.');
                    self.pos = dir.next_point(self.pos);
                    self.map.insert(self.pos, 'D');
                    res = '.'
                }
                2 => {
                    self.map.insert(self.pos, '.');
                    self.pos = dir.next_point(self.pos);
                    self.map.insert(self.pos, 'O');
                    res = 'O'
                }
                _ => unreachable!("not be here"),
            }
        }
        res
    }

    /// Explore an intcode maze by finding a wall and then keeping ones left hand on it (or right when acw is true)
    fn explore_maze(mut self, acw: bool) -> Self {
        let mut dir = Direction::North;
        self.map.insert(self.pos, 'D');

        while self.try_move(dir) != '#' {}
        dir = dir.rotate(acw);

        loop {
            //try to move forward
            let next = self.try_move(dir);
            match next {
                '#' => {
                    // turn right and move (loop)
                    dir = dir.rotate(acw);
                }
                '.' => {
                    // we moved, so turn left and sww if we can move that way
                    dir = dir.rotate(!acw);
                }
                _ => break,
            }
        }

        self.map.insert((0, 0), 'S');
        self
    }
}

impl AocDay for Day15 {
    type Input = Vec<i64>;
    type Result1 = i64;
    type Result2 = i64;

    fn day() -> usize {
        15
    }
    fn load() -> Res<Self::Input> {
        intcode::Machine::load_tape_from_file("data/2019/day_15.in")
    }

    /// Do two maze explorations (one left and one right, to ensure that it's fully explored),
    /// and then search (A*) through the open spaces to find the shortest path.
    fn part_1(input: &Self::Input) -> Self::Result1 {
        // Go both directions to fully explore the maze
        let map = MazeSim::new(input, None).explore_maze(false).map;
        let map = MazeSim::new(input, Some(map)).explore_maze(true).map;

        // We only care about routes, throw away the walls
        let map: Map = map.into_iter().filter(|(_, v)| *v != '#').collect();
        let map_ref = &map;
        geometry::output_map(&map);

        let (&goal, _) = map.iter().find(|(_, &v)| v == 'O').unwrap();

        let start: (i64, i64) = (0, 0);

        search(
            start,
            |p| *p == goal,
            |p, distance| {
                Direction::array().iter().filter_map(move |direction| {
                    let successor = direction.next_point(p);
                    if !map_ref.contains_key(&successor) {
                        // not on the path
                        return None;
                    }
                    Some(HeapElem {
                        elem: successor,
                        distance: distance + 1,
                        heuristic: ((goal.0 - successor.0).abs() + (goal.1 - successor.1).abs())
                            as u64,
                    })
                })
            },
        )
        .1 as i64
    }

    /// Do two maze explorations (one left and one right, to ensure that it's fully explored),
    /// and then do a breadth first search to find the "depth" of the "tree".
    fn part_2(input: &Self::Input) -> Self::Result2 {
        // Go both directions to fully explore the maze
        let map = MazeSim::new(input, None).explore_maze(false).map;
        let map = MazeSim::new(input, Some(map)).explore_maze(true).map;

        // We only care about routes, throw away the walls
        let map: Map = map.into_iter().filter(|(_, v)| *v != '#').collect();
        let (&goal, _) = map.iter().find(|(_, &v)| v == 'O').unwrap();

        depth(&map, goal)
    }
}

fn depth(map: &Map, start: Point) -> i64 {
    let mut queue: VecDeque<(Point, i64)> = VecDeque::new();
    let mut queued: HashSet<Point> = HashSet::new();
    queue.push_back((start, 0));
    queued.insert(start);
    let mut depth = 0;

    while let Some(point) = queue.pop_front() {
        let ((x, y), d) = point;
        depth = d;
        let successors = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
        for successor in successors.iter() {
            if !map.contains_key(successor) {
                // not on the path
                continue;
            }
            if queued.contains(successor) {
                continue;
            }
            queue.push_back((*successor, depth + 1));
            queued.insert((x, y));
        }
    }

    depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = Day15::load().unwrap();
        let res = Day15::part_1(&input);
        assert_eq!(res, 354);
    }

    #[test]
    fn part_2() {
        let input = Day15::load().unwrap();
        let res = Day15::part_2(&input);
        assert_eq!(res, 370);
    }
}
