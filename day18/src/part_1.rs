use std::collections::HashSet;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Elements {
    Dug,
    Empty,
    Marked,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = match s {
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            "R" => Direction::Right,
            _ => panic!(),
        };
        Ok(dir)
    }
}

struct Grid {
    height: i32,
    width: i32,
    elements: Vec<Elements>,
}

fn get_index_iterator(
    dir: Direction,
    num: i32,
    width: i32,
    current_h: i32,
    current_w: i32,
) -> (i32, i32, Box<dyn Iterator<Item = i32>>) {
    let current_index = current_h * width + current_w;
    let mut new_h = current_h;
    let mut new_w = current_w;
    let new_iter: Box<dyn Iterator<Item = i32>> = match dir {
        Direction::Left => {
            new_w -= num;
            Box::new((current_index - num..=current_index).rev())
        }
        Direction::Right => {
            new_w += num;
            Box::new(current_index..=current_index + num)
        }
        Direction::Down => {
            new_h += num;
            Box::new((current_index..=current_index + num * width).step_by(width as usize))
        }
        Direction::Up => {
            new_h -= num;
            Box::new(
                (current_index - num * width..=current_index)
                    .rev()
                    .step_by(width as usize),
            )
        }
    };
    dbg!(current_h, current_w);
    dbg!(dir);
    (new_h, new_w, new_iter)
}

impl Grid {
    fn initialize_grid(input: &str) -> (Grid, i32, i32) {
        let line_iterator = input.split_terminator('\n').map(|line| parse_line(line));
        let coords_iter = line_iterator.scan((0, 0), |acc, (dir, num)| {
            match dir {
                Direction::Left => {
                    acc.1 -= num;
                }
                Direction::Right => {
                    acc.1 += num;
                }
                Direction::Down => {
                    acc.0 += num;
                }
                Direction::Up => {
                    acc.0 -= num;
                }
            }
            Some(acc.clone())
        });
        let min_w = coords_iter
            .clone()
            .min_by(|p1, p2| p1.1.cmp(&p2.1))
            .unwrap()
            .1;
        let max_w = coords_iter
            .clone()
            .max_by(|p1, p2| p1.1.cmp(&p2.1))
            .unwrap()
            .1;
        let min_h = coords_iter
            .clone()
            .min_by(|p0, p2| p0.0.cmp(&p2.0))
            .unwrap()
            .0;
        let max_h = coords_iter
            .clone()
            .max_by(|p0, p2| p0.0.cmp(&p2.0))
            .unwrap()
            .0;
        dbg!(max_h);
        dbg!(min_h);
        let height = max_h - min_h + 1;
        let width = max_w - min_w + 1;
        dbg!(height);
        dbg!(width);
        let elements = vec![Elements::Empty; (height * width) as usize];
        (
            Grid {
                height,
                width,
                elements,
            },
            -min_h,
            -min_w,
        )
    }
    fn print_dug(&self) {
        self.elements.chunks(self.width as usize).for_each(|c| {
            let s_chunk: String = c
                .iter()
                .map(|e| match e {
                    Elements::Dug => '#',
                    _ => '.',
                })
                .collect();
            println!("{s_chunk}");
        })
    }
    fn dig_with_plan(&mut self, plan: &str, height_start: i32, width_start: i32) {
        let line_iterator = plan.split_terminator('\n').map(|line| parse_line(line));
        let mut current_h = height_start;
        let mut current_w = width_start;
        line_iterator.for_each(|(dir, num)| {
            let (new_h, new_w, iter_index) =
                get_index_iterator(dir, num, self.width, current_h, current_w);
            iter_index.for_each(|index| {
                self.elements[index as usize] = Elements::Dug;
            });
            current_h = new_h;
            current_w = new_w;
        });
    }

    fn has_next(&self, index: i32, dir: Direction) -> Option<i32> {
        // dbg!(index, dir);
        match dir {
            Left => {
                if index % self.width != 0 {
                    Some(index - 1)
                } else {
                    None
                }
            }
            Right => {
                if index % self.width != self.width - 1 {
                    Some(index + 1)
                } else {
                    None
                }
            }
            Up => {
                if index / self.width != 0 {
                    Some(index - self.width)
                } else {
                    None
                }
            }
            Down => {
                if index / self.width != self.height - 1 {
                    Some(index + self.width)
                } else {
                    None
                }
            }
        }
    }
    fn carve_lake(&mut self) {
        while let Some(start_index) = self.elements.iter().enumerate().find_map(|(i, e)| {
            if e == &Elements::Empty {
                Some(i)
            } else {
                None
            }
        }) {
            let mut reachable = HashSet::new();
            let mut visited = HashSet::new();
            let mut current_element = Elements::Dug;
            reachable.insert(start_index);
            let all_directions = [Down, Up, Left, Right];
            while let Some(reach_index) = reachable.iter().cloned().next() {
                // dbg!(&reachable);
                visited.insert(reach_index);
                for dir in all_directions {
                    if let Some(index) = self.has_next(reach_index as i32, dir) {
                        if self.elements[index as usize] != Elements::Dug
                            && !visited.contains(&(index as usize))
                        {
                            reachable.insert(index as usize);
                        }
                    } else {
                        current_element = Elements::Marked;
                    }
                }
                reachable.remove(&reach_index);
                dbg!(visited.len());
            }
            visited
                .iter()
                .for_each(|i| self.elements[*i] = current_element);
        }
    }
    fn score(&self) -> usize {
        self.elements
            .iter()
            .filter(|&&e| e == Elements::Dug)
            .count()
    }
}

fn parse_line(line: &str) -> (Direction, i32) {
    let mut iter_line = line.split_whitespace();
    let (dir_str, num_str) = (iter_line.next().unwrap(), iter_line.next().unwrap());
    (
        Direction::from_str(dir_str).unwrap(),
        num_str.parse::<i32>().unwrap(),
    )
}

fn main() {
    // let input = include_str!("../input_test.txt");
    let input = include_str!("../input.txt");
    let (mut grid, height_start, width_start) = Grid::initialize_grid(input);
    grid.dig_with_plan(input, height_start, width_start);
    // grid.print_dug();
    grid.carve_lake();
    grid.print_dug();
    dbg!(grid.score());
}
