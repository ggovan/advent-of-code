use aoc_common::aoc_day::AocDay;
use aoc_common::files::Res;
use aoc_common::time_block;
use std::{fs::read_to_string, mem::swap};

pub struct Day08;

impl AocDay for Day08 {
    type Input = Vec<(i64, i64, i64)>;
    type Result1 = i64;
    type Result2 = i64;

    fn day() -> usize {
        8
    }

    fn load() -> Res<Self::Input> {
        let _input = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let input = read_to_string("data/2025/day_08.in")?;
        Ok(input
            .split("\n")
            .map(|line| {
                let mut it = line.split(",").map(|n| n.parse::<i64>().unwrap());
                (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
            })
            .collect::<Self::Input>())
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        let len = input.len();
        let mut edges: Vec<(i64, usize, usize)> = Vec::with_capacity(len * (len - 1) / 2);

        for i in 0..len - 1 {
            let (x1, y1, z1) = input[i];
            for j in (i + 1)..len {
                let (x2, y2, z2) = input[j];
                let dist = (x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2);
                edges.push((dist, i, j));
            }
        }

        edges.sort_unstable_by_key(|(d, _, _)| *d);

        // Vec<Vec<usize>> isn't great. A Vec<BitSet> would be better, but I only have a 64 bit BitSet.
        let mut networks: Vec<Vec<usize>> = vec![];
        let mut connections = if len == 20 { 10 } else { 1000 };

        for edge in edges {
            let (_d, i, j) = edge;
            let ni = networks.iter().position(|nodes| nodes.contains(&i));
            let nj = networks.iter().position(|nodes| nodes.contains(&j));
            match (ni, nj) {
                (None, None) => {
                    networks.push(vec![i, j]);
                    connections -= 1;
                }
                (Some(ni), None) => {
                    networks[ni].push(j);
                    connections -= 1;
                }
                (None, Some(nj)) => {
                    networks[nj].push(i);
                    connections -= 1;
                }
                (Some(mut ni), Some(mut nj)) => {
                    if ni != nj {
                        if nj < ni {
                            swap(&mut ni, &mut nj);
                        }
                        let mut nodes2 = networks.remove(nj);
                        networks[ni].append(&mut nodes2);
                        connections -= 1;
                    } else {
                        connections -= 1;
                    }
                }
            }
            if connections == 0 {
                break;
            }
        }

        networks.sort_unstable_by_key(|v| -(v.len() as i64));

        networks.iter().take(3).map(|v| v.len() as i64).product()
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        let len = input.len();
        // using a vector and then sorting is just as fast as a binary heap
        let mut edges: Vec<(i64, usize, usize)> = Vec::with_capacity(len * (len - 1) / 2);

        {
            let _t = time_block("build edges");
            for i in 0..len - 1 {
                let (x1, y1, z1) = input[i];
                for j in (i + 1)..len {
                    let (x2, y2, z2) = input[j];
                    let dist = (x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2);
                    assert!(dist >= 0);
                    edges.push((dist, i, j));
                }
            }
        }

        {
            let _t = time_block("sort edges");
            edges.sort_unstable_by_key(|(d, _, _)| *d);
        }

        let mut connections = if len == 20 { 19 } else { 999 };
        let mut final_edge: Option<(usize, usize)> = None;

        let mut networks_seen = 0;
        let mut network_by_node: Vec<Option<usize>> = vec![None; len];

        {
            let _t = time_block("build network");
            for edge in edges {
                let (_d, i, j) = edge;

                let ni = network_by_node[i];
                let nj = network_by_node[j];
                match (ni, nj) {
                    (None, None) => {
                        networks_seen += 1;
                        network_by_node[i] = Some(networks_seen);
                        network_by_node[j] = Some(networks_seen);
                        connections -= 1;
                    }
                    (Some(ni), None) => {
                        network_by_node[j] = Some(ni);
                        connections -= 1;
                    }
                    (None, Some(nj)) => {
                        network_by_node[i] = Some(nj);
                        connections -= 1;
                    }
                    (Some(mut ni), Some(mut nj)) => {
                        if ni != nj {
                            if nj < ni {
                                swap(&mut ni, &mut nj);
                            }
                            network_by_node.iter_mut().for_each(|n| {
                                if let Some(net_idx) = n {
                                    if *net_idx == nj {
                                        *n = Some(ni);
                                    }
                                }
                            });
                            connections -= 1;
                        } else {
                            // no new connection made
                            // connections-=1;
                        }
                    }
                }
                if connections == 0 {
                    final_edge = Some((i, j));
                    break;
                }
            }
        }

        let final_edge = final_edge.unwrap();
        // 8079278220
        input[final_edge.0].0 * input[final_edge.1].0
    }
}
