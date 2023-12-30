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

    fn is_ratio(&self, index: usize) -> bool {
        let mut vector_indicies_check = Vec::new();
        let start_h = index / self.width;
        let start_w = index % self.width;
        let max_width = self.width - 1;
        let max_height = self.height - 1;
        // check left
        match start_w {
            0 => (),
            _ => vector_indicies_check.push(start - 1),
        };
        // check top
        match start_h {
            0 => (),
            _ => vector_indicies_check.push(start + self.width),
        };
        // check right
        match start_w {
            w if w == max_width => (),
            _ => vector_indicies_check.push(end + 1),
        };
        // check bottom
        match start_h {
            h if h == max_height => (),
            _ => vector_indicies_check.push(start - self.width),
        };
        dbg!(&vector_indicies_check);
        match (start_w, start_h) {
            (0, _) => (),
            (_, 0) => (),
            _ => vector_indicies_check.push(start - 1 - self.width),
        }
        // check top_right
        match (end_w, start_h) {
            (w, _) if w == max_width => (),
            (_, 0) => (),
            _ => vector_indicies_check.push(end + 1 - self.width),
        }
        // check bottom_right
        match (end_w, end_h) {
            (w, _) if w == max_width => (),
            (_, h) if h == max_height => (),
            _ => vector_indicies_check.push(end + 1 + self.width),
        }
        // check bottom_left
        match (start_w, end_h) {
            (0, _) => (),
            (_, h) if h == max_height => (),
            _ => vector_indicies_check.push(start - 1 + self.width),
        }
        vector_indicies_check
            .into_iter()
            .map(|index| {
                let has_digit = self.engine[index].to_digit(10).is_some();
                if has_digit {
                    let index_w = index % max_width;
                    if index_w + 1 == max_width || (index_w == start_w + 1) {
                        1
                    } else if self.engine[index + 1].to_digit().is_none() {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                }
            })
            .sum()
    }
}

fn main() {
    let input = include_str!("../input_test.txt");
    // let input = include_str!("../input.txt");
    let width = input.split("\n").next().unwrap().chars().count();
    let height = input.split_terminator("\n").count();
    let mut engine = input.chars().collect::<Vec<_>>();
    engine.retain(|&c| c != '\n');
    let grid = Grid {
        height,
        width,
        engine,
    };
    let mut vec_all_num: Vec<_> = (0..grid.height)
        .flat_map(|at_height| grid.find_num(at_height).into_iter())
        .collect();
    vec_all_num.retain(|pair| grid.is_part_number(pair));
    let sum_part: u32 = vec_all_num
        .into_iter()
        .map(|pair| {
            let start = pair.0;
            let end = pair.1;
            let number_string = &grid.engine[start..=end].into_iter().collect::<String>();
            number_string.parse::<u32>().unwrap()
        })
        .sum();
    dbg!(&sum_part);
}
