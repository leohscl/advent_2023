use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
enum Terrain {
    Rock,
    Ash,
}

#[derive(Debug)]
struct Pattern {
    height: usize,
    width: usize,
    grid: Vec<Terrain>,
}

impl FromStr for Pattern {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.split_terminator('\n').next().unwrap().chars().count();
        let height = s.split_terminator('\n').count();
        let grid = s
            .chars()
            .filter(|&c| c != '\n')
            .map(|c| match c {
                '#' => Terrain::Rock,
                '.' => Terrain::Ash,
                _ => panic!("char does not translate to Terrain"),
            })
            .collect();
        Ok(Pattern {
            height,
            width,
            grid,
        })
    }
}
impl Pattern {
    fn get_score(&self) -> usize {
        let sum_vertical = (0..self.width - 1)
            .filter_map(|potential_mirror_i| {
                if self.check_mirror_vertical(potential_mirror_i) {
                    Some(potential_mirror_i + 1)
                } else {
                    None
                }
            })
            .sum::<usize>();
        let sum_horizontal = (0..self.height - 1)
            .filter_map(|potential_mirror_i| {
                if self.check_mirror_horizontal(potential_mirror_i) {
                    Some(potential_mirror_i + 1)
                } else {
                    None
                }
            })
            .sum::<usize>();
        // dbg!(sum_horizontal);
        // dbg!(sum_vertical);
        sum_vertical + sum_horizontal * 100
    }

    fn check_mirror_horizontal(&self, index_mirror: usize) -> bool {
        (0..=index_mirror)
            .rev()
            .zip(index_mirror + 1..self.height)
            .all(|(r1, r2)| {
                let start_row_1 = r1 * self.width;
                let start_row_2 = r2 * self.width;
                let it_r1 = start_row_1..;
                let it_r2 = start_row_2..;
                it_r1
                    .zip(it_r2)
                    .take(self.width)
                    // .inspect(|p| {
                    //     dbg!(p);
                    // })
                    .all(|(e1, e2)| self.grid[e1] == self.grid[e2])
            })
    }

    fn check_mirror_vertical(&self, index_mirror: usize) -> bool {
        (0..=index_mirror)
            .rev()
            .zip(index_mirror + 1..self.width)
            .all(|(c1, c2)| {
                let it_c1 = c1..;
                let it_c2 = c2..;
                it_c1
                    .zip(it_c2)
                    .step_by(self.width)
                    .take(self.height)
                    .all(|(e1, e2)| self.grid[e1] == self.grid[e2])
            })
    }
}

fn main() {
    // let input = include_str!("../input_test.txt");
    let input = include_str!("../input.txt");
    let sum_mirrors = input
        .split("\n\n")
        .map(|paragraph| {
            let pattern = Pattern::from_str(paragraph).unwrap();
            pattern.get_score()
        })
        .sum::<usize>();
    dbg!(sum_mirrors);
}
