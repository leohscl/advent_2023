use std::collections::HashMap;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let hash_maxes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    // let input = include_str!("../input_test.txt");
    let input = include_str!("../input.txt");
    let sum_id: u32 = input
        .split_terminator("\n")
        .filter_map(|line| {
            let mut iterator_split_col = line.split(":");
            let opt_id: Option<u32> = iterator_split_col
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .map(|e| e.parse().unwrap());
            let remaining_line = iterator_split_col.next().unwrap();
            let valid = remaining_line
                .split([',', ';'])
                .map(|show_str| {
                    let mut iter_show = show_str.split_whitespace();
                    let (number, color) = (iter_show.next().unwrap(), iter_show.next().unwrap());
                    number.parse::<u32>().unwrap() <= *hash_maxes.get(color).unwrap()
                })
                .fold(true, |acc, new| acc && new);
            if valid {
                opt_id
            } else {
                None
            }
        })
        .sum();
    dbg!(sum_id);
}

fn part_2() {
    // let input = include_str!("../input_test.txt");
    let input = include_str!("../input.txt");
    let sum_id: u32 = input
        .split_terminator("\n")
        .map(|line| {
            let mut hash_maxes = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
            let iterator_split_col = line.split(":");
            let remaining_line = iterator_split_col.last().unwrap();
            remaining_line.split([',', ';']).for_each(|show_str| {
                let mut iter_show = show_str.split_whitespace();
                let (number, color) = (iter_show.next().unwrap(), iter_show.next().unwrap());
                let max = number
                    .parse::<u32>()
                    .unwrap()
                    .max(*hash_maxes.get(color).unwrap());
                hash_maxes.entry(color).and_modify(|entry| *entry = max);
            });
            hash_maxes.values().fold(1, |acc, val| acc * val)
        })
        .sum();
    dbg!(sum_id);
}
