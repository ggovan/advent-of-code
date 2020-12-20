use crate::aoc_2020::Aoc2020;
use crate::files::Res;
use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

pub struct Day19;

type RuleMap = HashMap<usize, Rule>;

impl Aoc2020 for Day19 {
    type Input = (RuleMap, Vec<String>);
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        19
    }
    fn load() -> Res<Self::Input> {
        let input = read_to_string("data/2020/day_19.in")?;
        let (rules, text) = input.split_once("\n\n").unwrap();
        Ok((
            parse_rules(rules),
            text.lines().map(|l| l.to_string()).collect(),
        ))
    }

    fn part_1((map, input): &Self::Input) -> Self::Result1 {
        input
            .iter()
            .filter(|l| matcher_2(&map, &l.chars().collect::<Vec<_>>()))
            .count()
    }

    fn part_2((map, input): &Self::Input) -> Self::Result2 {
        let mut map = map.clone();

        map.insert(8, Either((42, 0, 0), (42, 8, 0)));
        map.insert(11, Either((42, 31, 0), (42, 11, 31)));

        input
            .iter()
            .filter(|l| matcher_2(&map, &l.chars().collect::<Vec<_>>()))
            .count()
    }
}

struct StackElem {
    rule: usize,
    index: usize,
    queue: VecDeque<usize>,
}

fn matcher_2(map: &RuleMap, s: &[char]) -> bool {
    let mut stack: VecDeque<StackElem> = VecDeque::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(0);
    let mut index = 0;
    let mut go_right = false;

    while let Some(rule) = queue.pop_front() {
        match map[&rule] {
            Char(c) if s.get(index).cloned() == Some(c) => {
                index += 1;
            }
            Char(_) => {
                if let Some(stack_elem) = stack.pop_back() {
                    index = stack_elem.index;
                    queue = stack_elem.queue;
                    queue.push_front(stack_elem.rule);
                    go_right = true;
                } else {
                    return false;
                }
            }
            Sequence((p1, p2, p3)) => {
                if p3 != 0 {
                    queue.push_front(p3);
                }
                if p2 != 0 {
                    queue.push_front(p2);
                }
                queue.push_front(p1);
            }
            Either(s1, s2) => {
                if go_right {
                    go_right = false;

                    let (p1, p2, p3) = s2;

                    if p3 != 0 {
                        queue.push_front(p3);
                    }
                    if p2 != 0 {
                        queue.push_front(p2);
                    }
                    queue.push_front(p1);
                } else {
                    let cloned_queue = queue.clone();
                    stack.push_back(StackElem {
                        rule,
                        index,
                        queue: cloned_queue,
                    });

                    let (p1, p2, p3) = s1;

                    if p3 != 0 {
                        queue.push_front(p3);
                    }
                    if p2 != 0 {
                        queue.push_front(p2);
                    }
                    queue.push_front(p1);
                }
            }
        }
    }

    index == s.len()
}

type Seq = (usize, usize, usize);

#[derive(Copy, Clone)]
pub enum Rule {
    Char(char),
    Either(Seq, Seq),
    Sequence(Seq),
}
use Rule::*;

fn parse_rules(s: &str) -> HashMap<usize, Rule> {
    s.lines().map(parse_rule).collect()
}

fn str_to_seq(s: &str) -> Seq {
    let mut iter = s.split(' ');
    (
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap_or("0").parse().unwrap(),
        iter.next().unwrap_or("0").parse().unwrap(),
    )
}

fn parse_rule(s: &str) -> (usize, Rule) {
    let (id, s) = s.split_once(": ").unwrap();
    let rule = if let Some((left, right)) = s.split_once(" | ") {
        Either(str_to_seq(left), str_to_seq(right))
    } else if let Some((_, s)) = s.split_once('"') {
        Char(s.chars().next().unwrap())
    } else {
        Sequence(str_to_seq(s))
    };
    (id.parse().unwrap(), rule)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let map = parse_rules(
            r#"0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b""#,
        );

        let input = (
            map,
            vec![
                "aab".to_string(),
                "bab".to_string(),
                "aabb".to_string(),
                "aba".to_string(),
            ],
        );

        let res = Day19::part_1(&input);
        assert_eq!(res, 2);
    }

    #[test]
    fn part_1_example_2() {
        let map = parse_rules(
            r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b""#,
        );

        let input = (
            map,
            vec![
                "ababbb".to_string(),
                "bababa".to_string(),
                "abbbab".to_string(),
                "aaabbb".to_string(),
                "aaaabbb".to_string(),
            ],
        );
        let res = Day19::part_1(&input);
        assert_eq!(res, 2);
    }

    #[test]
    fn part_2_example_2() {
        let map = parse_rules(
            r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1"#,
        );

        let input = (
            map,
            vec![
                "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa".to_string(),
                "bbabbbbaabaabba".to_string(),
                "babbbbaabbbbbabbbbbbaabaaabaaa".to_string(),
                "aaabbbbbbaaaabaababaabababbabaaabbababababaaa".to_string(),
                "bbbbbbbaaaabbbbaaabbabaaa".to_string(),
                "bbbababbbbaaaaaaaabbababaaababaabab".to_string(),
                "ababaaaaaabaaab".to_string(),
                "ababaaaaabbbaba".to_string(),
                "baabbaaaabbaaaababbaababb".to_string(),
                "abbbbabbbbaaaababbbbbbaaaababb".to_string(),
                "aaaaabbaabaaaaababaa".to_string(),
                "aaaabbaaaabbaaa".to_string(),
                "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa".to_string(),
                "babaaabbbaaabaababbaabababaaab".to_string(),
                "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba".to_string(),
            ],
        );
        let res = Day19::part_2(&input);
        assert_eq!(res, 12);
    }
}
