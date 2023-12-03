#![feature(test)]
extern crate test;

use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    
    println!("{}", do_aoc(&input));
}

// const RED_IN_BAG: u8 = 12;
// const GREEN_IN_BAG: u8 = 13;
// const BLUE_IN_BAG: u8 = 14;

fn do_aoc(input: &str) -> usize {
    let mut sum: usize = 0;

    for line in input.lines() {
        let line = &line[line.find(':').unwrap()+1..line.len()];
        // println!("{}",line);
        let mut red_count:  u8 = 0;
        let mut green_count: u8 = 0;
        let mut blue_count: u8 = 0;

        for score in line.split([',',';']) {
            let (count_str, color) = score[1..score.len()].split_once(' ').unwrap();
            
            let count = count_str.bytes()
                .map(|x| x - b'0')
                .fold(0, |acc, d| (acc * 10) + d);

            match color {
                "red" => if red_count < count {red_count = count;},
                "blue" =>  if blue_count < count {blue_count = count;}, 
                _ => if green_count < count {green_count = count;}
            }
        }
        
        sum += (red_count as usize)*(green_count as usize)*(blue_count as usize);
    }

    sum
}

#[bench]
fn bench_day2(b: &mut test::Bencher) {
    let file = fs::read_to_string("input").unwrap();
    
    b.iter(|| do_aoc(&file));
}