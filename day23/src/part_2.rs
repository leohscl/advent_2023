#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    South,
    North,
    West,
    East,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Terrain {
    Path,
    Forest,
}

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use Direction::*;
use Terrain::*;

struct Garden {
    height: i32,
    width: i32,
    first_index: i32,
    last_index: i32,
    grid: HashMap<i32, Terrain>,
}

impl FromStr for Garden {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.split_terminator('\n').next().unwrap().chars().count();
        let height = s.split_terminator('\n').count();
        let mut grid = HashMap::new();
        s.chars()
            .filter(|&c| c != '\n')
            .enumerate()
            .for_each(|(i, c)| {
                let terrain = match c {
                    '.' => Terrain::Path,
                    '#' => Terrain::Forest,
                    '>' => Terrain::Path,
                    '<' => Terrain::Path,
                    '^' => Terrain::Path,
                    'v' => Terrain::Path,
                    _ => panic!("char does not translate to Terrain"),
                };
                grid.insert(i as i32, terrain);
            });
        let first_index = 1;
        let last_index = (height * width - 2) as i32;
        Ok(Garden {
            height: height as i32,
            width: width as i32,
            first_index,
            last_index,
            grid,
        })
    }
}

type Path = HashSet<i32>;

impl Garden {
    fn build_path(&self, current_index: i32, current_path: Path) -> Vec<Path> {
        if current_index == self.last_index {
            return vec![current_path];
        }
        let iter_directions = [North, West, South, East].into_iter();
        iter_directions
            .filter_map(|dir| self.next_index(dir, current_index, &current_path))
            .map(|index| {
                let mut new_path = current_path.clone();
                new_path.insert(index);
                self.build_path(index, new_path)
            })
            .flatten()
            .collect()
    }

    fn next_index(
        &self,
        direction: Direction,
        previous_index: i32,
        current_path: &Path,
    ) -> Option<i32> {
        let new_index = match direction {
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
        }?;
        if current_path.contains(&new_index) {
            None
        } else {
            match self.grid[&new_index] {
                Forest => None,
                _ => Some(new_index),
            }
        }
    }
}

fn main() {
    // let input = include_str!("../input_test_1.txt");
    // let input = include_str!("../input_test.txt");
    let input = include_str!("../input.txt");
    let garden = Garden::from_str(input).unwrap();
    let start_path = HashSet::new();
    // start_path.insert(garden.first_index);
    let paths = garden.build_path(garden.first_index, start_path);
    dbg!(paths.len());
    let max = paths
        .into_iter()
        .map(|path| path.iter().len())
        .max()
        .unwrap();
    dbg!(max);
}
