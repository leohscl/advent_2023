use core::str::FromStr;
use std::collections::HashMap;
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
enum Terrain {
    RoundRock,
    CubeRock,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Reflector {
    height: usize,
    width: usize,
    grid: Vec<Terrain>,
}

impl FromStr for Reflector {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.split_terminator('\n').next().unwrap().chars().count();
        let height = s.split_terminator('\n').count();
        let grid = s
            .chars()
            .filter(|&c| c != '\n')
            .map(|c| match c {
                'O' => Terrain::RoundRock,
                '#' => Terrain::CubeRock,
                '.' => Terrain::Empty,
                _ => panic!("char does not translate to Terrain"),
            })
            .collect();
        Ok(Reflector {
            height,
            width,
            grid,
        })
    }
}

impl Reflector {
    fn tilt_up(&mut self) {
        let index_starts = 0..self.width;
        let increment = self.width as i64;
        let num = self.height;
        self.tilt(index_starts, increment, num);
    }
    fn tilt_down(&mut self) {
        let start = (self.height - 1) * self.width;
        let end = self.height * self.width;
        let index_starts = start..end;
        let increment = -(self.width as i64);
        let num = self.height;
        self.tilt(index_starts, increment, num);
    }
    fn tilt_left(&mut self) {
        let index_starts = (0..self.grid.len()).step_by(self.width);
        let increment = 1;
        let num = self.width;
        self.tilt(index_starts, increment, num);
    }
    fn tilt_right(&mut self) {
        let index_starts = (self.width - 1..self.grid.len()).step_by(self.width);
        let increment = -1;
        let num = self.width;
        self.tilt(index_starts, increment, num);
    }
    fn tilt<I>(&mut self, index_starts: I, increment: i64, num: usize)
    where
        I: Iterator<Item = usize>,
    {
        index_starts.for_each(|index_start| {
            let mut previous_index_solid = index_start as i64;
            let iterate_value =
                std::iter::successors(Some(index_start as i64), |n| Some(n + increment));
            iterate_value
                .take(num)
                .for_each(|index_elt| match self.grid[index_elt as usize] {
                    Terrain::CubeRock => previous_index_solid = index_elt + increment,
                    Terrain::Empty => (),
                    Terrain::RoundRock => {
                        self.grid[index_elt as usize] = Terrain::Empty;
                        self.grid[previous_index_solid as usize] = Terrain::RoundRock;
                        previous_index_solid += increment;
                    }
                })
        })
    }
    fn score(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .map(|(i, terrain)| {
                if terrain == &Terrain::RoundRock {
                    let line = i / self.width;
                    let score = self.height - line;
                    score
                } else {
                    0
                }
            })
            .sum()
    }
    fn print_grid_elements(&self) {
        self.grid.chunks(self.width).for_each(|line_elts| {
            let res: String = line_elts
                .into_iter()
                .map(|e| match e {
                    Terrain::RoundRock => 'O',
                    Terrain::CubeRock => '#',
                    Terrain::Empty => '.',
                })
                .collect();
            println!("{res}");
        })
    }
}

fn main() {
    // let input = include_str!("../input_test.txt");
    let input = include_str!("../input.txt");
    let mut reflector = Reflector::from_str(input).unwrap();
    let cycles = 1000000000;
    let mut hash_reflector = HashMap::new();
    for cycle_i in 1..cycles {
        reflector.tilt_up();
        reflector.tilt_left();
        reflector.tilt_down();
        reflector.tilt_right();
        // reflector.print_grid_elements();
        if let Some(cycle_previous_i) = hash_reflector.get(&reflector) {
            if (cycles - cycle_i) % (cycle_previous_i - cycle_i) == 0 {
                break;
            }
            println!("{cycle_previous_i}, {cycle_i}");
        } else {
            hash_reflector.insert(reflector.clone(), cycle_i);
        }
    }
    dbg!(reflector.score());
}
