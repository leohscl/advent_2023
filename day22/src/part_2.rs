use derive_more::Add;
use derive_more::Mul;
use derive_more::Sub;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Sub, Mul, Add, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl FromStr for Point {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter_values = s.split(',').map(|s_i| s_i.parse::<i32>().unwrap());
        let (x, y, z) = (
            iter_values.next().unwrap(),
            iter_values.next().unwrap(),
            iter_values.next().unwrap(),
        );
        Ok(Point { x, y, z })
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
enum Dimension {
    X,
    Y,
    Z,
}

use Dimension::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Brick {
    start_point: Point,
    extention: Dimension,
    extention_num: i32,
}

impl FromStr for Brick {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter_point = s.split(['~']);
        let point_1 = Point::from_str(iter_point.next().unwrap()).unwrap();
        let point_2 = Point::from_str(iter_point.next().unwrap()).unwrap();
        let (extention, extention_num) = match point_2 - point_1.clone() {
            Point { x, y: 0, z: 0 } => (Dimension::X, x),
            Point { x: 0, y, z: 0 } => (Dimension::Y, y),
            Point { x: 0, y: 0, z } => (Dimension::Z, z),
            _ => panic!("Error in input, not single line"),
        };
        assert!(extention_num >= 0);
        Ok(Brick {
            start_point: point_1,
            extention,
            extention_num,
        })
    }
}

impl Brick {
    fn get_index_brick_support(&self, all_bricks: &VecDeque<Brick>) -> Option<Vec<usize>> {
        let point_z = Point { x: 0, y: 0, z: 1 };
        let point_y = Point { x: 0, y: 1, z: 0 };
        let point_x = Point { x: 1, y: 0, z: 0 };
        if self.start_point.z == 1 {
            return None;
        }
        let all_points_below = match self.extention {
            Dimension::Z => vec![self.start_point.clone() - point_z.clone()],
            Dimension::Y => (0..=self.extention_num)
                .map(|y_i_add| {
                    self.start_point.clone() + point_y.clone() * y_i_add - point_z.clone()
                })
                .collect(),
            Dimension::X => (0..=self.extention_num)
                .map(|x_i_add| {
                    self.start_point.clone() + point_x.clone() * x_i_add - point_z.clone()
                })
                .collect(),
        };
        Some(
            all_bricks
                .iter()
                .enumerate()
                .filter(|(_, b)| b.extention == Z || b.start_point.z == self.start_point.z - 1)
                .filter_map(|(i, brick)| {
                    if all_points_below.iter().any(|p| brick.occupies(p)) {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect(),
        )
    }

    fn can_fall_1_block(&self, all_bricks: &VecDeque<Brick>) -> bool {
        self.get_index_brick_support(all_bricks)
            .map(|v| v.is_empty())
            .unwrap_or(false)
    }

    fn max(&self, dimension: Dimension) -> i32 {
        let mut return_value = match dimension {
            X => self.start_point.x,
            Y => self.start_point.y,
            Z => self.start_point.z,
        };
        if self.extention == dimension {
            return_value += self.extention_num;
        }
        return_value
    }

    fn occupies(&self, p: &Point) -> bool {
        let point_diff = p.clone() - self.start_point.clone();
        // dbg!(&point_diff);
        let (extention, extention_num) = match point_diff {
            Point { x: 0, y: 0, z: 0 } => return true,
            Point { x, y: 0, z: 0 } => (Dimension::X, x),
            Point { x: 0, y, z: 0 } => (Dimension::Y, y),
            Point { x: 0, y: 0, z } => (Dimension::Z, z),
            _ => return false,
        };
        extention == self.extention && extention_num <= self.extention_num && extention_num >= 0
    }
}
fn print_y_z(bricks: &VecDeque<Brick>) {
    let max_x = bricks.iter().map(|b| b.max(Dimension::X)).max().unwrap();
    let max_y = bricks.iter().map(|b| b.max(Dimension::Y)).max().unwrap();
    let max_z = bricks.iter().map(|b| b.max(Dimension::Z)).max().unwrap();
    // dbg!(max_x, max_y, max_z);
    for z_i in (0..=max_z).rev() {
        let s: String = (0..=max_y)
            .map(|y_i| {
                (0..=max_x).find_map(|x_i| {
                    let point_test = Point {
                        x: x_i,
                        y: y_i,
                        z: z_i,
                    };
                    bricks.iter().enumerate().find_map(|(i, brick)| {
                        if brick.occupies(&point_test) {
                            // dbg!(&brick);
                            // dbg!(&point_test);
                            Some(i)
                        } else {
                            None
                        }
                    })
                })
            })
            .map(|opt_i| {
                if let Some(i) = opt_i {
                    char::from_digit(i as u32, 10).unwrap()
                } else {
                    '.'
                }
            })
            .collect();
        println!("{s}");
    }
}

fn print_x_z(bricks: &VecDeque<Brick>) {
    let max_x = bricks.iter().map(|b| b.max(Dimension::X)).max().unwrap();
    let max_y = bricks.iter().map(|b| b.max(Dimension::Y)).max().unwrap();
    let max_z = bricks.iter().map(|b| b.max(Dimension::Z)).max().unwrap();
    // dbg!(max_x, max_y, max_z);
    for z_i in (0..=max_z).rev() {
        let s: String = (0..=max_x)
            .map(|x_i| {
                (0..=max_y).find_map(|y_i| {
                    let point_test = Point {
                        x: x_i,
                        y: y_i,
                        z: z_i,
                    };
                    bricks.iter().enumerate().find_map(|(i, brick)| {
                        if brick.occupies(&point_test) {
                            // dbg!(&brick);
                            // dbg!(&point_test);
                            Some(i)
                        } else {
                            None
                        }
                    })
                })
            })
            .map(|opt_i| {
                if let Some(i) = opt_i {
                    char::from_digit(i as u32, 10).unwrap()
                } else {
                    '.'
                }
            })
            .collect();
        println!("{s}");
    }
}

fn main() {
    // let input = include_str!("../input_test.txt");
    let input = include_str!("../input.txt");
    let mut all_bricks: VecDeque<_> = input
        .split_terminator('\n')
        .map(|line| Brick::from_str(line).unwrap())
        .collect();
    // print_x_z(&all_bricks);
    // print_y_z(&all_bricks);
    dbg!("collected");

    all_bricks
        .make_contiguous()
        .sort_by(|b1, b2| b1.max(Dimension::Z).cmp(&b2.max(Dimension::Z)));
    dbg!("sorted");
    let mut fully_fallen_num = 0;
    while let Some(i) = all_bricks
        .iter()
        .enumerate()
        .skip(fully_fallen_num)
        .find_map(|(i, brick)| {
            if brick.can_fall_1_block(&all_bricks) {
                Some(i)
            } else {
                None
            }
        })
    {
        fully_fallen_num = i;
        let point_z = Point { x: 0, y: 0, z: 1 };
        all_bricks[i].start_point = all_bricks[i].start_point.clone() - point_z;
    }
    dbg!("fallen");
    let hash_depend: HashMap<usize, Option<Vec<usize>>> = (0..all_bricks.len())
        .map(|i| (i, all_bricks[i].get_index_brick_support(&all_bricks)))
        .collect();
    let max_z = all_bricks
        .iter()
        .map(|b| b.max(Dimension::Z))
        .max()
        .unwrap();
    let sum_dis = (0..all_bricks.len())
        .map(|i| {
            let mut hash_will_fall = HashSet::new();
            hash_will_fall.insert(i);
            let start_z = all_bricks[i].start_point.z;
            (start_z + 1..=max_z).for_each(|z| {
                all_bricks
                    .iter()
                    .enumerate()
                    .filter(|(_, b)| b.start_point.z == z)
                    .for_each(|(i, _)| {
                        if hash_depend[&i]
                            .as_ref()
                            .map(|v| {
                                v.iter()
                                    .filter(|i| !hash_will_fall.contains(i))
                                    .next()
                                    .is_none()
                            })
                            .expect("there cannot be no vector of support here")
                        {
                            hash_will_fall.insert(i);
                        }
                    })
            });
            hash_will_fall.iter().count() - 1
        })
        .sum::<usize>();
    // check for disinteration
    dbg!(sum_dis);
}
