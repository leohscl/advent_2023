struct Adder {
    start_dest: u64,
    start_src: u64,
    range: u64,
}
struct Converter {
    vector_map: Vec<Adder>,
}

impl Converter {
    fn convert(&self, value: u64) -> u64 {
        let mut converted_result = value;
        for adder in self.vector_map.iter() {
            if adder.start_src <= value && value < adder.start_src + adder.range {
                let diff = value - adder.start_src;
                converted_result = adder.start_dest + diff;
            }
        }
        converted_result
    }
}

fn main() {
    // let input = include_str!("../input_test.txt");
    let input = include_str!("../input.txt");
    let mut paragraph_iter = input.split_terminator("\n\n");
    let mut line_1_iter = paragraph_iter.next().unwrap().split_whitespace();
    line_1_iter.next();
    let starting_ranges: Vec<u64> = line_1_iter.map(|s| s.parse::<u64>().unwrap()).collect();
    let iterator_starting_values = starting_ranges.chunks(2).flat_map(|chunk| {
        let start = chunk[0];
        let size = chunk[1];
        start..(start + size)
    });
    let vec_converter: Vec<Converter> = paragraph_iter
        .map(|paragraph| {
            let mut iter_lines = paragraph.split('\n');
            iter_lines.next();
            let vector_map: Vec<_> = iter_lines
                .map(|line| {
                    let vec_numbers: Vec<u64> = line
                        .split_whitespace()
                        .map(|num_str| num_str.parse::<u64>().unwrap())
                        .collect();
                    Adder {
                        start_dest: vec_numbers[0],
                        start_src: vec_numbers[1],
                        range: vec_numbers[2],
                    }
                })
                .collect();
            Converter { vector_map }
        })
        .collect();
    let smallest_value = iterator_starting_values
        .map(|starting_value| {
            vec_converter
                .iter()
                .fold(starting_value, |value, converter| converter.convert(value))
        })
        .min();
    dbg!(smallest_value);
}
