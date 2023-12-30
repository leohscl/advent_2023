fn main() {
    let input = include_str!("../input.txt");
    dbg!(part_1(input));
}

fn part_1(input: &str) -> String {
    let mut iter_lines = input.split_terminator("\n");
    let time_line = iter_lines.next().unwrap();
    let distance_line = iter_lines.next().unwrap();
    let times = time_line
        .split_whitespace()
        .skip(1)
        .map(|time_str| time_str.parse::<u32>().unwrap());
    let distances = distance_line
        .split_whitespace()
        .skip(1)
        .map(|distance_str| distance_str.parse::<u32>().unwrap());
    let result = times
        .zip(distances)
        .map(|(time, distance)| {
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
            last_passing - first_passing + 1
        })
        .fold(1, |acc, value| acc * value);
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
        assert_eq!(result, "288".to_string());
    }
}
