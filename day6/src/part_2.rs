fn main() {
    let input = include_str!("../input.txt");
    dbg!(part_1(input));
}

fn part_1(input: &str) -> String {
    let mut iter_lines = input.split_terminator("\n");
    let time_line = iter_lines.next().unwrap();
    let distance_line = iter_lines.next().unwrap();
    let time: u64 = time_line
        .split(":")
        .skip(1)
        .map(|s| {
            let mut vec_chars: Vec<_> = s.to_string().chars().collect();
            vec_chars.retain(|&c| c != ' ');
            vec_chars.into_iter().collect::<String>().parse().unwrap()
        })
        .next()
        .unwrap();
    let distance: u64 = distance_line
        .split(":")
        .skip(1)
        .map(|s| {
            let mut vec_chars: Vec<_> = s.to_string().chars().collect();
            vec_chars.retain(|&c| c != ' ');
            vec_chars.into_iter().collect::<String>().parse().unwrap()
        })
        .next()
        .unwrap();
    let first_passing = (0..)
        .find_map(|time_push| {
            if time_push * (time - time_push) > distance {
                Some(time_push)
            } else {
                None
            }
        })
        .unwrap();
    let last_passing = (0..time)
        .rev()
        .find_map(|time_push| {
            if time_push * (time - time_push) > distance {
                Some(time_push)
            } else {
                None
            }
        })
        .unwrap();
    let result = last_passing - first_passing + 1;
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_test() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = part_1(input);
        assert_eq!(result, "71503".to_string());
    }
}
