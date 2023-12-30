use core::str::FromStr;
#[derive(PartialEq, Eq, Debug)]
enum Terrain {
    RoundRock,
    CubeRock,
    Empty,
}

#[derive(Debug)]
struct Reflector {
    height: usize,
    width: usize,
    grid: Vec<Terrain>,
}

impl FromStr for Reflector {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.split_terminator('\n').next().unwrap().chars().count();
        let height = s.split_terminator('\n').count();
        let grid = s
            .chars()
            .filter(|&c| c != '\n')
            .map(|c| match c {
                'O' => Terrain::RoundRock,
                '#' => Terrain::CubeRock,
                '.' => Terrain::Empty,
                _ => panic!("char does not translate to Terrain"),
            })
            .collect();
        Ok(Reflector {
            height,
            width,
            grid,
        })
    }
}

impl Reflector {
    fn tilt_up(&mut self) {
        let index_starts = 0..self.width;
        let increment = self.width;
        let num = self.height;
        self.tilt(index_starts, increment, num);
    }
    fn tilt<I>(&mut self, index_starts: I, increment: usize, num: usize)
    where
        I: Iterator<Item = usize>,
    {
        index_starts.for_each(|index_start| {
            let mut previous_index_solid = index_start;
            (index_start..)
                .step_by(increment)
                .take(num)
                .for_each(|index_elt| match self.grid[index_elt] {
                    Terrain::CubeRock => previous_index_solid = index_elt + increment,
                    Terrain::Empty => (),
                    Terrain::RoundRock => {
                        self.grid[index_elt] = Terrain::Empty;
                        self.grid[previous_index_solid] = Terrain::RoundRock;
                        previous_index_solid += increment;
                    }
                })
        })
    }
    fn score(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .map(|(i, terrain)| {
                if terrain == &Terrain::RoundRock {
                    let line = i / self.width;
                    let score = self.height - line;
                    // dbg!(line, score);
                    score
                } else {
                    0
                }
            })
            .sum()
    }
    fn print_grid_elements(&self) {
        self.grid.chunks(self.width).for_each(|line_elts| {
            let res: String = line_elts
                .into_iter()
                .map(|e| match e {
                    Terrain::RoundRock => 'O',
                    Terrain::CubeRock => '#',
                    Terrain::Empty => '.',
                })
                .collect();
            println!("{res}");
        })
    }
}

fn main() {
    // let input = include_str!("../input_test.txt");
    let input = include_str!("../input.txt");
    let mut reflector = Reflector::from_str(input).unwrap();
    dbg!(reflector.score());
    reflector.tilt_up();
    // reflector.print_grid_elements();
    dbg!(reflector.score());
}
