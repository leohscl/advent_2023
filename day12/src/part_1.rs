use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum Conditions {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Solution {
    data: Vec<Conditions>,
}

use Conditions::*;

impl FromStr for Solution {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .chars()
            .map(|c| match c {
                '.' => Operational,
                '#' => Damaged,
                '?' => Unknown,
                _ => panic!("unknown char"),
            })
            .collect();
        Ok(Solution { data })
    }
}

impl Solution {
    fn from_blocks(blocks: &Vec<usize>, combination: &VecDeque<usize>) -> Solution {
        let mut data = Vec::new();
        for i in 0..combination.len() {
            data.extend(std::iter::repeat(Operational).take(combination[i]));
            if i != combination.len() - 1 {
                data.extend(std::iter::repeat(Damaged).take(blocks[i]));
            }
        }
        Solution { data }
    }
    fn matches(&self, data_solution: &Solution) -> bool {
        self.data
            .iter()
            .zip(data_solution.data.iter())
            .all(|(test, validation)| match (test, validation) {
                (Operational, Damaged) => false,
                (Damaged, Operational) => false,
                _ => true,
            })
    }
}

fn create_combination_with_opt(
    num_choices: usize,
    freedom_range: usize,
) -> VecDeque<VecDeque<usize>> {
    let mut vec_mother = VecDeque::new();
    for length_opt in 0..=(freedom_range - num_choices) {
        for length_start in 0..=length_opt {
            let length_end = length_opt - length_start;
            // dbg!(length_start, length_end);
            let new_vec_mother =
                create_combination(num_choices, freedom_range - length_start - length_end)
                    .into_iter();
            vec_mother.extend(new_vec_mother.into_iter().map(|mut v| {
                v.push_front(length_start);
                v.push_back(length_end);
                v
            }));
        }
    }
    vec_mother
}

fn create_combination(num_choices: usize, freedom_range: usize) -> VecDeque<VecDeque<usize>> {
    assert!(freedom_range >= num_choices);
    let mut vec_mother = VecDeque::new();
    if num_choices == 1 {
        let mut new_vec = VecDeque::new();
        new_vec.push_front(freedom_range);
        vec_mother.push_front(new_vec);
    } else {
        for length in 1..=(freedom_range - num_choices + 1) {
            let new_vec_mother = create_combination(num_choices - 1, freedom_range - length);
            vec_mother.extend(new_vec_mother.into_iter().map(|mut v| {
                v.push_front(length);
                v
            }));
        }
    }
    return vec_mother;
}

#[cfg(test)]
mod tests {
    use crate::{create_combination, create_combination_with_opt};

    #[test]
    fn test_basic_combination() {
        let combination_1 = create_combination(2, 3);
        assert_eq!(combination_1.len(), 2);
        let combination_2 = create_combination(2, 4);
        assert_eq!(combination_2.len(), 3);
        let combination_3 = create_combination(3, 5);
        dbg!(&combination_3);
        assert_eq!(combination_3.len(), 6);
    }
    #[test]
    fn test_opt_combination() {
        let combination_1 = create_combination_with_opt(2, 3);
        assert_eq!(combination_1.len(), 4);
        let combination_2 = create_combination_with_opt(2, 4);
        dbg!(&combination_2);
        assert_eq!(combination_2.len(), 3 + 2 + 2 + 3);
    }
}

fn main() {
    // let input = include_str!("../input_test_0.txt");
    // let input = include_str!("../input_test_1.txt");
    let input = include_str!("../input.txt");
    let sum_combinations: usize = input
        .split_terminator("\n")
        .map(|line| {
            let mut iter_whitespace = line.split_whitespace();
            let (data, damaged_blocks) = (
                iter_whitespace.next().unwrap(),
                iter_whitespace.next().unwrap(),
            );
            let data_solution = Solution::from_str(data).unwrap();
            let vec_blocks: Vec<_> = damaged_blocks
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let num_choices = vec_blocks.len() - 1;
            let freedom_range = data_solution.data.len() - vec_blocks.iter().sum::<usize>();
            let combinations =
                create_combination_with_opt(num_choices as usize, freedom_range as usize);
            combinations
                .into_iter()
                .filter(|combination| {
                    let potential_solution = Solution::from_blocks(&vec_blocks, combination);
                    // dbg!(&potential_solution);
                    potential_solution.matches(&data_solution)
                })
                .count()
        })
        .sum();
    dbg!(sum_combinations);
}
