use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Hash)]
struct Solution {
    data: Vec<Condition>,
}

use Condition::*;

impl FromStr for Solution {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut unfolded_data = Vec::new();
        let data_iter = s.chars().map(|c| match c {
            '.' => Operational,
            '#' => Damaged,
            '?' => Unknown,
            _ => panic!("unknown char"),
        });
        // unfolding
        for _ in 0..4 {
            unfolded_data.extend(data_iter.clone());
            unfolded_data.push(Unknown)
        }
        unfolded_data.extend(data_iter);
        Ok(Solution {
            data: unfolded_data,
        })
    }
}

fn not_operational(condition: &Condition) -> bool {
    condition != &Operational
}
fn not_damaged(condition: &Condition) -> bool {
    condition != &Damaged
}

fn create_combination_with_opt(
    num_choices: usize,
    freedom_range: usize,
    data_solution: &Vec<Condition>,
    blocks: &Vec<usize>,
) -> usize {
    let mut hash_done = HashMap::new();
    let mut count = 0;
    for length_opt in 0..=(freedom_range - num_choices) {
        for length_start in 0..=length_opt {
            let length_end = length_opt - length_start;
            let iter_forward = data_solution.iter();
            let iter_backward = data_solution.iter().rev();
            let block_start = blocks[0];
            let block_end = blocks[blocks.len() - 1];
            if iter_forward.clone().take(length_start).all(not_damaged)
                && iter_forward
                    .clone()
                    .skip(length_start)
                    .take(block_start)
                    .all(not_operational)
                && iter_backward.clone().take(length_end).all(not_damaged)
                && iter_backward
                    .clone()
                    .skip(length_end)
                    .take(block_end)
                    .all(not_operational)
            {
                let to_take = freedom_range - length_start - length_end - block_start - block_end
                    + blocks.iter().sum::<usize>();
                let data_solution_trunc = iter_forward
                    .skip(length_start + block_start)
                    .take(to_take)
                    .cloned()
                    .collect();
                let new_blocks = blocks
                    .iter()
                    .cloned()
                    .skip(1)
                    .take(blocks.len() - 2)
                    .collect();
                count += create_combination(
                    num_choices,
                    freedom_range - length_start - length_end,
                    &data_solution_trunc,
                    &new_blocks,
                    &mut hash_done,
                );
            }
        }
    }
    count
}

fn create_combination(
    num_choices: usize,
    freedom_range: usize,
    data_solution: &Vec<Condition>,
    blocks: &Vec<usize>,
    hash_done: &mut HashMap<(usize, usize, Vec<Condition>, Vec<usize>), usize>,
) -> usize {
    assert!(freedom_range >= num_choices);
    let key = (
        num_choices,
        freedom_range,
        data_solution.clone(),
        blocks.clone(),
    );
    if let Some(value) = hash_done.get(&key) {
        return *value;
    }
    let mut count = 0;
    if num_choices == 1 {
        if data_solution.iter().all(not_damaged) {
            count += 1;
        }
    } else {
        for length in 1..=(freedom_range - num_choices + 1) {
            let iter_forward = data_solution.iter();
            if iter_forward.clone().take(length).all(not_damaged)
                && iter_forward
                    .clone()
                    .skip(length)
                    .take(blocks[0])
                    .all(not_operational)
            {
                let new_blocks = blocks.iter().skip(1).cloned().collect();
                let new_data_solution = iter_forward.skip(length + blocks[0]).cloned().collect();
                let add = create_combination(
                    num_choices - 1,
                    freedom_range - length,
                    &new_data_solution,
                    &new_blocks,
                    hash_done,
                );
                count += add
            }
        }
    }
    hash_done.insert(key, count);
    count
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::create_combination_with_opt;

    use crate::Solution;
    #[test]
    fn test_opt_combination() {
        // ??????, 1,2
        let solution_1 = Solution::from_str("??????").unwrap();
        let blocks: Vec<_> = "1,1,1"
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        dbg!(&blocks);
        let combination_1 = create_combination_with_opt(2, 3, &solution_1.data, &blocks);
        assert_eq!(combination_1, 4);
        let solution_2 = Solution::from_str("#?????").unwrap(); // ??????, 1,2
        let combination_2 = create_combination_with_opt(2, 3, &solution_2.data, &blocks);
        assert_eq!(combination_2, 3);
        let solution_3 = Solution::from_str("#????#").unwrap(); // ??????, 1,3
        let combination_3 = create_combination_with_opt(2, 3, &solution_3.data, &blocks);
        assert_eq!(combination_3, 2);
    }
}

fn main() {
    // let input = include_str!("../input_test_0.txt");
    // let input = include_str!("../input_test_1.txt");
    // let input = include_str!("../input_test.txt");
    let input = include_str!("../input.txt");
    // let input = include_str!("../input_test_perf.txt");
    let sum_combinations: usize = input
        .split_terminator("\n")
        .enumerate()
        .map(|(line_i, line)| {
            // println!("run_for_line {line_i}");
            let mut iter_whitespace = line.split_whitespace();
            let (data, damaged_blocks) = (
                iter_whitespace.next().unwrap(),
                iter_whitespace.next().unwrap(),
            );
            let data_solution_unfolded = Solution::from_str(data).unwrap();
            let mut vec_blocks_unfolded = Vec::new();
            let block_iter = damaged_blocks
                .split(',')
                .map(|s| s.parse::<usize>().unwrap());
            for _ in 0..5 {
                vec_blocks_unfolded.extend(block_iter.clone());
            }
            let num_choices = vec_blocks_unfolded.len() - 1;
            let freedom_range =
                data_solution_unfolded.data.len() - vec_blocks_unfolded.iter().sum::<usize>();
            let combinations = create_combination_with_opt(
                num_choices as usize,
                freedom_range as usize,
                &data_solution_unfolded.data,
                &vec_blocks_unfolded,
            );
            // dbg!(combinations);
            combinations
        })
        .sum();
    dbg!(sum_combinations);
}
