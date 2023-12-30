use core::str::FromStr;
use std::collections::{HashMap, HashSet};
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Direction {
    South,
    North,
    West,
    East,
}
impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
}
use Direction::*;

#[derive(Debug)]
struct HeatLossMap {
    height: i32,
    width: i32,
    grid: Vec<usize>,
}

impl FromStr for HeatLossMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.split_terminator('\n').next().unwrap().chars().count();
        let height = s.split_terminator('\n').count();
        dbg!(width);
        dbg!(height);
        let grid = s
            .chars()
            .filter(|&c| c != '\n')
            .map(|c| c as usize - '0' as usize)
            .collect();
        Ok(HeatLossMap {
            height: height as i32,
            width: width as i32,
            grid,
        })
    }
}
type Path = Vec<Direction>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConstrainedPath {
    constraint_direction: Option<Direction>,
    constraint_num: usize,
    heat_loss: usize,
    path: Path,
}
impl Ord for ConstrainedPath {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heat_loss.cmp(&other.heat_loss)
    }
}

impl PartialOrd for ConstrainedPath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.heat_loss.partial_cmp(&other.heat_loss)
    }
}

impl HeatLossMap {
    fn solve(&self) -> ConstrainedPath {
        let mut active_nodes = HashSet::new();
        active_nodes.insert(0);
        let mut hash_mins = HashMap::new();
        hash_mins.insert(
            0,
            vec![ConstrainedPath {
                constraint_direction: None,
                constraint_num: 0,
                heat_loss: 0,
                path: Vec::new(),
            }],
        );
        while !active_nodes.is_empty() {
            active_nodes = self.run_one_step(active_nodes, &mut hash_mins);
            // dbg!(&active_nodes);
            // dbg!(hash_mins.get(&1));
        }
        hash_mins
            .get(&((self.height * self.width - 1) as usize))
            .unwrap()
            .into_iter()
            .filter(|constr_path| constr_path.constraint_num >= 4)
            .min()
            .unwrap()
            .clone()
        // todo!()
    }

    fn run_one_step(
        &self,
        active_nodes: HashSet<usize>,
        hash_mins: &mut HashMap<usize, Vec<ConstrainedPath>>,
    ) -> HashSet<usize> {
        let mut new_active_nodes = HashSet::new();
        let all_directions = [North, South, East, West];
        active_nodes.into_iter().for_each(|index| {
            for constrained_path in hash_mins.get(&index).cloned().unwrap() {
                for dir in all_directions {
                    let same_direction =
                        if let Some(path_direction) = constrained_path.constraint_direction {
                            if dir == path_direction.opposite() {
                                continue;
                            }
                            if dir != path_direction && constrained_path.constraint_num < 4 {
                                continue;
                            }
                            let same_direction = dir == path_direction;
                            if same_direction && constrained_path.constraint_num == 10 {
                                continue;
                            }
                            same_direction
                        } else {
                            false
                        };
                    if let Some(new_index) = self.next_index(dir, index as i32).map(|i| i as usize)
                    {
                        let (constraint_direction, constraint_num) =
                            match (same_direction, constrained_path.constraint_num) {
                                (true, _) => (dir, constrained_path.constraint_num + 1),
                                (false, _) => (dir, 1),
                            };
                        let new_heat_loss = constrained_path.heat_loss + self.grid[new_index];
                        let mut new_path = constrained_path.path.clone();
                        new_path.push(dir);
                        let new_const_path = ConstrainedPath {
                            constraint_direction: Some(constraint_direction),
                            constraint_num,
                            heat_loss: new_heat_loss,
                            path: new_path,
                        };
                        if let Some(vec) = hash_mins.get_mut(&new_index) {
                            if let Some((i, similar_path)) =
                                vec.iter_mut().enumerate().find(|(_, const_path)| {
                                    const_path.constraint_num == constraint_num
                                        && const_path
                                            .constraint_direction
                                            .unwrap_or(constraint_direction.opposite())
                                            == constraint_direction
                                })
                            {
                                if new_heat_loss < similar_path.heat_loss {
                                    vec[i] = new_const_path.clone();
                                    new_active_nodes.insert(new_index);
                                } else {
                                    continue;
                                }
                            }
                        }
                        new_active_nodes.insert(new_index);
                        hash_mins
                            .entry(new_index)
                            .and_modify(|vec| vec.push(new_const_path.clone()))
                            .or_insert(vec![new_const_path]);
                    }
                }
            }
        });
        new_active_nodes
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
}

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../input_test.txt");
    // let input = include_str!("../input_test_2.txt");
    // let input = include_str!("../input_test_small.txt");
    let heat_loss_map = HeatLossMap::from_str(input).unwrap();
    let min_path = heat_loss_map.solve();
    dbg!(&min_path.path);
    // dbg!(&min_path.path.iter().filter(|d| d == &&East).count());
    // dbg!(&min_path.path.iter().filter(|d| d == &&South).count());
    dbg!(&min_path.heat_loss);
}
