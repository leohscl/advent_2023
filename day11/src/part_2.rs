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
    fn get_expansion(&self) -> (Vec<usize>, Vec<usize>) {
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
        (indexes_duplication_row, indexes_duplication_col)
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
    let (indicies_col, indicies_row) = universe.get_expansion();
    let coordinates_galaxy = universe.get_galaxy_coords();
    let sum_paths: i64 = coordinates_galaxy
        .iter()
        .combinations(2)
        .map(|chunk_g| {
            let g1 = chunk_g[0];
            let g2 = chunk_g[1];
            let min_col = g1.0.min(g2.0);
            let max_col = g1.0.max(g2.0);
            let min_row = g1.1.min(g2.1);
            let max_row = g1.1.max(g2.1);
            let path_len = (g1.0 - g2.0).abs() + (g1.1 - g2.1).abs();
            let crossed_row = (min_row..max_row)
                .filter(|i| indicies_row.contains(&(*i as usize)))
                .count();
            let crossed_col = (min_col..max_col)
                .filter(|i| indicies_col.contains(&(*i as usize)))
                .count();
            path_len + ((crossed_row + crossed_col) * (1000000 - 1)) as i64
        })
        .sum();
    dbg!(sum_paths);
}
