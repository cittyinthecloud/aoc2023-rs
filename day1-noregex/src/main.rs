#![feature(test)]
extern crate test;

use std::{fs, str::Lines};

fn main() {
    let file = fs::read_to_string("input").unwrap();
    let lines = file.lines();
    do_aoc(lines);
}

const NUMBERS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn do_aoc(lines: Lines) {
    let mut sum = 0;

    for line in lines {
        let mut val = 0;
        let bytes_line = line.as_bytes();

        'fwd_loop: for i in 0..bytes_line.len() {
            let c = bytes_line[i];
            if c > b'0' && b':' > c {
                val = ((c - b'0') as i32) * 10;
                break;
            }

            for (d, name) in NUMBERS.iter().enumerate() {
                if let Some(slice) = bytes_line.get(i..i + name.len()) {
                    if *name == slice {
                        val = 10 * (d as i32 + 1);
                        break 'fwd_loop;
                    }
                }
            }
        }

        'rev_loop: for i in (0..bytes_line.len()).rev() {
            let c = bytes_line[i];
            if c > b'0' && b':' > c {
                val += (c - b'0') as i32;
                break;
            }

            for (d, name) in NUMBERS.iter().enumerate() {
                if i + 1 < name.len() {
                    continue;
                }
                if let Some(slice) = bytes_line.get(i + 1 - name.len()..=i) {
                    if *name == slice {
                        val += d as i32 + 1;
                        break 'rev_loop;
                    }
                }
            }
        }

        sum += val
    }

    println!("{}", sum)
}

#[bench]
fn bench_day1(b: &mut test::Bencher) {
    let file = fs::read_to_string("input").unwrap();
    let lines = file.lines();

    b.iter(|| do_aoc(lines.clone()));
}
