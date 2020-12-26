use crate::files::{read_better, Res};

pub fn day_3() -> Res<()> {
    println!("Day 3");
    let input = read_better("data/2019/day_3.in", &LineSeg::parse)?;
    let wires = input.map(|ss| Wire::from_segments(&ss)).collect::<Vec<_>>();

    let intersections = Wire::intersection_points(&wires[0], &wires[1], true);
    let min = intersections
        .iter()
        .map(|Point(x, y)| x.abs() + y.abs())
        .filter(|x| *x > 0)
        .min()
        .unwrap();
    println!("  part 1 {}", min);

    let min_dist = intersections
        .iter()
        .map(|&p| wires[0].distance_to(p) + wires[1].distance_to(p))
        .filter(|x| *x > 0)
        .min()
        .unwrap();
    println!("  part 2 {}", min_dist);
    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Point(i32, i32);

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    H,
    V,
}

#[derive(Debug)]
struct LineSeg {
    start: Point,
    length: i32,
    direction: Direction,
}

#[derive(Debug)]
struct Wire {
    segments: Vec<LineSeg>,
}

impl LineSeg {
    fn parse(input: &str) -> Self {
        let (direction, multiplier) = match input.chars().next().unwrap() {
            'R' => (Direction::H, 1),
            'L' => (Direction::H, -1),
            'U' => (Direction::V, 1),
            'D' => (Direction::V, -1),
            x => panic!("Unexpected direction '{}'", x),
        };
        let length: i32 = input.get(1..).unwrap().parse().unwrap();
        LineSeg {
            start: Point(0, 0),
            length: length * multiplier,
            direction,
        }
    }

    fn end_point(&self) -> Point {
        let Point(x, y) = self.start;
        match self.direction {
            Direction::H => Point(x + self.length, y),
            Direction::V => Point(x, y + self.length),
        }
    }

    fn is_point_on_line(&self, point: Point) -> bool {
        match self.direction {
            Direction::H => {
                let mut xs = [self.start.0, self.end_point().0];
                xs.sort_unstable();
                point.0 >= xs[0] && point.0 <= xs[1] && point.1 == self.start.1
            }
            Direction::V => {
                let mut ys = [self.start.1, self.end_point().1];
                ys.sort_unstable();
                point.1 >= ys[0] && point.1 <= ys[1] && point.0 == self.start.0
            }
        }
    }

    fn distance_to_point_on_line(&self, point: Point) -> i32 {
        match self.direction {
            Direction::H => (point.0 - self.start.0).abs(),
            Direction::V => (point.1 - self.start.1).abs(),
        }
    }

    fn find_intersection(seg1: &LineSeg, seg2: &LineSeg) -> Option<Point> {
        match (seg1.direction, seg2.direction) {
            (Direction::H, Direction::V) => {
                let point = Point(seg2.start.0, seg1.start.1);
                if seg1.is_point_on_line(point) && seg2.is_point_on_line(point) {
                    Some(point)
                } else {
                    None
                }
            }
            (Direction::V, Direction::H) => {
                let point = Point(seg1.start.0, seg2.start.1);
                if seg1.is_point_on_line(point) && seg2.is_point_on_line(point) {
                    Some(point)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Wire {
    fn from_segments(segs: &[LineSeg]) -> Self {
        let mut pos = Point(0, 0);
        let mut positioned_segs: Vec<LineSeg> = Vec::new();

        for seg in segs {
            let new_seg = LineSeg { start: pos, ..*seg };
            pos = new_seg.end_point();
            positioned_segs.push(new_seg);
        }
        Wire {
            segments: positioned_segs,
        }
    }

    fn intersection_points(w1: &Wire, w2: &Wire, include_edge: bool) -> Vec<Point> {
        let mut res = Vec::new();
        for seg1 in &w1.segments {
            for seg2 in &w2.segments {
                if let Some(point) = LineSeg::find_intersection(seg1, seg2) {
                    if include_edge {
                        res.push(point)
                    } else {
                        // i.e. we check if this is the edge of one of the lines
                        // added for day 17
                        if point == seg1.start
                            || point == seg1.end_point()
                            || point == seg2.start
                            || point == seg2.end_point()
                        {
                            // don't count it;
                        } else {
                            res.push(point)
                        }
                    }
                }
            }
        }
        res
    }

    fn distance_to(&self, point: Point) -> i32 {
        let mut dist = 0;
        for seg in &self.segments {
            if seg.is_point_on_line(point) {
                dist += seg.distance_to_point_on_line(point);
                break;
            } else {
                dist += seg.length.abs()
            }
        }
        dist
    }
}
