use aoc_common::aoc_day::AocDay;
use aoc_common::files::Res;
use itertools::Itertools;
use std::fs::read_to_string;

pub struct Day09;

impl AocDay for Day09 {
    type Input = Vec<(i64, i64)>;
    type Result1 = i64;
    type Result2 = i64;

    fn day() -> usize {
        09
    }

    fn load() -> Res<Self::Input> {
        parse(&read_to_string("data/2025/day_09.in")?)
    }

    fn part_1(input: &Self::Input) -> Self::Result1 {
        input
            .iter()
            .enumerate()
            .map(|(i, (x1, y1))| {
                input
                    .iter()
                    .skip(i)
                    .map(|&(x2, y2)| (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1))
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .unwrap() as i64
    }

    fn part_2(input: &Self::Input) -> Self::Result2 {
        let mut lines = input.clone();
        lines.push(input[0]);
        let mut lines = lines.into_iter().tuple_windows().collect::<Vec<((i64, i64),(i64, i64))>>();
        // longer lines are more likely to intersect
        lines.sort_unstable_by_key(|&((x1, y1), (x2, y2))| {
            -((x1.abs_diff(x2) + y1.abs_diff(y2)) as i64)
        });

        let mut max = 0;

        input
            .iter()
            .enumerate()
            .for_each(|(i, &(x1, y1))| {
                input
                    .iter()
                    .skip(i + 1)
                    .for_each(|&(x2, y2)| {
                        let size = (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1);
                        if size <= max {
                            return;
                        }

                        let (min_x, max_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                        let (min_y, max_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };


                        if lines
                            .iter()
                            .any(|&((lx1, ly1), (lx2, ly2))| {
                               intersects((min_x, min_y), (max_x, max_y), ((lx1, ly1), (lx2, ly2)))
                            })
                        {
                            // if there's any intersecting line then this is invalid
                        } else {
                            max = size;
                        }
                    })
            });
        max as i64
    }
}

fn intersects((x_min, y_min): (i64, i64), (x_max, y_max): (i64, i64), line: ((i64, i64),(i64, i64))) -> bool {
    let ((lx1, ly1), (lx2, ly2)) = line;
    let (lmin_x, lmax_x) = if lx1 < lx2 { (lx1, lx2) } else { (lx2, lx1) };
    let (lmin_y, lmax_y) = if ly1 < ly2 { (ly1, ly2) } else { (ly2, ly1) };


    if lmin_x == lmax_x {
        // vertical line within x bound
        if lmin_x > x_min && lmin_x < x_max {
            // crosses top line
            if (lmin_y <= y_min && lmax_y > y_min)
                // crosses bottom line
                || (lmin_y < y_max && lmax_y >= y_max) {
                return true;
            }
        }
    } else if lmin_y == lmax_y {
        // horizontal line within y bounds
        if lmin_y > y_min && lmin_y < y_max {
            // crosses left line
            if (lmin_x <= x_min && lmax_x > x_min)
                // crosses right line
                || (lmin_x < x_max && lmax_x >= x_max) {
                return true;
            }
        }
    }
    false
}

fn parse(input: &str) -> Res<Vec<(i64, i64)>> {
    let res = input
        .lines()
        .map(|line| {
            let mut it = line.split(",").map(|n| n.parse::<i64>().unwrap());
            Ok((it.next().unwrap(), it.next().unwrap()))
        })
        .collect::<Res<Vec<(i64, i64)>>>()?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = parse(
            r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        )
        .unwrap();
        let res = Day09::part_1(&input);
        assert_eq!(res, 50);
    }

    #[test]
    fn part_2() {
        let input = parse(
            r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        )
        .unwrap();
        let res = Day09::part_2(&input);
        assert_eq!(res, 24);
    }

    #[test]
    fn test_intersects() {
        assert!(intersects((3, 3), (6, 6), ((1, 4), (7, 4))));
        assert!(intersects((3, 3), (6, 6), ((1, 4), (5, 4))));
        assert!(!intersects((3, 3), (6, 6), ((1, 2), (7, 2))));
        assert!(!intersects((2, 3), (9, 7), ((9, 7), (9, 5))));
        assert!(intersects((2, 3), (9, 7), ((9, 5), (2, 5))));
    }
}
