use super::Aoc2020;
use crate::files::{read_lines, Res};
use lazy_static::*;
use regex::Regex;
use std::collections::HashMap;

pub struct Day04;

impl Aoc2020 for Day04 {
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
        let fn_map = create_fn_map();
        input.iter().filter(|pp| is_valid(&fn_map, pp)).count()
    }
}

type FnMap = HashMap<String, &'static dyn Fn(&str) -> bool>;

fn create_fn_map() -> FnMap {
    // I tried to make this a lazy_static in `is_valid`.
    let mut m: HashMap<String, &'static dyn Fn(&str) -> bool> = HashMap::new();
    m.insert("byr".to_owned(), &byr);
    m.insert("iyr".to_owned(), &iyr);
    m.insert("eyr".to_owned(), &eyr);
    m.insert("hgt".to_owned(), &hgt);
    m.insert("hcl".to_owned(), &hcl);
    m.insert("ecl".to_owned(), &ecl);
    m.insert("pid".to_owned(), &pid);
    m.insert("cid".to_owned(), &cid);
    m
}

fn is_valid(fn_map: &FnMap, pp: &[(std::string::String, std::string::String)]) -> bool {
    (pp.len() == 8
        || pp.len() == 7
            && pp
                .iter()
                .all(|(key, _): &(std::string::String, std::string::String)| key != "cid"))
        && pp.iter().all(|(k, v)| fn_map[k](v))
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
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    RE.is_match(s)
}

fn ecl(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(amb|blu|brn|grn|gry|hzl|oth)$").unwrap();
    }
    RE.is_match(s)
}

fn pid(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    RE.is_match(s)
}

fn cid(_: &str) -> bool {
    true
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
        let fn_map = create_fn_map();

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
        assert_eq!(is_valid(&fn_map, &pp), false);
    }
}
