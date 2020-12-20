use crate::aoc_2020::Aoc2020;
use crate::files::Res;
use std::collections::HashMap;
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
            .filter(|l| {
                if let Some(len) = matches(map, l, 0, 0) {
                    if len == l.len() {
                        // println!("Accepted: {}", l);
                        len == l.len()
                    } else {
                        // println!("Rejected: {}", l);
                        false
                    }
                } else {
                    // println!("Rejected: {}", l);
                    false
                }
            })
            .count()
    }

    fn part_2((map, input): &Self::Input) -> Self::Result2 {
        let mut map = map.clone();

        map.insert(8, Either((8, 0, 0), (42, 8, 0)));
        map.insert(11, Either((42, 31, 0), (42, 11, 31)));

        input
            .iter()
            .filter(|l| {
                if let Some(len) = matches(&map, l, 0, 0) {
                    if len == l.len() {
                        // println!("Accepted: {}", l);
                        len == l.len()
                    } else {
                        // println!("Rejected: {}", l);
                        false
                    }
                } else {
                    // println!("Rejected: {}", l);
                    false
                }
            })
            .count()
    }
}

fn matches(map: &RuleMap, s: &str, rule: usize, index: usize) -> Option<usize> {
    println!("{} {} {}", s, rule, index);
    if index >= s.len() {
        return None;
    }
    match map[&rule] {
        Char(c) => {
            s.chars()
                .skip(index)
                .next()
                .and_then(|o| if o == c { Some(index + 1) } else { None })
        }
        Sequence(seq) => match_seq(map, s, seq, index),
        Either(s1, s2) => match_seq(map, s, s2, index).or_else(|| match_seq(map, s, s1, index)),
    }
}

fn match_seq(map: &RuleMap, s: &str, (s1, s2, s3): Seq, index: usize) -> Option<usize> {
    matches(map, s, s1, index)
        .and_then(|index| {
            if s2 == 0 {
                Some(index)
            } else {
                matches(map, s, s2, index)
            }
        })
        .and_then(|index| {
            if s3 == 0 {
                Some(index)
            } else {
                matches(map, s, s3, index)
            }
        })
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
