use aoc_common::aoc_day::AocDay;
use aoc_common::bitset::Bitset;
use aoc_common::files::Res;
use std::collections::VecDeque;
use std::fs::read_to_string;
use tinyvec::ArrayVec;

pub struct Day10;

type TinyVec = tinyvec::ArrayVec<[usize; 10]>;
type TV64 = tinyvec::ArrayVec<[i64; 10]>;

impl AocDay for Day10 {
    type Input = Vec<(Bitset, Vec<Bitset>, Vec<TinyVec>, TV64)>;
    type Result1 = i64;
    type Result2 = i64;

    fn day() -> usize {
        10
    }

    fn load() -> Res<Self::Input> {
        Ok(parse(&read_to_string("data/2025/day_10.in")?))
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        // hashbrown so that 3ms -> 1ms
        let mut seen: hashbrown::HashSet<Bitset> = hashbrown::HashSet::with_capacity(1024);
        let mut queue: VecDeque<(Bitset, usize)> = VecDeque::with_capacity(128);

        input
            .iter()
            .map(|(goal, switches, _, _)| {
                seen.clear();
                queue.clear();
                seen.insert(Bitset::empty());
                queue.push_back((Bitset::empty(), 0));

                // TODO, this but don't push a button twice

                while let Some((current, d)) = queue.pop_front() {
                    for &switch in switches {
                        let switched = current.disjoint(switch);
                        if switched == *goal {
                            return d as i64 + 1;
                        }
                        if seen.insert(switched) {
                            queue.push_back((switched, d + 1));
                        }
                    }
                }

                unreachable!();
            })
            .sum()
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        return -1;
        // possible optimisations
        // - are there any indices that are only ever in a single switch?
        //   - if so, we can just apply that switch the required number of times
        // - are there any single index switches?
        //   - if so, we can solve for the other numbers, and add that later
        // - order joltage targets by number of switches affecting them asc?
        //   - order of switching doesn't mater, only the number of times each switch was used
        // - is there an index that is set everytime another index is?
        //   - if so, we can ... what?
        // - we can remove all rules that build the smallest joltage

        let mut sum = 0;
        for machine in input {
            println!("New machine");
            let (_, _, switches_vec, joltage) = machine;
            let mut switches_vec = switches_vec.clone();
            let mut start: TV64 = joltage.iter().map(|_x| 0).collect();
            let mut preopt_distance = 0;

            println!("  joltage width: {:?}", joltage.len());

            preopt_distance += remove_least_common_switches(&mut switches_vec, joltage, &mut start);

            let most_common = joltage
                .iter()
                .enumerate()
                .map(|(i, &_j)| {
                    (
                        i,
                        switches_vec
                            .iter()
                            .filter(|switch_vec| switch_vec.contains(&i))
                            .count(),
                    )
                })
                .max_by_key(|&(_i, c)| c)
                .unwrap();

            println!(
                "  most common index: {}, count: {}, joltage: {}",
                most_common.0, most_common.1, joltage[most_common.0]
            );

            let single_switch = switches_vec
                .iter()
                .filter(|switch_vec| switch_vec.len() == 1)
                .map(|switch_vec| switch_vec[0])
                .collect::<Vec<usize>>();
            println!("  single index switches: {:?}", single_switch);

            println!("  starting search: {:?}", start);

            let res = (0, 0);
            // let res = search::search(
            //     start,
            //     |state| state == joltage,
            //     |state, distance| {
            //         switches_vec.iter().filter_map(move |switch_vec| {
            //             let mut new_state = state.clone();
            //             for &switch in switch_vec {
            //                 new_state[switch] += 1;
            //             }
            //             if joltage.iter().zip(new_state.iter()).any(|(j, s)| s > j) {
            //                 return None;
            //             }
            //             let heuristic = 0;
            //             let heuristic = joltage
            //                 .iter()
            //                 .zip(new_state.iter())
            //                 .map(|(j, s)| j.abs_diff(*s))
            //                 .max()
            //                 .unwrap();
            //             Some(search::HeapElem {
            //                 elem: new_state,
            //                 distance: distance + 1,
            //                 heuristic: heuristic as u64,
            //             })
            //         })
            //     },
            // );
            sum += res.1 as i64 + preopt_distance;
        }
        sum
    }
}

fn remove_least_common_switches(
    switches_vec: &mut Vec<TinyVec>,
    joltage: &TV64,
    start: &mut TV64,
) -> i64 {
    let mut total_distance = 0;
    let mut removed_indices: TinyVec = tinyvec::array_vec!([usize; 10]);

    loop {
        let least_common = joltage
            .iter()
            .enumerate()
            .map(|(i, &_j)| {
                (
                    i,
                    switches_vec
                        .iter()
                        .filter(|switch_vec| switch_vec.contains(&i))
                        .count(),
                )
            })
            .filter(|(i, _)| !removed_indices.contains(i))
            .min_by_key(|&(_i, c)| c)
            .unwrap();

        // if least_common.1 > 1 {
        //     break;
        // }

        println!(
            "  least common index: {}, count: {}, joltage: {}",
            least_common.0, least_common.1, joltage[least_common.0]
        );

        if least_common.1 > 1 {
            break;
        }

        let s = switches_vec
            .iter()
            .find(|switch_vec| switch_vec.contains(&least_common.0))
            .unwrap();

        let j = joltage[least_common.0];
        for i in s.iter() {
            start[*i] += j;
        }
        total_distance += j;

        println!("    removing switch {:?}, increasing distance by {}", s, j);

        removed_indices.push(least_common.0);
        switches_vec.retain(|switch_vec| !switch_vec.contains(&least_common.0));
    }

    total_distance
}

fn parse(s: &str) -> Vec<(Bitset, Vec<Bitset>, Vec<TinyVec>, TV64)> {
    s.split('\n')
        .map(|l| {
            let mut it = l.split(' ');
            let mut goal: Bitset = Bitset::empty();
            let goals = it.next().unwrap();
            goals.bytes().skip(1).enumerate().for_each(|(i, b)| {
                goal = match b {
                    b'#' => goal.set(i),
                    _ => goal,
                };
            });

            let switches_bs: Vec<Bitset> = it
                .filter_map(|xs| {
                    if xs.starts_with('{') {
                        return None;
                    }
                    let bs = xs
                        .trim_prefix('(')
                        .trim_suffix(')')
                        .split(',')
                        .map(|x| x.parse::<usize>().unwrap())
                        .fold(Bitset::empty(), |acc, x| acc.set(x));
                    Some(bs)
                })
                .collect();

            let switches_vec: Vec<TinyVec> = l
                .split(' ')
                .skip(1)
                .filter_map(|xs| {
                    if xs.starts_with('{') {
                        return None;
                    }
                    let vs: ArrayVec<[usize; 10]> = xs
                        .trim_prefix('(')
                        .trim_suffix(')')
                        .split(',')
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect();
                    Some(vs)
                })
                .collect();

            let joltage: TV64 = l
                .split('{')
                .nth(1)
                .unwrap()
                .trim_end_matches('}')
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect();

            (goal, switches_bs, switches_vec, joltage)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::vec;
    use tinyvec::array_vec;

    use super::*;

    #[test]
    fn parse_input() {
        let input = parse(
            r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );
        assert_eq!(
            input[..1],
            vec![(
                Bitset::from(0b0110),
                vec![
                    Bitset::from(0b1000),
                    Bitset::from(0b1010),
                    Bitset::from(0b0100),
                    Bitset::from(0b1100),
                    Bitset::from(0b0101),
                    Bitset::from(0b0011),
                ],
                vec![
                    array_vec!([usize; 10] => 3),
                    array_vec!([usize; 10] => 1, 3),
                    array_vec!([usize; 10] => 2),
                    array_vec!([usize; 10] => 2, 3),
                    array_vec!([usize; 10] => 0, 2),
                    array_vec!([usize; 10] => 0, 1),
                ],
                array_vec!([i64; 10] => 3, 5, 4, 7),
            ),]
        );
    }

    #[test]
    fn part_1() {
        let input = parse(
            r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );
        let res = Day10::part_1(&input);
        assert_eq!(res, 7);
    }

    #[test]
    fn part_2() {
        let input = parse(
            r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );
        let res = Day10::part_2(&input);
        assert_eq!(res, 5); //33);
    }
}
