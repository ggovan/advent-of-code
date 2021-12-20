use aoc_common::aoc_day::AocDay;
use aoc_common::files::Res;
use std::fs::read_to_string;

pub struct Day16;

impl AocDay for Day16 {
    type Input = Vec<u16>;
    type Result1 = u64;
    type Result2 = u64;

    fn day() -> usize {
        16
    }
    fn load() -> Res<Self::Input> {
        let input_str = read_to_string("data/2021/day_16.in")?;

        Ok(input_str
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| c.to_digit(16).unwrap() as u16)
            .collect())
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        parse_packet(input, 0).0
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        parse_packet(input, 0).1
    }
}

fn parse_packet(input: &<Day16 as AocDay>::Input, start: usize) -> (u64, u64, usize) {
    let packet_version = read(input, start, 3);
    let operator = read(input, start + 3, 3);
    if operator == 4 {
        // read_literal
        let mut val = 0;
        let mut i = 0;
        loop {
            let next = read(input, start + 6 + i * 5, 5);
            val <<= 4;
            val |= next & 0b1111;
            if next & 0b1_0000 == 0 {
                return (packet_version, val, start + 6 + i * 5 + 5);
            }
            i += 1;
        }
    } else {
        let length_type = read(input, start + 6, 1);
        let mut vec: Vec<u64> = vec![];
        let mut pvs = packet_version;

        let end = if length_type == 0 {
            let length = read(input, start + 7, 15) as usize;
            let mut sub_start = start + 7 + 15;
            while sub_start < start + 7 + 15 + length {
                let (pv, v, ss) = parse_packet(input, sub_start);
                pvs += pv;
                sub_start = ss;
                vec.push(v)
            }
            start + 7 + 15 + length
        } else {
            let sub_pac_count = read(input, start + 7, 11);
            let mut sub_start = start + 7 + 11;
            for _ in 0..sub_pac_count {
                let (p_v, v, len) = parse_packet(input, sub_start);
                sub_start = len;
                pvs += p_v;
                vec.push(v);
            }
            sub_start
        };

        let v: u64 = match operator {
            0 => vec.iter().sum(),
            1 => vec.iter().product(),
            2 => *vec.iter().min().unwrap(),
            3 => *vec.iter().max().unwrap(),
            5 => {
                if vec[0] > vec[1] {
                    1
                } else {
                    0
                }
            }
            6 => {
                if vec[0] < vec[1] {
                    1
                } else {
                    0
                }
            }
            7 => {
                if vec[0] == vec[1] {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!("unknown"),
        };

        (pvs, v, end)
    }
}

fn read(input: &<Day16 as AocDay>::Input, start: usize, len: usize) -> u64 {
    let mut result = 0u64;
    for i in start..(start + len) {
        result <<= 1;
        result |= (input[i / 4] >> (3 - (i % 4)) & 1) as u64;
    }
    result
}
