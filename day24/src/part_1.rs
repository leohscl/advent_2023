use std::str::FromStr;

use derive_more::Add;
use derive_more::Mul;
use derive_more::Sub;

#[derive(Debug, Clone, PartialEq, PartialOrd, Sub, Mul, Add)]
struct Point<T> {
    x: T,
    y: T,
    // z: f64,
}

struct Area {
    min: f64,
    max: f64,
}

impl Point<i64> {
    fn is_parallel(&self, other: &Point<i64>) -> bool {
        if other.x == 0 || other.y == 0 {
            panic!();
        }
        self.x * other.x == self.y * other.y
    }
}
impl Point<f64> {
    fn in_area(&self, area: &Area) -> bool {
        area.min <= self.x && self.x <= area.max && area.min <= self.y && self.y <= area.max
    }
}

#[derive(Debug)]
struct Hailstone {
    start: Point<i64>,
    trajectory: Point<i64>,
}

impl FromStr for Hailstone {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter_num = s.split(" @ ").flat_map(|s| {
            s.split([',', ' '])
                .filter(|t| t != &"")
                .map(|n| n.parse::<i64>().unwrap())
        });
        let (xp, yp, _zp) = (
            iter_num.next().unwrap(),
            iter_num.next().unwrap(),
            iter_num.next().unwrap(),
        );
        let (xt, yt, _zt) = (
            iter_num.next().unwrap(),
            iter_num.next().unwrap(),
            iter_num.next().unwrap(),
        );
        let start = Point { x: xp, y: yp };
        let trajectory = Point { x: xt, y: yt };
        Ok(Hailstone { start, trajectory })
    }
}
impl Hailstone {
    fn intersect(&self, other: &Hailstone) -> Option<Point<f64>> {
        if self.trajectory.is_parallel(&other.trajectory) {
            println!("parallel paths");
            return None;
        }
        let x1 = self.start.x as f64;
        let x2 = other.start.x as f64;
        let y1 = self.start.y as f64;
        let y2 = other.start.y as f64;
        let v1x = self.trajectory.x as f64;
        let v2x = other.trajectory.x as f64;
        let v1y = self.trajectory.y as f64;
        let v2y = other.trajectory.y as f64;
        let t2_num = x2 - x1 + (y1 - y2) * v1x / v1y;
        let t2_den = v2y / v1y * v1x - v2x;
        let t2 = t2_num / t2_den;
        let x_cross = x2 + t2 * v2x;
        let y_cross = y2 + t2 * v2y;
        let t1 = (y2 + t2 * v2y - y1) / v1y;
        if t2 < 0f64 || t1 < 0f64 {
            println!("Past");
            return None;
        }
        // let x_cross = x1 + t1 * v1x;
        // let y_cross = y1 + t1 * v1y;
        // dbg!(x_cross);
        // dbg!(y_cross);
        Some(Point {
            x: x_cross,
            y: y_cross,
        })
    }
}

fn main() {
    let input = include_str!("../input_test.txt");
    // let input = include_str!("../input.txt");

    let hailstones: Vec<Hailstone> = input
        .split_terminator('\n')
        .map(|s| Hailstone::from_str(s).unwrap())
        .collect();

    dbg!(&hailstones);

    let area = Area {
        min: 7f64,
        max: 27f64,
    };

    // let area = Area {
    //     min: 200000000000000f64,
    //     max: 400000000000000f64,
    // };

    let sum_in_area: i64 = (0..hailstones.len())
        .map(|h_i| {
            (h_i + 1..hailstones.len())
                .map(|h_j| {
                    if let Some(p) = hailstones[h_i].intersect(&hailstones[h_j]) {
                        if p.in_area(&area) {
                            // dbg!(p);
                            // println!("inside");
                            1
                        } else {
                            // dbg!(p);
                            // println!("outside");
                            0
                        }
                    } else {
                        0
                    }
                })
                .sum::<i64>()
        })
        .sum::<i64>();
    dbg!(sum_in_area);
}
