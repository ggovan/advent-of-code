use aoc_common::aoc_day::AocDay;
use aoc_common::files::Res;
use std::fs::read_to_string;

pub struct Day16;

impl AocDay for Day16 {
    type Input = (Vec<Rule>, Ticket, Vec<Ticket>);
    type Result1 = u64;
    type Result2 = u64;

    fn day() -> usize {
        16
    }
    fn load() -> Res<Self::Input> {
        let input = read_to_string("data/2020/day_16.in")?;
        let mut iter = input.split("\n\n");
        let rules = iter
            .next()
            .unwrap()
            .lines()
            .map(parse_rule)
            .collect::<Vec<_>>();
        let my_ticket = parse_ticket(iter.next().unwrap().lines().nth(1).unwrap());
        let tickets = iter
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .map(parse_ticket)
            .collect::<Vec<_>>();
        Ok((rules, my_ticket, tickets))
    }

    fn part_1((rules, _, tickets): &Self::Input) -> Self::Result1 {
        tickets
            .iter()
            .map(|t| {
                t.iter()
                    .filter(|&v| !rules.iter().any(|r| r.valid(*v)))
                    .sum::<u64>()
            })
            .sum()
    }

    fn part_2((rules, mine, tickets): &Self::Input) -> Self::Result2 {
        let valid_tickets = tickets
            .iter()
            .filter(|t| t.iter().all(|&v| rules.iter().any(|r| r.valid(v))))
            .collect::<Vec<_>>();

        let mut possible_rules_in_position: [usize; 0x14] = [0b11111111111111111111; 20];

        // get rid of all the rules that don't fit a column
        for (col, c_rs) in possible_rules_in_position.iter_mut().enumerate() {
            for (r_i, r) in rules.iter().enumerate() {
                if !valid_tickets.iter().all(|t| r.valid(t[col])) {
                    *c_rs &= !(1 << r_i);
                }
            }
        }

        loop {
            for col in 0..possible_rules_in_position.len() {
                let p_rs = possible_rules_in_position[col];
                if p_rs.count_ones() == 1 {
                    // remove this from all others
                    for r in possible_rules_in_position.iter_mut() {
                        if p_rs == *r {
                            continue;
                        }
                        *r &= !p_rs;
                    }
                }
            }

            if possible_rules_in_position
                .iter()
                .all(|rs| rs.count_ones() == 1)
            {
                break;
            }
        }

        possible_rules_in_position
            .iter()
            .enumerate()
            .map(|(i, r)| (i, &rules[r.trailing_zeros() as usize]))
            .filter(|(_, r)| r.name.starts_with("departure"))
            .map(|(i, _)| mine[i])
            .product()
    }
}

fn parse_rule(s: &str) -> Rule {
    let (name, cs) = s.split_once(": ").unwrap();
    let (c1, c2) = cs.split_once(" or ").unwrap();
    let (c1_min, c1_max) = c1.split_once('-').unwrap();
    let (c2_min, c2_max) = c2.split_once('-').unwrap();
    Rule {
        name: name.to_owned(),
        clause_1: (c1_min.parse().unwrap(), c1_max.parse().unwrap()),
        clause_2: (c2_min.parse().unwrap(), c2_max.parse().unwrap()),
    }
}

type Clause = (u64, u64);

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    name: String,
    clause_1: Clause,
    clause_2: Clause,
}
impl Rule {
    fn valid(&self, v: u64) -> bool {
        (self.clause_1.0..=self.clause_1.1).contains(&v)
            || (self.clause_2.0..=self.clause_2.1).contains(&v)
    }
}

type Ticket = [u64; 20];

fn parse_ticket(s: &str) -> Ticket {
    let mut ticket = [0; 20];
    for (i, v) in s.split(',').enumerate() {
        ticket[i] = v.parse().unwrap();
    }
    ticket
}
