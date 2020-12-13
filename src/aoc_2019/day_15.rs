use super::intcode;
use crate::aoc_2020::Aoc2020;
use crate::files::Res;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{cmp::Ordering, collections::VecDeque};

pub struct Day15;

type Point = (i64, i64);

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate_cw(&self) -> Self {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn rotate_acw(&self) -> Self {
        use Direction::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    fn rotate(&self, acw: bool) -> Self {
        if acw {
            self.rotate_acw()
        } else {
            self.rotate_cw()
        }
    }

    fn as_input(&self) -> i64 {
        use Direction::*;
        match self {
            North => 1,
            East => 4,
            South => 2,
            West => 3,
        }
    }

    fn next_point(&self, (x, y): Point) -> Point {
        use Direction::*;
        match self {
            North => (x, y - 1),
            East => (x + 1, y),
            South => (x, y + 1),
            West => (x - 1, y),
        }
    }
}

type Map = HashMap<Point, char>;

struct MazeSim {
    machine: intcode::Machine,
    map: Map,
    pos: Point,
}

impl MazeSim {
    fn new(input: &Vec<i64>, map: Option<Map>) -> Self {
        MazeSim {
            machine: intcode::Machine::new(input, vec![]),
            map: map.unwrap_or(HashMap::new()),
            pos: (0, 0),
        }
    }

    /// try to move in the given direction, return the character that was there (or is there if you can't move).
    fn try_move(&mut self, dir: Direction) -> char {
        let mut res = ' ';
        if let Some(output) = self.machine.run_to_output(Some(dir.as_input())) {
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

impl Aoc2020 for Day15 {
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
        intcode::output_map(&map);

        let (&goal, _) = map.iter().find(|(_, &v)| v == 'O').unwrap();

        search(&map, (0, 0), goal)
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

/// A point with a distance, and heuristic for use in A*
#[derive(PartialEq, Copy, Clone, Eq)]
struct HeapElem {
    point: Point,
    dist: i64,
    heuristic: i64,
}

fn create_he(point: Point, dist: i64, (gx, gy): &Point) -> HeapElem {
    // use the manhattan distance as a heuristic
    let heuristic = gx.abs() + gy.abs();
    HeapElem {
        point,
        dist,
        heuristic,
    }
}

impl Ord for HeapElem {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.dist + self.heuristic)
            .cmp(&(other.dist + other.heuristic))
            .reverse()
    }
}

impl PartialOrd for HeapElem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Uses A* to find the min distance to the goal.
/// Turns out that that doesn't matter, as I got the same answer when using a max instead of min heap.
/// It might make it more efficient, but no point since the maze is much slower.
fn search(map: &Map, start: Point, goal: Point) -> i64 {
    let mut queue: BinaryHeap<HeapElem> = BinaryHeap::new();
    let mut visited: HashSet<Point> = HashSet::new();
    queue.push(create_he(start, 0, &goal));

    while queue.peek().unwrap().point != goal {
        let HeapElem {
            point,
            dist,
            heuristic: _,
        } = queue.pop().unwrap();
        if visited.contains(&point) {
            continue;
        }
        let (x, y) = point;
        let successors = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
        for successor in successors.iter() {
            if !map.contains_key(successor) {
                // not on the path
                continue;
            }
            if visited.contains(successor) {
                continue;
            }
            queue.push(create_he(*successor, dist + 1, &goal));
        }

        visited.insert((x, y));
    }

    queue.peek().unwrap().dist
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
