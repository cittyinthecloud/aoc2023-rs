#![feature(test)]
extern crate test;
use std::{cmp::min, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();

    println!("{}", do_aoc(&input));
}

fn do_aoc(input: &str) -> usize {
    let mut sum: usize = 0;

    let grid: Vec<&[u8]> = input.lines().map(|x| x.as_bytes()).collect();

    for (y, row) in grid.iter().enumerate() {
        let mut start_of_number: usize = usize::MAX;
        let mut end_of_number: usize = usize::MAX;
        let mut number = 0;
        for (x, b) in row.iter().enumerate() {
            let b = *b;
            if b >= b'0' && b <= b'9' {
                if start_of_number == usize::MAX {
                    start_of_number = x;
                }

                end_of_number = x;
                number = (number * 10) + (b - b'0') as usize;
            } else if start_of_number != usize::MAX {
                // We finished a number, symbol check

                for check_x in start_of_number.saturating_sub(1)..=end_of_number + 1 {
                    for check_y in y.saturating_sub(1)..=y + 1 {
                        if check_y >= grid.len() {
                            break;
                        }

                        let c = grid[check_y][check_x];
                        if c != b'.' && !(c >= b'0' && c <= b'9') {
                            sum += number;
                            break;
                        }
                    }
                }
                number = 0;
                start_of_number = usize::MAX;
                end_of_number = usize::MAX;
            }
        }

        if start_of_number != usize::MAX {
            for check_x in start_of_number.checked_sub(1).unwrap_or(0)..=end_of_number {
                for check_y in y.checked_sub(1).unwrap_or(0)..=min(y + 1, grid.len() - 1) {
                    let c = grid[check_y][check_x];
                    if c != b'.' && !(c >= b'0' && c <= b'9') {
                        sum += number;
                        break;
                    }
                }
            }
        }
    }
    sum
}

#[bench]
fn bench_day3(b: &mut test::Bencher) {
    let file = fs::read_to_string("input").unwrap();

    b.iter(|| do_aoc(&file));
}
