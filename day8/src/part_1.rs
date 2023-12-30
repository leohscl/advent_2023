use std::collections::HashMap;

struct Node {
    left: String,
    right: String,
}

fn main() {
    let input = include_str!("../input_test_1.txt");
    // let input = include_str!("../input_test_2.txt");
    // let input = include_str!("../input.txt");
    let mut iterator_split_paragraph = input.split("\n\n");
    let instructions = iterator_split_paragraph.next().unwrap();
    let nodes_str = iterator_split_paragraph.next().unwrap();
    let nodes: HashMap<&str, Node> = nodes_str
        .split_terminator("\n")
        .map(|line| {
            let (node_name, tuple_str_l_r) = line.split_once(" = ").unwrap();
            let mut iter_string = tuple_str_l_r.split(", ");
            let (mut left, mut right) = (
                iter_string.next().unwrap().to_string(),
                iter_string.next().unwrap().to_string(),
            );
            left.retain(|c| c != '(');
            right.retain(|c| c != ')');
            (node_name, Node { left, right })
        })
        .collect();
    let starting_node = &nodes["AAA"];
    let num_instructions = get_num_instructions(starting_node, instructions, &nodes);
    dbg!(num_instructions);
}

fn get_num_instructions(
    starting_node: &Node,
    instructions: &str,
    nodes: &HashMap<&str, Node>,
) -> u32 {
    let mut num_instructions = 0;
    let mut current_node = starting_node;
    for c in instructions.chars().cycle() {
        let next_str = match c {
            'L' => current_node.left.as_str(),
            'R' => current_node.right.as_str(),
            _ => panic!("Wrog instruction"),
        };
        num_instructions += 1;
        if next_str == "ZZZ" {
            break;
        } else {
            current_node = &nodes[next_str];
        }
    }
    num_instructions
}
