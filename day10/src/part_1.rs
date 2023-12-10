use std::str::FromStr;

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

struct Grid {
    height: usize,
    width: usize,
    elements: Vec<Pipe>,
}

impl Grid {
    fn traverse_loop(&self) -> usize {
        let mut steps = 0;
        let start_index = self
            .elements
            .iter()
            .enumerate()
            .find_map(|(i, p)| if p == &Start { Some(i) } else { None })
            .unwrap();
        let mut current_index = start_index;
        let mut current_direction = N;
        let all_directions = [N, E, S, W];
        for potential_direction_start in all_directions {
            let potential_new_index = self.get_new_index(start_index, potential_direction_start);
            if self.elements[potential_new_index].has_cardinal(potential_direction_start.opposite())
            {
                current_direction = potential_direction_start;
                break;
            }
        }
        loop {
            dbg!(current_direction);
            steps += 1;
            current_index = self.get_new_index(current_index, current_direction);
            if current_index == start_index {
                break;
            }
            dbg!(self.elements[current_index]);
            current_direction = current_direction.opposite();
            current_direction = current_direction
                .other()
                .into_iter()
                .find(|&d| self.elements[current_index].has_cardinal(d))
                .unwrap();
        }
        steps
    }

    fn get_new_index(&self, old_index: usize, direction: Cardinal) -> usize {
        match direction {
            N => old_index - self.width,
            E => old_index + 1,
            S => old_index + self.width,
            W => old_index - 1,
        }
    }
}

fn main() {
    // let input = include_str!("../input_test_1.txt");
    // let input = include_str!("../input_test_2.txt");
    let input = include_str!("../input.txt");
    let height = input.split("\n").count();
    let width = input.split("\n").next().unwrap().chars().count();
    let elements = input
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| Pipe::from_str(&c.to_string()).unwrap())
        .collect();
    let grid = Grid {
        height,
        width,
        elements,
    };
    let steps = grid.traverse_loop();
    dbg!(steps / 2);
}
