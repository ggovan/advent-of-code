use aoc_common::aoc_day::AocDay;
use aoc_common::files::Res;
use hashbrown::{HashMap, HashSet};
use std::collections::VecDeque;
use std::fs::read_to_string;

pub struct Day11;

impl AocDay for Day11 {
    type Input = Vec<(String, Vec<String>)>;
    type Result1 = i64;
    type Result2 = i64;

    fn day() -> usize {
        11
    }

    fn load() -> Res<Self::Input> {
        Ok(parse(&read_to_string("data/2025/day_11.in")?))
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        let reversed_graph = reverse_graph(input);
        paths_from(input, &reversed_graph, "you", "out")
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        let route = ["svr", "fft", "dac", "out"];

        let reversed_graph = reverse_graph(input);

        // 349322478796032
        paths_from(input, &reversed_graph, route[0], route[1])
            * paths_from(input, &reversed_graph, route[1], route[2])
            * paths_from(input, &reversed_graph, route[2], route[3])
    }
}

fn reverse_graph<'a>(input: &'a Vec<(String, Vec<String>)>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut rev_graph: HashMap<&str, Vec<&str>> = HashMap::with_capacity(input.len() * 2);

    for (node, children) in input.iter() {
        for child in children.iter() {
            rev_graph
                .entry(child.as_str())
                .or_default()
                .push(node.as_str());
        }
    }

    rev_graph
}

fn paths_from<'a>(
    input: &Vec<(String, Vec<String>)>,
    reversed_graph: &HashMap<&'a str, Vec<&'a str>>,
    start: &str,
    goal: &str,
) -> i64 {
    let mut costs: HashMap<&str, i64> = HashMap::with_capacity(input.len() * 2);

    costs.insert(start, 1);

    let mut iterations = 0;

    let mut nodeset = HashSet::with_capacity(reversed_graph.len());
    for (key, vs) in reversed_graph.iter() {
        nodeset.insert(*key);
        for v in vs.iter() {
            nodeset.insert(*v);
        }
    }

    let mut queue: VecDeque<&str> = nodeset.clone().into_iter().collect();

    queue.push_front(goal);

    while (costs.get(goal).is_none()) && !queue.is_empty() {
        iterations += 1;
        if iterations == 1_000_000 {
            println!("Queue: {:?}", queue);
            println!("Costs so far: {:?}", costs);
            unreachable!("Stuck in loop computing costs");
        }
        let node = queue.pop_front().unwrap();
        if costs.get(node).is_some() {
            continue;
        }
        let parents = match reversed_graph.get(node) {
            Some(p) => p,
            None => {
                // unreachable node, so 0 cost
                costs.insert(node, 0);
                continue;
            }
        };
        let mut all_parents_known = true;
        let mut total_cost: i64 = 0;

        if parents.is_empty() {
            costs.insert(node, 0);
            continue;
        }

        for parent in parents.iter() {
            match costs.get(parent) {
                Some(c) => total_cost += c,
                None => {
                    all_parents_known = false;
                    break;
                }
            }
        }

        if all_parents_known {
            costs.insert(node, total_cost);
        } else {
            queue.push_back(node);
        }
    }
    costs.get(goal).cloned().unwrap()
}

fn parse(input: &str) -> <Day11 as AocDay>::Input {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let key = parts.next().unwrap().to_string();
            let values = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            (key, values)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    //          you
    //     bbb        ccc (1)
    //   ddd eee    ddd eee fff (1)
    //

    #[test]
    fn part_1() {
        let input = parse(TEST_INPUT);
        let res = Day11::part_1(&input);
        assert_eq!(res, 5);
    }

    const TEST_INPUT_2: &str = r"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn part_2() {
        let input = parse(TEST_INPUT_2);
        let res = Day11::part_2(&input);
        assert_eq!(res, 2);
    }
}
