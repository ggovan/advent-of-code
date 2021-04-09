use aoc_common::files::Res;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

type Pos2D = (/*x*/ i32, /*y*/ i32);
type RationalVector = (
    i32, // dx - rationalised
    i32, // dy - rationalised
    i32, // magnitude
    f32, // angle
);

fn max_asteroids(input: &str) -> (i32, i32, i32) {
    let asteroids = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_x, p)| *p == '#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect::<Vec<_>>();
    let aster_ref = &asteroids;

    asteroids
        .iter()
        .map(move |&coord| count_visible(coord, aster_ref))
        .max_by_key(|(_, _, n)| *n)
        .unwrap()
}

fn asteroid_shooting(input: &str, (x, y): Pos2D, goal: usize) -> Pos2D {
    let asteroids = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_x, p)| *p == '#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect::<Vec<_>>();
    let aster_ref = &asteroids;

    let mut angles: HashMap<Pos2D, Vec<RationalVector>> = HashMap::new();

    for (x2, y2) in aster_ref {
        if *x2 == x && *y2 == y {
            continue;
        }
        let (rx, ry, m, a) = rationalize((x2 - x, -(y2 - y)));
        if let Some(vec) = angles.get_mut(&(rx, ry)) {
            vec.push((*x2, *y2, m, a));
        } else {
            angles.insert((rx, ry), vec![(*x2, *y2, m, a)]);
        }
    }

    angles
        .values_mut()
        .for_each(|v| v.sort_by_key(|point| point.2));

    let mut rays = angles
        .keys()
        .map(|k| (k.0, k.1, angles[k][0].3))
        .collect::<Vec<_>>();
    rays.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    assert_eq!(count_visible((x, y), &asteroids).2, rays.len() as i32);
    assert_eq!(rays[0], (0, 1, 0.0));

    let mut circuit = 0;
    let mut hits = 0;

    let coord = 'outer: loop {
        for ray in &rays {
            if let Some(r) = angles[&(ray.0, ray.1)].get(circuit) {
                hits += 1;
                if hits == goal {
                    break 'outer r;
                }
            }
        }
        circuit += 1;
    };

    (coord.0, coord.1)
}

fn count_visible((x, y): Pos2D, asteroids: &[Pos2D]) -> (i32, i32, i32) {
    let visible = asteroids
        .iter()
        .map(move |(x2, y2)| (x2 - x, y2 - y))
        .filter(|(x, y)| *x != 0 || *y != 0)
        .map(|coord| {
            let (rx, ry, _d, _a) = rationalize(coord);
            (rx, ry)
        })
        .collect::<HashSet<_>>();
    (x, y, visible.len() as i32)
}

fn rationalize(coord: Pos2D) -> RationalVector {
    match coord {
        (x, 0) if x > 0 => (1, 0, x, 90.0),
        (x, 0) if x < 0 => (-1, 0, x.abs(), 270.0),
        (0, y) if y > 0 => (0, 1, y, 0.0),
        (0, y) if y < 0 => (0, -1, y.abs(), 180.0),
        (x, y) => {
            let f = gcf(x, y).abs();
            let angle = (x as f32).atan2(y as f32).to_degrees();
            let angle = match angle {
                a if a < 0.0 => 360.0 + a,
                a => a,
            };
            (x / f, y / f, x.abs() + y.abs(), angle)
        }
    }
}

fn gcf(a: i32, b: i32) -> i32 {
    match (a, b) {
        (a, 0) => a,
        (a, b) => gcf(b, a % b),
    }
}

pub fn day_10() -> Res<((i32, i32, i32), Pos2D)> {
    let day_10_in: String = read_to_string("data/2019/day_10.in")?;
    println!("Day 10");
    let res_1 = max_asteroids(&day_10_in);
    println!(" part 1 {:?}", res_1);

    let res_2 = asteroid_shooting(&day_10_in, (res_1.0, res_1.1), 200);
    println!(" part 2 {:?}", res_2);

    Ok((res_1, res_2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = ".#..#\n\
                     .....\n\
                     #####\n\
                     ....#\n\
                     ...##";
        let res = max_asteroids(input);
        assert_eq!(res, (3, 4, 8));
    }

    #[test]
    fn example_5() {
        let input = ".#..##.###...#######\n\
                     ##.############..##.\n\
                     .#.######.########.#\n\
                     .###.#######.####.#.\n\
                     #####.##.#.##.###.##\n\
                     ..#####..#.#########\n\
                     ####################\n\
                     #.####....###.#.#.##\n\
                     ##.#################\n\
                     #####.##.###..####..\n\
                     ..######..##.#######\n\
                     ####.##.####...##..#\n\
                     .#####..#.######.###\n\
                     ##...#.##########...\n\
                     #.##########.#######\n\
                     .####.#.###.###.#.##\n\
                     ....##.##.###..#####\n\
                     .#.#.###########.###\n\
                     #.#.#.#####.####.###\n\
                     ###.##.####.##.#..##";
        let res = max_asteroids(input);
        assert_eq!(res, (11, 13, 210));
    }

    #[test]
    fn day_10_test() -> Res<()> {
        let res = day_10()?;
        assert_eq!(res, ((31, 20, 319), (5, 17)));
        Ok(())
    }

    #[test]
    fn example_6() {
        let input = ".#....#####...#..\n\
                     ##...##.#####..##\n\
                     ##...#...#.#####.\n\
                     ..#.....#...###..\n\
                     ..#.#.....#....##";
        let res = asteroid_shooting(input, (8, 3), 30);
        assert_eq!(res, (7, 0));
    }

    #[test]
    fn example_5_2() {
        let input = ".#..##.###...#######\n\
                     ##.############..##.\n\
                     .#.######.########.#\n\
                     .###.#######.####.#.\n\
                     #####.##.#.##.###.##\n\
                     ..#####..#.#########\n\
                     ####################\n\
                     #.####....###.#.#.##\n\
                     ##.#################\n\
                     #####.##.###..####..\n\
                     ..######..##.#######\n\
                     ####.##.####...##..#\n\
                     .#####..#.######.###\n\
                     ##...#.##########...\n\
                     #.##########.#######\n\
                     .####.#.###.###.#.##\n\
                     ....##.##.###..#####\n\
                     .#.#.###########.###\n\
                     #.#.#.#####.####.###\n\
                     ###.##.####.##.#..##";
        let res = asteroid_shooting(input, (11, 13), 200);
        assert_eq!(res, (8, 2));
    }
}
