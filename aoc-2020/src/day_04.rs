use aoc_common::files::{read_lines, Res};
use aoc_common::aoc_day::AocDay;
use lazy_static::*;
use regex::Regex;

pub struct Day04;

impl AocDay for Day04 {
    type Input = Vec<Vec<(String, String)>>;
    type Result1 = usize;
    type Result2 = usize;

    fn day() -> usize {
        4
    }

    fn load() -> Res<Self::Input> {
        Ok(read_lines("data/2020/day_04.in")?
            .map(|l| l.unwrap())
            .fold::<Self::Input, _>(vec![vec![]], |mut out, line| {
                if line.is_empty() {
                    out.push(vec![]);
                }

                out.last_mut().unwrap().append(
                    &mut line
                        .split(' ')
                        .filter(|w| !w.is_empty())
                        .map(|c| {
                            let mut d = c.split(':');
                            (d.next().unwrap().to_owned(), d.next().unwrap().to_owned())
                        })
                        .collect::<Vec<_>>(),
                );

                out
            }))
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        input
            .iter()
            .filter(|pp| pp.len() == 8 || (pp.len() == 7 && pp.iter().all(|(key, _)| key != "cid")))
            .count()
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        input.iter().filter(|pp| is_valid(pp)).count()
    }
}

fn check_rules((k, v): &(std::string::String, std::string::String)) -> bool {
    match k.as_ref() {
        "byr" => byr(v),
        "iyr" => iyr(v),
        "eyr" => eyr(v),
        "hgt" => hgt(v),
        "hcl" => hcl(v),
        "ecl" => ecl(v),
        "pid" => pid(v),
        "cid" => true,
        _ => panic!("Unknown key"),
    }
}

fn is_valid(pp: &[(std::string::String, std::string::String)]) -> bool {
    (pp.len() == 8 || pp.len() == 7 && pp.iter().all(|(key, _)| key != "cid"))
        && pp.iter().all(check_rules)
}

fn byr(s: &str) -> bool {
    let v = s.parse::<usize>().unwrap();
    (1920..=2002).contains(&v) && s.len() == 4
}

fn iyr(s: &str) -> bool {
    let v = s.parse::<usize>().unwrap();
    (2010..=2020).contains(&v) && s.len() == 4
}

fn eyr(s: &str) -> bool {
    let v = s.parse::<usize>().unwrap();
    (2020..=2030).contains(&v) && s.len() == 4
}

fn hgt(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    }
    RE.captures(s)
        .and_then(|captures| {
            captures[1]
                .parse::<usize>()
                .map(|v| match &captures[2] {
                    "cm" => (150..=193).contains(&v),
                    "in" => (59..=76).contains(&v),
                    _ => false,
                })
                .ok()
        })
        .unwrap_or(false)
}

fn hcl(s: &str) -> bool {
    let mut iter = s.chars();
    s.len() == 7
        && iter.next().unwrap() == '#'
        && iter.all(|c| ('0'..='9').contains(&c) || ('a'..='f').contains(&c))
}

fn ecl(s: &str) -> bool {
    matches!(s, "amb" | "blu" | "brn" | "grn" | "gry" | "hzl" | "oth")
}

fn pid(s: &str) -> bool {
    s.len() == 9 && s.chars().all(|c| ('0'..='9').contains(&c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byr() {
        assert_eq!(byr("1920"), true);
        assert_eq!(byr("2002"), true);
        assert_eq!(byr("2003"), false);
        assert_eq!(byr("1919"), false);
    }

    #[test]
    fn test_iyr() {
        assert_eq!(iyr("2010"), true);
        assert_eq!(iyr("2020"), true);
        assert_eq!(iyr("2009"), false);
        assert_eq!(iyr("2021"), false);
    }

    #[test]
    fn test_eyr() {
        assert_eq!(eyr("2020"), true);
        assert_eq!(eyr("2030"), true);
        assert_eq!(eyr("2019"), false);
        assert_eq!(eyr("2031"), false);
    }

    #[test]
    fn test_hgt() {
        assert_eq!(hgt("60in"), true);
        assert_eq!(hgt("73in"), true);
        assert_eq!(hgt("190cm"), true);
        assert_eq!(hgt("150cm"), true);
        assert_eq!(hgt("149cm"), false);
        assert_eq!(hgt("77in"), false);
        assert_eq!(hgt("194cm"), false);
        assert_eq!(hgt("190in"), false);
        assert_eq!(hgt("190"), false);
        assert_eq!(hgt("cm"), false);
    }

    #[test]
    fn test_hcl() {
        assert_eq!(hcl("#123abc"), true);
        assert_eq!(hcl("#1a0b4f"), true);
        assert_eq!(hcl("#ff0ab3"), true);
        assert_eq!(hcl("#1a0b4fa"), false);
        assert_eq!(hcl("#1a4fa"), false);
        assert_eq!(hcl("#123abz"), false);
        assert_eq!(hcl("123abc"), false);
    }

    #[test]
    fn test_ecl() {
        assert_eq!(ecl("amb"), true);
        assert_eq!(ecl("blu"), true);
        assert_eq!(ecl("brn"), true);
        assert_eq!(ecl("gry"), true);
        assert_eq!(ecl("grn"), true);
        assert_eq!(ecl("hzl"), true);
        assert_eq!(ecl("oth"), true);
        assert_eq!(ecl("wat"), false);
    }

    #[test]
    fn test_pid() {
        assert_eq!(pid("000000001"), true);
        assert_eq!(pid("0123456789"), false);
        assert_eq!(pid("0000a0001"), false);
    }

    #[test]
    fn test_example_1() {
        let pp = vec![
            ("eyr".to_owned(), "1972".to_owned()),
            ("cid".to_owned(), "100".to_owned()),
            ("hcl".to_owned(), "#18171d".to_owned()),
            ("ecl".to_owned(), "amb".to_owned()),
            ("hgt".to_owned(), "170".to_owned()),
            ("pid".to_owned(), "186cm".to_owned()),
            ("iyr".to_owned(), "2018".to_owned()),
            ("byr".to_owned(), "1926".to_owned()),
        ];
        assert_eq!(is_valid(&pp), false);
    }
}
