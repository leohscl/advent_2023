fn main() {
    // let input = include_str!("../input_test.txt");
    let input = include_str!("../input.txt");
    let sum_missing: i64 = input
        .split_terminator("\n")
        .map(|line| {
            let mut vec_last_elt: Vec<i64> = Vec::new();
            let mut vec_diff: Vec<i64> = line
                .split_whitespace()
                .map(|n_s| n_s.parse::<i64>().unwrap())
                .collect();
            let mut all_zero = vec_diff
                .iter()
                .fold(true, |acc, &value| acc && (value == 0i64));
            vec_last_elt.push(vec_diff.iter().last().cloned().unwrap());
            while !all_zero {
                vec_diff = construct_diff(vec_diff);
                vec_last_elt.push(vec_diff.iter().last().cloned().unwrap());
                all_zero = vec_diff
                    .iter()
                    .fold(true, |acc, &value| acc && (value == 0i64));
            }
            vec_last_elt.into_iter().sum::<i64>()
        })
        .sum();
    dbg!(sum_missing);
}

fn construct_diff(vec_input: Vec<i64>) -> Vec<i64> {
    vec_input
        .windows(2)
        .map(|chunk| chunk[1] - chunk[0])
        .collect()
}
