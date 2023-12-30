#[derive(Debug)]
struct Grid {
    height: usize,
    width: usize,
    engine: Vec<char>,
}

impl Grid {
    fn find_gear(&self, at_height: usize) -> Vec<usize> {
        let mut current = at_height * self.width;
        let end = (at_height + 1) * self.width;
        let mut vec_res = Vec::new();
        while current != end {
            if self.engine[current] == '*' {
                vec_res.push(current);
            }
            current += 1;
        }
        vec_res
    }

    fn find_num(&self, at_height: usize) -> Vec<(usize, usize)> {
        let mut current = at_height * self.width;
        let end = (at_height + 1) * self.width;
        let mut vec_res = Vec::new();
        while current != end {
            if let Some(_) = self.engine[current].to_digit(10) {
                let start_num = current;
                while current != end {
                    if self.engine[current].to_digit(10).is_none() {
                        break;
                    } else {
                        current += 1;
                    }
                }
                let end_num = current - 1;
                vec_res.push((start_num, end_num));
            } else {
                current += 1;
            }
        }
        vec_res
    }

    fn close_by(&self, index: usize, index_test: usize) -> bool {
        let index_w = index % self.width;
        let index_h = index / self.height;
        let index_test_w = index_test % self.width;
        let index_test_h = index_test / self.height;
        (index_h.max(index_test_h) - index_h.min(index_test_h) <= 1)
            && (index_w.max(index_test_w) - index_w.min(index_test_w) <= 1)
    }

    fn get_gear_ratio(&self, index_gear: usize, all_num: &Vec<(usize, usize)>) -> u32 {
        let mut all_num_clone = all_num.clone();
        all_num_clone
            .retain(|pair| self.close_by(index_gear, pair.0) | self.close_by(index_gear, pair.1));
        if all_num_clone.len() >= 2 {
            all_num_clone
                .into_iter()
                .map(|pair| {
                    let start = pair.0;
                    let end = pair.1;
                    let number_string = &self.engine[start..=end].into_iter().collect::<String>();
                    number_string.parse::<u32>().unwrap()
                })
                .fold(1, |acc, value| acc * value)
        } else {
            0
        }
    }
}

fn main() {
    // let input = include_str!("../input_test.txt");
    let input = include_str!("../input.txt");
    let width = input.split("\n").next().unwrap().chars().count();
    let height = input.split_terminator("\n").count();
    let mut engine = input.chars().collect::<Vec<_>>();
    engine.retain(|&c| c != '\n');
    let grid = Grid {
        height,
        width,
        engine,
    };
    let vec_all_num: Vec<_> = (0..grid.height)
        .flat_map(|at_height| grid.find_num(at_height).into_iter())
        .collect();
    let vec_all_gears: Vec<_> = (0..grid.height)
        .flat_map(|at_height| grid.find_gear(at_height).into_iter())
        .collect();
    let sum_gear_ratio: u32 = vec_all_gears
        .into_iter()
        .map(|index_gear| grid.get_gear_ratio(index_gear, &vec_all_num))
        .sum();
    dbg!(&sum_gear_ratio);
}
