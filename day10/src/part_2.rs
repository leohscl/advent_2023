use std::{collections::{VecDeque, HashSet}, str::FromStr};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Pipe {
    NS,
    WE,
    NE,
    SE,
    SW,
    NW,
    Ground,
    Start,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Cardinal {
    N,
    E,
    S,
    W,
}

impl Cardinal {
    fn opposite(&self) -> Cardinal {
        match self {
            N => S,
            S => N,
            E => W,
            W => E,
        }
    }
    fn other(&self) -> [Cardinal; 3] {
        match self {
            N => [W, S, E],
            S => [N, E, W],
            E => [W, N, S],
            W => [E, N, S],
        }
    }
}

use Pipe::*;

impl FromStr for Pipe {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(NS),
            "-" => Ok(WE),
            "L" => Ok(NE),
            "F" => Ok(SE),
            "7" => Ok(SW),
            "J" => Ok(NW),
            "." => Ok(Ground),
            "S" => Ok(Start),
            _ => panic!("Unrecognised string"),
        }
    }
}

use Cardinal::*;

impl Pipe {
    fn has_cardinal(&self, cardinal: Cardinal) -> bool {
        match (self, cardinal) {
            (&p, N) if p == NS || p == NE || p == NW => true,
            (&p, S) if p == NS || p == SE || p == SW => true,
            (&p, E) if p == WE || p == NE || p == SE => true,
            (&p, W) if p == WE || p == SW || p == NW => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Mark {
    Blank,
    Inside,
    Outside,
    Loop,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct MarkedPipe {
    pipe: Pipe,
    mark: Mark,
}

struct Grid {
    height: usize,
    width: usize,
    elements: Vec<MarkedPipe>,
}

impl Grid {
    fn print_grid(&self) {
        self.elements.chunks(self.width).for_each(|line_elts| {
            let res: String = line_elts
                .into_iter()
                .map(|e| if e.mark == Mark::Loop { 'L' } else { 'O' })
                .collect();
            println!("{res}");
        })
    }
}

impl Grid {
    fn mark_loop(&mut self) -> usize {
        let mut steps = 0;
        let start_index = self
            .elements
            .iter()
            .enumerate()
            .find_map(|(i, p)| if p.pipe == Start { Some(i) } else { None })
            .unwrap();
        let mut current_index = start_index;
        let mut current_direction = N;
        let all_directions = [N, E, S, W];
        for potential_direction_start in all_directions {
            let potential_new_index = self
                .get_new_index(start_index, potential_direction_start)
                .unwrap();
            if self.elements[potential_new_index]
                .pipe
                .has_cardinal(potential_direction_start.opposite())
            {
                current_direction = potential_direction_start;
                break;
            }
        }
        let start_direction = current_direction;
        loop {
            steps += 1;
            current_index = self
                .get_new_index(current_index, current_direction)
                .unwrap();
            if current_index == start_index {
                break;
            }
            let marked_pipe = &mut self.elements[current_index];
            marked_pipe.mark = Mark::Loop;
            current_direction = current_direction.opposite();
            current_direction = current_direction
                .other()
                .into_iter()
                .find(|&d| marked_pipe.pipe.has_cardinal(d))
                .unwrap();
        }
        let pipe_start = match (current_direction, start_direction) {
            (N, S) | (S, N) => NS,
            (E, S) | (S, E) => SE,
            (W, S) | (S, W) => SW,
            (N, E) | (E, N) => NE,
            (W, N) | (N, W) => NW,
            (W, E) | (E, W) => WE,
            _ => panic!(),
        };
        self.elements[start_index] = MarkedPipe {
            pipe: pipe_start,
            mark: Mark::Loop,
        };
        steps
    }

    fn get_new_index(&self, old_index: usize, direction: Cardinal) -> Option<usize> {
        match direction {
            N => old_index.checked_sub(self.width),
            E => old_index.checked_add(1),
            S => old_index.checked_add(self.width),
            W => old_index.checked_sub(1),
        }
    }

    fn try_mark(&mut self) -> bool {
        let mut visited_index: HashSet<usize> = HashSet::new();
        let mut to_check_index: VecDeque<usize> = VecDeque::new();
        let mut mark_variant = Mark::Inside;
        let all_directions = [N, E, S, W];
        if let Some(start) = self
            .elements
            .iter()
            .enumerate()
            .find_map(|(i, marked_pipe)| {
                if marked_pipe.mark == Mark::Blank {
                    Some(i)
                } else {
                    None
                }
            })
        {
            to_check_index.push_back(start);
            loop {
                if to_check_index.next
            }
            visited_index.push_back(start);
            for elt_index in visited_index {
                if self.elements[elt_index].mark == Mark::Blank {
                    self.elements[elt_index].mark = mark_variant;
                }
            }
            true
        } else {
            false
        }
    }
}

fn main() {
    let input = include_str!("../input_test_1.txt");
    // let input = include_str!("../input_test_2.txt");
    // let input = include_str!("../input.txt");
    let height = input.split('\n').count();
    let width = input.split('\n').next().unwrap().chars().count();
    let elements = input
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| {
            let pipe = Pipe::from_str(&c.to_string()).unwrap();
            MarkedPipe {
                pipe,
                mark: Mark::Blank,
            }
        })
        .collect();
    let mut grid = Grid {
        height,
        width,
        elements,
    };
    let steps = grid.mark_loop();
    while grid.try_mark() {
        grid.print_grid();
    }
    dbg!(steps / 2);
}
