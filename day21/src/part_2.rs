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
    type_height: i64,
    type_width: i64,
    reachable: HashSet<(i64, i64)>,
    grid: HashMap<(i64, i64), Terrain>,
    even_reached: HashSet<(i64, i64)>,
    odd_reached: HashSet<(i64, i64)>,
}

impl FromStr for Garden {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let type_width = s.split_terminator('\n').next().unwrap().chars().count() as i64;
        let type_height = s.split_terminator('\n').count() as i64;
        let mut reachable = HashSet::new();
        let odd_reached = HashSet::new();
        let mut even_reached = HashSet::new();
        let mut grid = HashMap::new();
        s.chars()
            .filter(|&c| c != '\n')
            .enumerate()
            .for_each(|(i, c)| {
                let index = i as i64;
                let h = index / type_width;
                let w = index % type_width;
                let terrain = match c {
                    '.' => Terrain::Plots,
                    '#' => Terrain::Rock,
                    'S' => {
                        reachable.insert((h, w));
                        even_reached.insert((h, w));
                        Terrain::Plots
                    }
                    _ => panic!("char does not translate to Terrain"),
                };
                grid.insert((h, w), terrain);
            });
        Ok(Garden {
            type_height,
            type_width,
            reachable,
            grid,
            even_reached,
            odd_reached,
        })
    }
}

impl Garden {
    fn is_rock(&self, index: (i64, i64)) -> bool {
        let (h, w) = index;
        let res = self.grid[&(
            h.rem_euclid(self.type_height),
            w.rem_euclid(self.type_width),
        )] == Rock;
        res
    }

    fn next_index(&self, direction: Direction, previous_index: (i64, i64)) -> Option<(i64, i64)> {
        let (h, w) = previous_index;
        let new_index = match direction {
            Direction::West => (h, w + 1),
            Direction::East => (h, w - 1),
            Direction::North => (h - 1, w),
            Direction::South => (h + 1, w),
        };
        if self.is_rock(new_index) {
            None
        } else {
            Some(new_index)
        }
    }
    fn step_once(&mut self, even: bool) {
        let directions = [North, South, West, East];

        let to_insert = match even {
            true => &self.odd_reached,
            false => &self.even_reached,
        };

        let mut to_add = HashSet::new();

        self.reachable.iter().for_each(|index_start| {
            directions.into_iter().for_each(|dir| {
                if let Some(i) = self.next_index(dir, *index_start) {
                    if !to_insert.contains(&i) {
                        to_add.insert(i);
                    }
                }
            })
        });
        let to_insert = match even {
            true => &mut self.odd_reached,
            false => &mut self.even_reached,
        };
        to_insert.extend(to_add.iter());
        self.reachable = to_add;
    }

    fn count(&self) -> (usize, usize) {
        (
            self.even_reached.iter().count(),
            self.odd_reached.iter().count(),
        )
    }
}

fn main() {
    let input = include_str!("../input_test.txt");
    // let input = include_str!("../input.txt");
    let mut garden = Garden::from_str(input).unwrap();
    for step_i in 0..10000 {
        garden.step_once(step_i % 2 == 0);
    }
    dbg!(garden.count());
}
