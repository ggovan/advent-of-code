use aoc_common::aoc_day::AocDay;
use aoc_common::files::{read_lines, Res};
use std::collections::HashMap;

pub struct Day14;

type Chem = (String, u64);
type Rule = (Vec<Chem>, Chem);

impl AocDay for Day14 {
    type Input = Vec<Rule>;
    type Result1 = u64;
    type Result2 = u64;

    fn day() -> usize {
        14
    }

    fn load() -> Res<Self::Input> {
        Ok(read_lines("data/2019/day_14.in")?
            .map(|l| parse_rule(&l.unwrap()))
            .collect())
    }

    fn part_1(rules: &Self::Input) -> Self::Result1 {
        let ore = "ORE".to_string();
        let ore_rule = (vec![], (ore.clone(), 1));
        let fuel = "FUEL".to_string();
        let mut map: HashMap<&str, &Rule> = HashMap::new();
        for rule in rules {
            map.insert(&rule.1 .0, rule);
        }
        map.insert(&ore, &ore_rule);

        // A topologically ordered list of chemicals
        let mut ordered = topo_order(&map, &fuel);
        ordered.push(&ore);

        let mut need_map: HashMap<&str, u64> = HashMap::new();
        for o in &ordered {
            need_map.insert(o, 0);
        }
        need_map.insert(&fuel, 1);

        for o in ordered {
            let need = need_map[o];

            let (inputs, (_, count)) = map[o];
            let reactions = (need / count) + if need % count != 0 { 1_u64 } else { 0 };

            for (c, n) in inputs {
                need_map.insert(c, need_map[&c.as_ref()] + n * reactions);
            }
        }

        need_map[&ore.as_ref()]
    }

    fn part_2(rules: &Self::Input) -> Self::Result2 {
        let ore = "ORE".to_string();
        let ore_rule = (vec![], (ore.clone(), 1));
        let ore_count = 1_000_000_000_000;
        let fuel = "FUEL".to_string();
        let mut map: HashMap<&str, &Rule> = HashMap::new();
        for rule in rules {
            map.insert(&rule.1 .0, rule);
        }
        map.insert(&ore, &ore_rule);

        let mut bank: HashMap<&str, u64> = HashMap::new();
        for rule in rules {
            bank.insert(&rule.1 .0, 0);
        }
        bank.insert(&ore, ore_count);

        // we can make at least ore/part1 fuels
        let mut count = ore_count / Self::part_1(rules);
        can_create(&map, &mut bank, &fuel, count);

        {
            // Heuristically pick some more fuel to use from the current fuel rate.
            // The fuel rate can't go down from here, right?
            let used_ore = ore_count - bank[&ore.as_ref()];
            let ore_per_fuel = used_ore as f64 / count as f64;
            let get_more = (bank[&ore.as_ref()] as f64 / ore_per_fuel).floor() as u64;
            count += get_more;
            can_create(&map, &mut bank, &fuel, get_more);
        }

        while can_create(&map, &mut bank, &fuel, 1) {
            count += 1;
            println!("ORE: {}, FUEL: {}", bank[&ore.as_ref()], &count);
        }

        // dbg!(&bank);

        count
    }
}

fn can_create(
    map: &HashMap<&str, &Rule>,
    bank: &mut HashMap<&str, u64>,
    chem: &str,
    amount: u64,
) -> bool {
    let banked = bank[chem];
    if banked >= amount {
        *bank.get_mut(chem).unwrap() = banked - amount;
        return true;
    }
    if chem == "ORE" {
        return false;
    }

    let need = amount - banked;
    let (inputs, (_, count)) = map[chem];
    let reactions = (need / count) + if need % count != 0 { 1_u64 } else { 0 };

    let left = reactions * count - need;

    *bank.get_mut(chem).unwrap() = left;

    inputs
        .iter()
        .all(|(c, a)| can_create(map, bank, c, reactions * a))
}

fn topo_order<'a>(map: &HashMap<&str, &'a Rule>, chem: &'a str) -> Vec<&'a str> {
    if chem == "ORE" {
        return vec![];
    }

    let (inputs, _) = map[chem];

    let mut ordered: Vec<&str> =
        inputs
            .iter()
            .map(|(c, _)| topo_order(map, c))
            .fold(Vec::new(), |mut acc, tp| {
                let mut new = tp
                    .into_iter()
                    .filter(|c| !acc.contains(c))
                    .collect::<Vec<_>>();
                new.append(&mut acc);
                new
            });

    ordered.insert(0, chem);
    ordered
}

fn parse_rule(s: &str) -> Rule {
    let (ins, out) = s.split_once(" => ").unwrap();
    let inputs = ins
        .split(", ")
        .map(|c| {
            let (cx, cn) = c.split_once(' ').unwrap();
            (cn.to_string(), cx.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();
    let output = {
        let (cx, cn) = out.split_once(' ').unwrap();
        (cn.to_string(), cx.parse::<u64>().unwrap())
    };
    (inputs, output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule() {
        let res = parse_rule("1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP");
        assert_eq!(
            res,
            (
                vec![
                    ("MZWV".to_string(), 1),
                    ("XDBXC".to_string(), 17),
                    ("XCVML".to_string(), 3)
                ],
                ("XMNCP".to_string(), 2)
            )
        );
    }

    #[test]
    fn test_part_1_example_1() {
        let input = "10 ORE => 10 A\n\
                     1 ORE => 1 B\n\
                     7 A, 1 B => 1 C\n\
                     7 A, 1 C => 1 D\n\
                     7 A, 1 D => 1 E\n\
                     7 A, 1 E => 1 FUEL"
            .lines()
            .map(|l| parse_rule(l))
            .collect::<Vec<_>>();
        let res = Day14::part_1(&input);
        assert_eq!(res, 31);
    }

    #[test]
    fn test_part_1_example_2() {
        let input = "9 ORE => 2 A\n\
                     8 ORE => 3 B\n\
                     7 ORE => 5 C\n\
                     3 A, 4 B => 1 AB\n\
                     5 B, 7 C => 1 BC\n\
                     4 C, 1 A => 1 CA\n\
                     2 AB, 3 BC, 4 CA => 1 FUEL"
            .lines()
            .map(|l| parse_rule(l))
            .collect::<Vec<_>>();
        let res = Day14::part_1(&input);
        assert_eq!(res, 165);
    }

    #[test]
    fn test_part_1_example_3() {
        let input = "171 ORE => 8 CNZTR\n\
                     7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
                     114 ORE => 4 BHXH\n\
                     14 VRPVC => 6 BMBT\n\
                     6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
                     6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
                     15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
                     13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
                     5 BMBT => 4 WPTQ\n\
                     189 ORE => 9 KTJDG\n\
                     1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
                     12 VRPVC, 27 CNZTR => 2 XDBXC\n\
                     15 KTJDG, 12 BHXH => 5 XCVML\n\
                     3 BHXH, 2 VRPVC => 7 MZWV\n\
                     121 ORE => 7 VRPVC\n\
                     7 XCVML => 6 RJRHP\n\
                     5 BHXH, 4 VRPVC => 5 LTCX"
            .lines()
            .map(|l| parse_rule(l))
            .collect::<Vec<_>>();
        let res = Day14::part_1(&input);
        assert_eq!(res, 2210736);
    }

    #[test]
    fn test_part_2_example_3() {
        let input = "171 ORE => 8 CNZTR\n\
                     7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
                     114 ORE => 4 BHXH\n\
                     14 VRPVC => 6 BMBT\n\
                     6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
                     6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
                     15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
                     13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
                     5 BMBT => 4 WPTQ\n\
                     189 ORE => 9 KTJDG\n\
                     1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
                     12 VRPVC, 27 CNZTR => 2 XDBXC\n\
                     15 KTJDG, 12 BHXH => 5 XCVML\n\
                     3 BHXH, 2 VRPVC => 7 MZWV\n\
                     121 ORE => 7 VRPVC\n\
                     7 XCVML => 6 RJRHP\n\
                     5 BHXH, 4 VRPVC => 5 LTCX"
            .lines()
            .map(|l| parse_rule(l))
            .collect::<Vec<_>>();
        let res = Day14::part_2(&input);
        assert_eq!(res, 460664);
    }
}
