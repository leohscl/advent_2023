use std::str::FromStr;

use derive_more::Add;
use derive_more::Mul;
use derive_more::Sub;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Sub, Mul, Add, Ord)]
struct Point {
    x: i32,
    y: i32,
    // z: i32,
}

struct Area {
    c1: Point,
    c2: Point,
}

impl Point {
    fn checked_div(&self, other: &Point) -> Option<Point> {
        let x = self.x.checked_div(other.x)?;
        let y = self.y.checked_div(other.y)?;
        // let z = self.z.checked_div(other.z)?;
        // Some(Point { x, y, z })
        Some(Point { x, y })
    }

    fn in_area(&self, area: &Area) -> bool {
        &area.c1 <= self && self <= &area.c2
    }
}

struct Hailstone {
    start: Point,
    trajectory: Point,
}

impl FromStr for Hailstone {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
impl Hailstone {
    fn intersect(&self, other: &Hailstone) -> Option<Point> {
        let div = self
            .trajectory
            .checked_div(&other.trajectory)
            .expect("Should not be 0");
        if div.x == div.y {
            dbg!("parallel paths");
            return None;
        }
        let diff_start = other.start.clone() - self.start.clone();
        let trajectory_gain = self.trajectory.clone() - other.trajectory.clone();
        // x1 + xv1t1 = x2 + xv2t2
        // dx = - traj_1_x * t1 + traj_2_x * t2
        // t1 = t2 * traj_2_x / traj_1_x - dx / traj_1_x
        // dy = - traj_1_y * t1 + traj_2_y * t2
        // dy / traj_2_y + t1 * traj_1_y / traj_2_y * traj_2_x = t2
        // dy / traj_2_y + (t2 * traj_2_x / traj_1_x - dx / traj_1_x) * traj_1_y / traj_2_y * traj_2_x = t2
        // dy / traj_2_y +  - dx / traj_1_x * traj_1_y / traj_2_y * traj_2_x = t2 - t2 * traj_2_x / traj_1_x * traj_1_y / traj_2_y * traj_2_x
        // let t_ns = diff_start.checked_div(&trajectory_gain).exp();
        todo!()
    }
}

fn main() {
    let input = include_str!("../input_test.txt");
    let hailstones: Vec<Hailstone> = input
        .split_terminator('\n')
        .map(|s| Hailstone::from_str(s).unwrap())
        .collect();

    let area = Area {
        c1: Point { x: 7, y: 7 },
        c2: Point { x: 27, y: 27 },
    };
    let sum_in_area: u64 = (0..hailstones.len())
        .map(|h_i| {
            (0..hailstones.len())
                .filter(|h_j| h_i != *h_j)
                .map(|h_j| {
                    if let Some(p) = hailstones[h_i].intersect(&hailstones[h_j]) {
                        if p.in_area(&area) {
                            1
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        })
        .sum::<u64>();
    dbg!(sum_in_area);
}
