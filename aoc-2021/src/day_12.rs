use aoc_common::aoc_day::AocDay;
use aoc_common::files::Res;
use std::fs::read_to_string;

pub struct Day12;

impl AocDay for Day12 {
    type Input = Vec<(String, String)>;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        12
    }
    fn load() -> Res<Self::Input> {
        Ok(read_to_string("data/2021/day_12.in")?
            .lines()
            .flat_map(parse)
            .collect())
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        find_all_routes(input, &mut vec!["start".to_owned()], true)
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        find_all_routes(input, &mut vec!["start".to_owned()], false)
    }
}

fn parse(line: &str) -> Vec<(String, String)> {
    let (left, right) = line.split_once("-").unwrap();
    vec![
        (left.to_owned(), right.to_owned()),
        (right.to_owned(), left.to_owned()),
    ]
}

fn find_all_routes(
    paths: &[(String, String)],
    from: &mut Vec<String>,
    visited_small: bool,
) -> usize {
    if *from.last().unwrap() == "end" {
        1
    } else {
        paths
            .iter()
            .filter_map(|(start, end)| {
                if start != from.last().unwrap() || "start" == end {
                    None
                } else {
                    let big = is_big(end);
                    let unseen = big || unseen(end, from);

                    if big || unseen || !visited_small {
                        // I could replace all strings with small structs e.g. `(label, big)`.
                        // This would prevent these clones.
                        // Or I could try to RC, not done that in 12 months!
                        from.push(end.clone());
                        let sum = find_all_routes(
                            paths,
                            from,
                            if big {
                                visited_small
                            } else {
                                !unseen || visited_small
                            },
                        );
                        from.pop();
                        Some(sum)
                    } else {
                        None
                    }
                }
            })
            .sum()
    }
}

fn is_big(s: &str) -> bool {
    s.chars().all(|c| c.is_uppercase())
}

fn unseen(s: &str, path: &[String]) -> bool {
    path.iter().all(|e| e != s)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_parse_input() {
        let res = parse(
            r#"start-A"#
// start-b
// A-c
// A-b
// b-d
// A-end
// b-end"#,
        );
        assert_eq!(res, ("start".to_owned(), "b".to_owned()));
    }
}
