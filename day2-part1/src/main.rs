#![feature(test)]
extern crate test;

use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    println!("{}", do_aoc(&input));
}

const RED_IN_BAG: u8 = 12;
const GREEN_IN_BAG: u8 = 13;
const BLUE_IN_BAG: u8 = 14;

// const COUNT_TABLE: [u8; 3] = [BLUE_IN_BAG,RED_IN_BAG,GREEN_IN_BAG];

fn do_aoc(input: &str) -> usize {
    let mut sum = 0;

    'mainloop: for (i, line) in input.lines().enumerate() {
        let line = &line[line.find(':').unwrap() + 1..line.len()];
        // println!("{}",line);

        // for score in line.split([',',';']) {
        for score in line.split([',', ';']) {
            let (count_str, color) = score[1..score.len()].split_once(' ').unwrap();

            // This is faster than the loop, and faster than parse::<u8>.
            let count = count_str
                .bytes()
                .map(|x| x - b'0')
                .fold(0, |acc, d| (acc * 10) + d);

            let max_count = match color {
                "red" => RED_IN_BAG,
                "blue" => BLUE_IN_BAG,
                _ => GREEN_IN_BAG,
            };

            if count > max_count {
                continue 'mainloop;
            }
        }

        sum += i + 1;
    }

    sum
}

#[bench]
fn bench_day2(b: &mut test::Bencher) {
    let file = fs::read_to_string("input").unwrap();

    b.iter(|| do_aoc(&file));
}
