use std::cmp::{Eq, Ordering, PartialEq};
use std::collections::{BinaryHeap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(PartialEq, Copy, Clone, Eq, Debug)]
pub struct HeapElem<E> {
    pub elem: E,
    pub distance: u64,
    pub heuristic: u64,
}

impl<E: Eq> Ord for HeapElem<E> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.distance + self.heuristic)
            .cmp(&(other.distance + other.heuristic))
            .reverse()
    }
}

impl<E: Eq> PartialOrd for HeapElem<E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn search<E: Hash + Eq + Clone, Iter: Iterator<Item = HeapElem<E>>>(
    start: E,
    is_goal: impl Fn(&E) -> bool,
    mut successors: impl FnMut(E, u64) -> Iter,
) -> (E, u64) {
    let _time = super::time_block("    search");
    let mut queue: BinaryHeap<HeapElem<E>> = BinaryHeap::new();
    let mut visited: HashSet<E> = HashSet::new();
    queue.push(HeapElem {
        elem: start,
        distance: 0,
        heuristic: 0,
    });

    while let Some(HeapElem { elem, distance, .. }) = queue.pop() {
        if visited.contains(&elem) {
            continue;
        }
        if is_goal(&elem) {
            return (elem, distance);
        }

        for next in successors(elem.clone(), distance) {
            queue.push(next)
        }

        visited.insert(elem);
    }

    unreachable!("Failed to reach goal");
}
