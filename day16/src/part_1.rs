use core::str::FromStr;
use itertools::Itertools;
use std::collections::HashSet;
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Terrain {
    BackMirror,
    ForwardMirror,
    Empty,
    VerticalSplitter,
    HorizontalSplitter,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Direction {
    South,
    North,
    West,
    East,
}

#[derive(Debug)]
struct Contraption {
    height: i32,
    width: i32,
    grid: Vec<Terrain>,
    energized: HashSet<i32>,
}

use Direction::*;
use Terrain::*;
impl FromStr for Contraption {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.split_terminator('\n').next().unwrap().chars().count();
        let height = s.split_terminator('\n').count();
        dbg!(width);
        dbg!(height);
        let grid = s
            .chars()
            .filter(|&c| c != '\n')
            .map(|c| match c {
                '|' => Terrain::VerticalSplitter,
                '-' => Terrain::HorizontalSplitter,
                '/' => Terrain::ForwardMirror,
                '\\' => Terrain::BackMirror,
                '.' => Terrain::Empty,
                _ => panic!("char does not translate to Terrain"),
            })
            .collect();
        let energized = HashSet::new();
        Ok(Contraption {
            height: height as i32,
            width: width as i32,
            grid,
            energized,
        })
    }
}

impl Contraption {
    fn print_light(&self) {
        let string: Vec<char> = (0..self.height)
            .cartesian_product(0..self.width)
            .map(|(i, j)| {
                if self.energized.contains(&(i * self.width + j)) {
                    '#'
                } else {
                    '.'
                }
            })
            .collect();
        string.chunks(self.width as usize).for_each(|chunk| {
            let line = chunk.iter().collect::<String>();
            println!("{line}");
        })
    }
    fn score(&self) -> u64 {
        (0..self.height)
            .cartesian_product(0..self.width)
            .map(|(i, j)| {
                if self.energized.contains(&(i * self.width + j)) {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
    fn print_grid_elements(&self) {
        self.grid.chunks(self.width as usize).for_each(|line_elts| {
            let res: String = line_elts
                .into_iter()
                .map(|e| match e {
                    Terrain::VerticalSplitter => '|',
                    Terrain::HorizontalSplitter => '-',
                    Terrain::ForwardMirror => '/',
                    Terrain::BackMirror => '\\',
                    Terrain::Empty => '.',
                })
                .collect();
            println!("{res}");
        })
    }

    fn next_index(&self, direction: Direction, previous_index: i32) -> Option<i32> {
        match direction {
            Direction::West => {
                if (previous_index % self.width) != 0 {
                    Some(previous_index - 1)
                } else {
                    None
                }
            }
            Direction::East => {
                if (previous_index % self.width) != self.width - 1 {
                    Some(previous_index + 1)
                } else {
                    None
                }
            }
            Direction::North => {
                let new_index = previous_index - self.width;
                if new_index < 0 {
                    None
                } else {
                    Some(new_index)
                }
            }
            Direction::South => {
                let new_index = previous_index + self.width;
                if new_index >= self.width * self.height {
                    None
                } else {
                    Some(new_index)
                }
            }
        }
    }

    fn get_new_directions(&self, direction: Direction, index: i32) -> Vec<Direction> {
        match (direction, self.grid[index as usize]) {
            (_, Empty) => vec![direction],
            (West, BackMirror) => vec![North],
            (North, BackMirror) => vec![West],
            (South, BackMirror) => vec![East],
            (East, BackMirror) => vec![South],
            (West, ForwardMirror) => vec![South],
            (South, ForwardMirror) => vec![West],
            (North, ForwardMirror) => vec![East],
            (East, ForwardMirror) => vec![North],
            (East | West, HorizontalSplitter) => vec![direction],
            (North | South, VerticalSplitter) => vec![direction],
            (North | South, HorizontalSplitter) => vec![East, West],
            (East | West, VerticalSplitter) => vec![North, South],
        }
    }

    fn emulate_light(
        &mut self,
        index: i32,
        direction: Direction,
        hash_checked: &mut HashSet<(i32, Direction)>,
    ) {
        if !hash_checked.contains(&(index, direction)) {
            hash_checked.insert((index, direction));
            self.energized.insert(index);
            let new_directions = self.get_new_directions(direction, index);
            for dir in new_directions {
                if let Some(new_index) = self.next_index(dir, index) {
                    self.emulate_light(new_index, dir, hash_checked)
                }
            }
        }
    }
}

fn main() {
    // let input = include_str!("../input_test.txt");
    let input = include_str!("../input.txt");
    let mut contraption = Contraption::from_str(input).unwrap();
    contraption.print_grid_elements();
    contraption.emulate_light(0, Direction::East, &mut HashSet::new());
    println!("");
    contraption.print_light();
    let score = contraption.score();
    dbg!(score);
}
