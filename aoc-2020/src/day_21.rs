use aoc_common::aoc_day::AocDay;
use aoc_common::files::Res;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::rc::Rc;

pub struct Day21;

impl AocDay for Day21 {
    type Input = Vec<(HashSet<String>, HashSet<String>)>;
    type Result1 = usize;
    type Result2 = String;

    fn day() -> usize {
        21
    }
    fn load() -> Res<Self::Input> {
        let input = read_to_string("data/2020/day_21.in")?;
        Ok(input.lines().map(parse_line).collect())
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        solve_both_parts(input).0
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        solve_both_parts(input).1
    }
}

fn solve_both_parts(input: &[(HashSet<String>, HashSet<String>)]) -> (usize, String) {
    let mut allergens: HashSet<Rc<String>> = HashSet::new();
    let mut ingredients: HashSet<Rc<String>> = HashSet::new();
    for f in input {
        for i in f.0.iter() {
            if !ingredients.contains(i) {
                ingredients.insert(Rc::new(i.clone()));
            }
        }
        for a in f.1.iter() {
            if !allergens.contains(a) {
                allergens.insert(Rc::new(a.clone()));
            }
        }
    }

    let mut solved_allergens = HashSet::new();
    let mut danger_pairs: Vec<(Rc<String>, Rc<String>)> = Vec::new();

    while solved_allergens.len() != allergens.len() {
        for a in allergens.iter() {
            if solved_allergens.contains(a) {
                continue;
            }
            let solved_ingredient = {
                let mut ingredients = ingredients.clone();
                for (i_s, _) in input
                    .iter()
                    .filter(|(_, a_s)| a_s.contains(<Rc<String> as Borrow<String>>::borrow(a)))
                {
                    ingredients = ingredients
                        .into_iter()
                        .filter(|i| {
                            i_s.contains(<Rc<String> as std::borrow::Borrow<String>>::borrow(i))
                        })
                        .collect();
                }
                if ingredients.len() == 1 {
                    ingredients.into_iter().next().clone()
                } else {
                    None
                }
            };
            if let Some(i) = solved_ingredient {
                solved_allergens.insert(a.clone());
                ingredients.remove(&i);
                danger_pairs.push((a.clone(), i.clone()));
            }
        }
    }

    // How many times do safe ingredients appear?
    let safe_ingredients = input
        .iter()
        .map(|(i_s, _)| i_s.iter().filter(|&i| ingredients.contains(i)).count())
        .sum();

    danger_pairs.sort_by_key(|(a, _)| a.clone());
    let dangerous_ingredients = danger_pairs
        .iter()
        .map(|(_, i)| i.to_string())
        .collect::<Vec<String>>()
        .join(",");

    (safe_ingredients, dangerous_ingredients)
}

fn parse_line(s: &str) -> (HashSet<String>, HashSet<String>) {
    let ingredients = s
        .split(" (")
        .next()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let allergens = s
        .split("(contains ")
        .nth(1)
        .unwrap()
        .split(')')
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();

    (ingredients, allergens)
}
