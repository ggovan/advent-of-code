use std::error::Error;
use std::fs::read_to_string;

type Res<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Asteroid {
    pos: Vector,
    vel: Vector,
}

type Vector = (i32, i32, i32);

fn parse(input: &str) -> Vec<Asteroid> {
    input
        .lines()
        .map(|l| {
            let x: i32;
            let y: i32;
            let z: i32;
            scan!(l.trim().bytes() => "<x={}, y={}, z={}>", x, y, z);
            Asteroid {
                pos: (x, y, z),
                vel: (0, 0, 0),
            }
        })
        .collect()
}

fn sim_step(input: &mut Vec<Asteroid>) {
    let new_velocities = input
        .iter()
        .map(|a| {
            input.iter().fold(a.vel, |acc, b| {
                (
                    acc.0 + vel_diff(a.pos.0, b.pos.0).0,
                    acc.1 + vel_diff(a.pos.1, b.pos.1).0,
                    acc.2 + vel_diff(a.pos.2, b.pos.2).0,
                )
            })
        })
        .collect::<Vec<_>>();

    for i in 0..input.len() {
        input[i].pos.0 = input[i].pos.0 + new_velocities[i].0;
        input[i].pos.1 = input[i].pos.1 + new_velocities[i].1;
        input[i].pos.2 = input[i].pos.2 + new_velocities[i].2;
        input[i].vel = new_velocities[i];
    }
}

fn vel_diff(a: i32, b: i32) -> (i32, i32) {
    match (a, b) {
        _ if a > b => (-1, 1),
        _ if a < b => (1, -1),
        _ => (0, 0),
    }
}

fn calc_energy(sys: &Vec<Asteroid>) -> i32 {
    sys.iter()
        .map(|a| {
            (a.pos.0.abs() + a.pos.1.abs() + a.pos.2.abs())
                * (a.vel.0.abs() + a.vel.1.abs() + a.vel.2.abs())
        })
        .sum()
}

fn until_stable(starting: &Vec<Asteroid>) -> i64 {
    let mut stable_x: Option<i64> = None;
    let mut stable_y: Option<i64> = None;
    let mut stable_z: Option<i64> = None;

    let start_x = starting
        .iter()
        .map(|a| (a.pos.0, a.vel.0))
        .collect::<Vec<_>>();
    let start_y = starting
        .iter()
        .map(|a| (a.pos.1, a.vel.1))
        .collect::<Vec<_>>();
    let start_z = starting
        .iter()
        .map(|a| (a.pos.2, a.vel.2))
        .collect::<Vec<_>>();

    let mut sys = starting.clone();
    let mut i = 0;

    loop {
        sim_step(&mut sys);
        i += 1;

        if stable_x.is_none() {
            if sys
                .iter()
                .map(|a| (a.pos.0, a.vel.0))
                .enumerate()
                .all(|(i, e)| e == start_x[i])
            {
                stable_x = Some(i);
            }
        }
        if stable_y.is_none() {
            if sys
                .iter()
                .map(|a| (a.pos.1, a.vel.1))
                .enumerate()
                .all(|(i, e)| e == start_y[i])
            {
                stable_y = Some(i);
            }
        }
        if stable_z.is_none() {
            if sys
                .iter()
                .map(|a| (a.pos.2, a.vel.2))
                .enumerate()
                .all(|(i, e)| e == start_z[i])
            {
                stable_z = Some(i);
            }
        }

        if stable_x.is_some() && stable_y.is_some() && stable_z.is_some() {
            break;
        }
    }

    let x = stable_x.unwrap();
    let y = stable_y.unwrap();
    let z = stable_z.unwrap();

    dbg!(x, y, z);

    lcm(z, lcm(x, y))
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcf(a, b)
}

fn gcf(a: i64, b: i64) -> i64 {
    match (a, b) {
        (a, 0) => a,
        (a, b) => gcf(b, a % b),
    }
}

pub fn day_12() -> Res<i32> {
    let input = read_to_string("data/2019/day_12.in")?;
    let parsed = parse(&input);
    let mut sys = parsed.clone();
    for _ in 0..1000 {
        sim_step(&mut sys)
    }
    let energy = calc_energy(&sys);
    println!("Day 12");
    println!("  part 1 {} ", energy);
    let stable_at = until_stable(&parsed);
    println!("  part 2 {} ", stable_at);
    Ok(energy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input() {
        let input = "<x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>";
        let res = parse(input);
        assert_eq!(
            res,
            vec![
                Asteroid {
                    pos: (-1, 0, 2),
                    vel: (0, 0, 0)
                },
                Asteroid {
                    pos: (2, -10, -7),
                    vel: (0, 0, 0)
                },
                Asteroid {
                    pos: (4, -8, 8),
                    vel: (0, 0, 0)
                },
                Asteroid {
                    pos: (3, 5, -1),
                    vel: (0, 0, 0)
                },
            ]
        );
    }

    #[test]
    fn parse_input_and_sim_step() {
        let input = "<x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>";
        let mut res = parse(input);

        sim_step(&mut res);

        assert_eq!(
            res,
            vec![
                Asteroid {
                    pos: (2, -1, 1),
                    vel: (3, -1, -1)
                },
                Asteroid {
                    pos: (3, -7, -4),
                    vel: (1, 3, 3)
                },
                Asteroid {
                    pos: (1, -7, 5),
                    vel: (-3, 1, -3)
                },
                Asteroid {
                    pos: (2, 2, 0),
                    vel: (-1, -3, 1)
                },
            ]
        );
    }

    #[test]
    fn parse_input_and_sim_step_10() {
        let input = "<x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>";
        let mut res = parse(input);

        for _ in 0..10 {
            sim_step(&mut res);
        }

        assert_eq!(
            res,
            vec![
                Asteroid {
                    pos: (2, 1, -3),
                    vel: (-3, -2, 1)
                },
                Asteroid {
                    pos: (1, -8, 0),
                    vel: (-1, 1, 3)
                },
                Asteroid {
                    pos: (3, -6, 1),
                    vel: (3, 2, -3)
                },
                Asteroid {
                    pos: (2, 0, 4),
                    vel: (1, -1, -1)
                },
            ]
        );
    }

    #[test]
    fn calc_energy_after_10() {
        let input = "<x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>";
        let mut res = parse(input);

        for _ in 0..10 {
            sim_step(&mut res);
        }

        assert_eq!(calc_energy(&res), 179);
    }

    #[test]
    fn test_stable() {
        let input = "<x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>";
        let res = parse(input);

        let stable = until_stable(&res);

        assert_eq!(stable, 2772);
    }
}
