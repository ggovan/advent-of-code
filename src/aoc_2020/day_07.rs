use super::Aoc2020;
use crate::files::{read_lines, Res};
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day07;

impl Aoc2020 for Day07 {
    type Input = Vec<Rule>;
    type Result1 = usize;
    type Result2 = u64;

    fn day() -> usize {
        7
    }

    fn load() -> Res<Self::Input> {
        Ok(read_lines("data/2020/day_07.in")?
            .map(|l| parse_ln(&l.unwrap()))
            .collect())
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        let mut inverse_map: HashMap<&String, Vec<&String>> = HashMap::new();
        let mut containing_bags: HashSet<&String> = HashSet::new();

        for Rule { bag, contains } in input {
            for (_, x) in contains {
                if let Some(vec) = inverse_map.get_mut(x) {
                    vec.push(bag);
                } else {
                    inverse_map.insert(x, vec![bag]);
                }
            }
        }

        let mut queue: VecDeque<&String> = VecDeque::new();
        let target = "shiny gold".to_owned();
        queue.push_back(&target);

        while let Some(b) = queue.pop_front() {
            if containing_bags.contains(b) {
                continue;
            }

            containing_bags.insert(b);
            if let Some(contains) = inverse_map.get(b) {
                for x in contains.iter() {
                    if containing_bags.contains(x) {
                        continue;
                    }
                    queue.push_back(x);
                }
            }
        }

        containing_bags.len() - 1
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        let mut map: HashMap<&String, &Rule> = HashMap::new();

        for r in input {
            map.insert(&r.bag, r);
        }
        let target = "shiny gold".to_owned();

        bag_contains(&map, &target) - 1
    }
}

fn bag_contains(map: &HashMap<&String, &Rule>, bag: &String) -> u64 {
    let mut count = 1_u64;
    if let Some(rule) = map.get(bag) {
        for (c, b2) in &rule.contains {
            count += *c as u64 * bag_contains(map, b2);
        }
    }

    count
}

#[derive(Debug, PartialEq)]
pub struct Rule {
    bag: String,
    contains: Vec<(usize, String)>,
}

fn parse_ln(line: &str) -> Rule {
    let mut m = line.split(" bags contain ");
    let bag = m.next().unwrap().to_owned();
    let contains = m
        .next()
        .unwrap()
        .split(", ")
        .filter_map(|s| {
            let mut m = s.split(' ');
            m.next()
                .unwrap()
                .parse::<usize>()
                .map(|count| {
                    (
                        count,
                        format!("{} {}", m.next().unwrap(), m.next().unwrap()),
                    )
                })
                .ok()
        })
        .collect::<Vec<_>>();
    Rule { bag, contains }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let res = parse_ln("vibrant green bags contain 3 shiny magenta bags, 1 drab magenta bag, 4 posh silver bags, 2 shiny orange bags.");
        assert_eq!(
            res,
            Rule {
                bag: "vibrant green".to_owned(),
                contains: vec![
                    (3, "shiny magenta".to_owned()),
                    (1, "drab magenta".to_owned()),
                    (4, "posh silver".to_owned()),
                    (2, "shiny orange".to_owned())
                ]
            }
        );
    }

    #[test]
    fn test_part_1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
                     dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
                     bright white bags contain 1 shiny gold bag.\n\
                     muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
                     shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
                     dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
                     vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
                     faded blue bags contain no other bags.\n\
                     dotted black bags contain no other bags."
            .split('\n')
            .map(parse_ln)
            .collect::<Vec<_>>();
        let res = Day07::part_1(&input);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_part_2() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
                     dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
                     bright white bags contain 1 shiny gold bag.\n\
                     muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
                     shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
                     dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
                     vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
                     faded blue bags contain no other bags.\n\
                     dotted black bags contain no other bags."
            .split('\n')
            .map(parse_ln)
            .collect::<Vec<_>>();
        let res = Day07::part_2(&input);
        assert_eq!(res, 32);
    }
}
