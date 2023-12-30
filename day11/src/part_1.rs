use itertools::Itertools;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone)]
enum Element {
    Empty,
    Galaxy,
}

impl Element {
    fn from_char(c: char) -> Element {
        match c {
            '.' => Element::Empty,
            '#' => Element::Galaxy,
            _ => panic!(),
        }
    }
}

struct Universe {
    width: i64,
    elements: Vec<Vec<Element>>,
}

impl FromStr for Universe {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let height = s.split_terminator("\n").count();
        let width = s.split_terminator("\n").next().unwrap().chars().count();
        let elements_all_vec: Vec<_> = s
            .chars()
            .filter_map(|c| {
                if c == '\n' {
                    None
                } else {
                    Some(Element::from_char(c))
                }
            })
            .collect();
        let elements = elements_all_vec
            .chunks(width)
            .map(|chunk| chunk.into_iter().cloned().collect())
            .collect();
        Ok(Universe {
            elements,
            width: width as i64,
        })
    }
}

impl Universe {
    fn expand(&self) -> Universe {
        let indexes_duplication_row: Vec<_> = self
            .elements
            .iter()
            .enumerate()
            .filter_map(|(i, row)| {
                if row.into_iter().all(|e| e == &Element::Empty) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();
        let indexes_duplication_col: Vec<_> = (0..self.elements.len())
            .into_iter()
            .filter_map(|i| {
                if self
                    .elements
                    .iter()
                    .flatten()
                    .skip(i)
                    .step_by(self.width as usize)
                    .all(|e| e == &Element::Empty)
                {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();
        let mut elements = Vec::new();
        let width = self.width;
        for col_i in 0..self.elements.len() {
            let mut row_vec = Vec::new();
            for row_i in 0..self.elements[0].len() {
                row_vec.push(self.elements[col_i][row_i].clone());
                if indexes_duplication_col.contains(&row_i) {
                    row_vec.push(Element::Empty);
                }
            }
            elements.push(row_vec.clone());
            if indexes_duplication_row.contains(&col_i) {
                elements.push(row_vec.clone());
            }
        }
        Universe {
            elements,
            width: width + indexes_duplication_col.len() as i64,
        }
    }

    fn get_galaxy_coords(&self) -> Vec<(i64, i64)> {
        self.elements
            .iter()
            .flatten()
            .enumerate()
            .filter_map(|(i, e)| if e == &Element::Galaxy { Some(i) } else { None })
            .map(|index| (index as i64 / self.width, index as i64 % self.width))
            .collect()
    }

    fn print_universe(&self) {
        for row in self.elements.iter() {
            let row_str: String = row
                .into_iter()
                .map(|e| match e {
                    Element::Empty => '.',
                    Element::Galaxy => '#',
                })
                .collect();
            println!("{row_str}");
        }
    }
}
fn main() {
    // let input = include_str!("../input_test.txt");
    let input = include_str!("../input.txt");
    let universe = Universe::from_str(input).unwrap();
    universe.print_universe();
    let universe_expanded = universe.expand();
    universe_expanded.print_universe();
    let coordinates_galaxy = universe_expanded.get_galaxy_coords();
    dbg!(&coordinates_galaxy);
    let sum_paths: i64 = coordinates_galaxy
        .iter()
        .enumerate()
        .combinations(2)
        .map(|chunk_g| {
            let (i1, g1) = chunk_g[0];
            let (i2, g2) = chunk_g[1];
            // dbg!(i1 + 1);
            // dbg!(i2 + 1);
            // dbg!(g1.0);
            // dbg!(g2.0);
            // dbg!(g1.1);
            // dbg!(g2.1);
            // dbg!(i2 + 1);
            let path_len = (g1.0 - g2.0).abs() + (g1.1 - g2.1).abs();
            // dbg!(path_len);
            path_len
        })
        .sum();
    dbg!(sum_paths);
}
