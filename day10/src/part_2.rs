use std::{collections::HashSet, str::FromStr};

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

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
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
    fn is_corner(&self) -> bool {
        match self {
            NS | WE => false,
            _ => true,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Mark {
    Blank,
    Inside,
    Outside,
    Loop,
    Duplicated,
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
    fn print_grid_marks(&self) {
        self.elements.chunks(self.width).for_each(|line_elts| {
            let res: String = line_elts
                .into_iter()
                .map(|e| match e.mark {
                    Mark::Loop => 'L',
                    Mark::Outside => 'O',
                    Mark::Blank => '.',
                    Mark::Inside => 'I',
                    Mark::Duplicated => 'D',
                })
                .collect();
            println!("{res}");
        })
    }
    fn print_grid_elements(&self) {
        self.elements.chunks(self.width).for_each(|line_elts| {
            let res: String = line_elts
                .into_iter()
                .map(|e| match e.pipe {
                    NS => "|",
                    WE => "-",
                    NE => "L",
                    SE => "F",
                    SW => "7",
                    NW => "J",
                    Ground => ".",
                    Start => "S",
                })
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
        let pipe_start = match (current_direction.opposite(), start_direction) {
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
        let new_index = match direction {
            N => old_index.checked_sub(self.width),
            E => old_index.checked_add(1),
            S => old_index.checked_add(self.width),
            W => old_index.checked_sub(1),
        };
        if let Some(i) = new_index {
            if i >= self.width * self.height {
                return None;
            }
        }
        new_index
    }

    fn extend_grid(&self) -> Grid {
        let width = self.width * 2;
        let height = self.height * 2;
        let duplicated_rows: Vec<_> = self
            .elements
            .chunks(self.width)
            .flat_map(|chunk| {
                let iter_new_chunk = chunk.into_iter().map(|marked_pipe| {
                    let new_pipe = match marked_pipe.pipe {
                        WE => Ground,
                        SW => NS,
                        SE => NS,
                        NE => Ground,
                        NW => Ground,
                        _ => marked_pipe.pipe,
                    };
                    MarkedPipe {
                        mark: Mark::Duplicated,
                        pipe: new_pipe,
                    }
                });
                chunk.into_iter().cloned().chain(iter_new_chunk)
            })
            .collect();
        let duplicated_cols = duplicated_rows
            .into_iter()
            .flat_map(|marked_pipe| {
                let new_pipe = match marked_pipe.pipe {
                    NS => Ground,
                    SE => WE,
                    NE => WE,
                    NW => Ground,
                    SW => Ground,
                    _ => marked_pipe.pipe,
                };
                let new_marked_pipe = MarkedPipe {
                    mark: Mark::Duplicated,
                    pipe: new_pipe,
                };
                [marked_pipe, new_marked_pipe].into_iter()
            })
            .collect();
        Grid {
            height,
            width,
            elements: duplicated_cols,
        }
    }

    fn try_mark(&mut self) -> bool {
        let mut visited_index: HashSet<usize> = HashSet::new();
        let mut to_check_indexes: HashSet<usize> = HashSet::new();
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
            to_check_indexes.insert(start);
            loop {
                if let Some(&to_check_index) = to_check_indexes.iter().next() {
                    visited_index.insert(to_check_index);
                    all_directions.iter().for_each(|&d| {
                        // check if outside grid
                        if let Some(index) = self.get_new_index(to_check_index, d) {
                            let marked_pipe = self.elements[index];
                            // check if we can thread on the next index
                            if marked_pipe.mark != Mark::Loop {
                                if !visited_index.contains(&index) {
                                    to_check_indexes.insert(index);
                                }
                            }
                        } else {
                            mark_variant = Mark::Outside;
                        }
                    });
                    to_check_indexes.remove(&to_check_index);
                } else {
                    break;
                }
            }
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
    // let input = include_str!("../input_test_1.txt");
    // let input = include_str!("../input_test_2.txt");
    // let input = include_str!("../input_test_3.txt");
    let input = include_str!("../input_test_4.txt");
    // let input = include_str!("../input.txt");
    let height = input.split_terminator('\n').count();
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
    let grid = Grid {
        height,
        width,
        elements,
    };
    let mut extended_grid = grid.extend_grid();
    extended_grid.mark_loop();
    extended_grid.print_grid_elements();
    extended_grid.print_grid_marks();
    while extended_grid.try_mark() {}
    extended_grid.print_grid_marks();
    // let count_i = grid
    //     .elements
    //     .into_iter()
    //     .filter(|m_p| m_p.mark == Mark::Inside)
    //     .count();
    // dbg!(count_i);
}
