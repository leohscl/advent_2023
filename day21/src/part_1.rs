use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    South,
    North,
    West,
    East,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Terrain {
    Rock,
    Plots,
}

use Direction::*;
use Terrain::*;

struct Garden {
    height: i32,
    width: i32,
    reachable: HashSet<i32>,
    grid: HashMap<i32, Terrain>,
}

impl FromStr for Garden {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.split_terminator('\n').next().unwrap().chars().count();
        let height = s.split_terminator('\n').count();
        let mut reachable = HashSet::new();
        let mut grid = HashMap::new();
        s.chars()
            .filter(|&c| c != '\n')
            .enumerate()
            .for_each(|(i, c)| {
                let terrain = match c {
                    '.' => Terrain::Plots,
                    '#' => Terrain::Rock,
                    'S' => {
                        reachable.insert(i as i32);
                        Terrain::Plots
                    }
                    _ => panic!("char does not translate to Terrain"),
                };
                grid.insert(i as i32, terrain);
            });
        Ok(Garden {
            height: height as i32,
            width: width as i32,
            reachable,
            grid,
        })
    }
}

impl Garden {
    fn next_index(&self, direction: Direction, previous_index: i32) -> Option<i32> {
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
        if self.grid[&new_index] == Rock {
            None
        } else {
            Some(new_index)
        }
    }
    fn step_once(&mut self) {
        let mut new_reachable = HashSet::new();
        let directions = [North, South, West, East];
        self.reachable.iter().for_each(|index_start| {
            directions.into_iter().for_each(|dir| {
                if let Some(i) = self.next_index(dir, *index_start) {
                    new_reachable.insert(i);
                }
            })
        });
        self.reachable = new_reachable;
    }

    fn count(&self) -> usize {
        self.reachable.iter().count()
    }
}

fn main() {
    // let input = include_str!("../input_test.txt");
    let input = include_str!("../input.txt");
    let mut garden = Garden::from_str(input).unwrap();
    for _ in 0..64 {
        garden.step_once();
    }
    dbg!(garden.count());
}
