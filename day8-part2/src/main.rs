#![feature(test)]
extern crate test;
use std::fs;

use gcd::Gcd;

#[derive(Default, Debug, Clone, Copy)]
struct Node<'a> {
    left_name: &'a str,
    right_name: &'a str,
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

    ((((name_bytes[0] - b'A') as usize * 32) + (name_bytes[1] - b'A') as usize) * 32)
        + (name_bytes[2] - b'A') as usize
}

fn do_aoc(input: &str) -> u64 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    lines.next(); // throw away the blank line

    let mut nodes = [Node::default(); 32 * 32 * 32];
    let mut cur_node_names = Vec::new();
    for line in lines {
        let (name, node) = Node::from_line(line);
        nodes[node_name_to_index(name)] = node;
        if name.as_bytes()[2] == b'A' {
            cur_node_names.push(name);
        }
    }

    // let ins_bytes = instructions.as_bytes();
    // let mut i = vec![0; cur_node_names.len()];
    // loop {
    //     let node_name = cur_node_names[0];
    //     let new_node_name = advance_by_steps(node_name, nodes, ins_bytes, i[0], i[0]+1);
    //     i[0] += 1;
    //     cur_node_names[0] = new_node_name;
    //     if new_node_name.as_bytes()[2] == b'Z' {
    //         cur_node_names[1] = advance_by_steps(cur_node_names[1], nodes, ins_bytes, i[1], i[0]);
    //         i[1] = i[0];
    //         if cur_node_names[1].as_bytes()[2] == b'Z' {
    //             cur_node_names[2] = advance_by_steps(cur_node_names[2], nodes, ins_bytes, i[2], i[1]);
    //             i[2] = i[1];
    //             println!("{:?} {:?}", i, cur_node_names);
    //             if cur_node_names[2].as_bytes()[2] == b'Z' {
    //                 cur_node_names[3] = advance_by_steps(cur_node_names[3], nodes, ins_bytes, i[3], i[2]);
    //                 i[3] = i[2];
    //                 if cur_node_names[3].as_bytes()[2] == b'Z' {
    //                     cur_node_names[4] = advance_by_steps(cur_node_names[4], nodes, ins_bytes, i[4], i[3]);
    //                     i[4] = i[3];
    //                     if cur_node_names[4].as_bytes()[2] == b'Z' {
    //                         return i[4] as u64;
    //                     }
    //                 }
    //             }
    //         }
    //     }

    // Trying LCM based on output from the above ^.

    cur_node_names
        .iter()
        .map(|node_name| {
            let mut node_name = *node_name;
            let mut steps = 0;
            loop {
                let node = nodes[node_name_to_index(node_name)];
                let inst = instructions.as_bytes()[steps % instructions.len()];
                node_name = if inst == b'L' {
                    node.left_name
                } else {
                    node.right_name
                };
                steps += 1;
                if node_name.as_bytes()[2] == b'Z' {
                    break;
                }
            }

            steps as u64
        })
        .fold(1, lcm)
}

// Algorithms class topics? In *my* AOC answers ?!?!
fn lcm(a: u64, b: u64) -> u64 {
    a * b / a.gcd_binary(b)
}

// fn gcd(a: u64, b: u64) -> u64 {
//     let mut a = a;
//     let mut b = b;

//     while a != b {
//         if a > b {
//             a = a - b;
//         } else {
//             b = b - a;
//         }
//     }

//     a
// }

// This cat's name is Binary GCD algorithm look him up on Wikipedia:
// // https://en.wikipedia.org/wiki/Binary_GCD_algorithm
// fn gcd(a: u64, b: u64) {

// }

// fn advance_by_steps<'a>(node_name: &'a str, nodes: [Node<'a>; 32*32*32], instructions: &[u8], start_index: usize, end_index: usize) -> &'a str {
//     let mut node_name = node_name;
//     for i in start_index..=end_index {
//         let inst = instructions[i%instructions.len()];
//         let node_index = node_name_to_index(node_name);
//         node_name = if inst == b'L' {
//             nodes[node_index].left_name
//         } else {
//             nodes[node_index].right_name
//         };

//     }

//     node_name
// }

#[bench]
fn bench_day8(b: &mut test::Bencher) {
    let file = fs::read_to_string("input").unwrap();

    b.iter(|| test::black_box(do_aoc(&file)));
}
