#![feature(test)]
extern crate test;
use std::fs;

#[derive(Default, Debug, Clone, Copy)]
struct Node<'a> {
    // name: &'a str,
    left_name: &'a str,
    right_name: &'a str,
    // left_node: RefCell<Option<Rc<Node<'a>>>>,
    // right_node: RefCell<Option<Rc<Node<'a>>>>,
}

impl<'a> Node<'a> {
    fn from_line(line: &'a str) -> (&'a str, Self) {
        (
            &line[0..3],
            Self {
                left_name: &line[7..10],
                right_name: &line[12..15],
            },
        )
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let answer = do_aoc(&input);
    println!("{answer}")
}

fn node_name_to_index(name: &str) -> usize {
    let name_bytes = name.as_bytes();

    ((((name_bytes[0] - b'A') as usize * 26) + (name_bytes[1] - b'A') as usize) * 26)
        + (name_bytes[2] - b'A') as usize
}

fn do_aoc(input: &str) -> u64 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    lines.next(); // throw away the blank line

    let mut nodes = [Node::default(); 26 * 26 * 26];
    for line in lines {
        let (name, node) = Node::from_line(line);
        nodes[node_name_to_index(name)] = node;
    }

    let ins_bytes = instructions.as_bytes();
    let mut cur_node = nodes[node_name_to_index("AAA")];
    let mut i = 0;
    loop {
        let instruction = ins_bytes[i % instructions.len()];

        let next_node_name = if instruction == b'L' {
            cur_node.left_name
        } else {
            cur_node.right_name
        };
        i += 1;

        if next_node_name == "ZZZ" {
            break;
        }
        cur_node = nodes[node_name_to_index(next_node_name)];
    }
    i as u64
}

#[bench]
fn bench_day8(b: &mut test::Bencher) {
    let file = fs::read_to_string("input").unwrap();

    b.iter(|| test::black_box(do_aoc(&file)));
}
