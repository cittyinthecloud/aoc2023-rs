#![feature(test)]
extern crate test;

use std::{fs, str::Lines};

use regex::Regex;

fn main() {
    let file = fs::read_to_string("input").unwrap();
    let lines = file.lines();
    //.map(|x| x.to_string()).collect();
    do_aoc(lines);
}

fn do_aoc(lines: Lines) {
    let mut sum = 0;
    let pattern = Regex::new("[1-9]|(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let reverse_pattern =
        Regex::new("[1-9]|(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();

    for line in lines {
        let line = line;

        let first_digit_match = pattern.find(&line).unwrap();

        let first_digit = if first_digit_match.len() == 1 {
            first_digit_match.as_str().parse::<i32>().unwrap()
        } else {
            match first_digit_match.as_str() {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => panic!("Not digit?"),
            }
        };

        let rev_line = line.chars().rev().collect::<String>();
        let last_digit_match = reverse_pattern.find(rev_line.as_str()).unwrap();

        let last_digit = if last_digit_match.len() == 1 {
            last_digit_match.as_str().parse::<i32>().unwrap()
        } else {
            match last_digit_match.as_str() {
                "eno" => 1,
                "owt" => 2,
                "eerht" => 3,
                "ruof" => 4,
                "evif" => 5,
                "xis" => 6,
                "neves" => 7,
                "thgie" => 8,
                "enin" => 9,
                _ => panic!("Not digit?"),
            }
        };

        let calib = first_digit * 10 + last_digit;
        sum += calib
    }
    println!("{}", sum)
}

#[bench]
fn bench_day1(b: &mut test::Bencher) {
    let file = fs::read_to_string("input").unwrap();
    let lines = file.lines();

    b.iter(|| do_aoc(lines.clone()));
}
