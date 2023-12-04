#![feature(test)]
extern crate test;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    println!("{}",do_aoc(&input))
}


struct Number {
    start: usize,
    end: usize,
    number: usize,
}

fn do_aoc(input: &str) -> usize { 
    let mut sum: usize = 0;
        
    let mut numbers = Vec::new(); 
    let mut gears = Vec::new();
    let mut numbers_cutoffs = [(0,0);141];

    let mut last_cutoff = 0;
    for (y, row) in input.lines().enumerate() {
        let mut start_of_number : usize = usize::MAX;
        let mut end_of_number: usize = usize::MAX;
        let mut number = 0;
        for (x, b) in row.bytes().enumerate() {
            if b >= b'0' && b <= b'9' {
                
                if start_of_number == usize::MAX {
                    start_of_number = x;
                } 
                
                end_of_number = x;
                number = (number * 10) + (b - b'0') as usize;
                
            } else {
                if start_of_number != usize::MAX {
                    numbers.push(Number { start: start_of_number, end: end_of_number, number: number });
    
                    number = 0;
                    start_of_number = usize::MAX;
                    end_of_number = usize::MAX;
                }

                if b == b'*' {
                    gears.push((x,y))
                }
            } 
        }

        if start_of_number != usize::MAX {
            numbers.push(Number { start: start_of_number, end: end_of_number, number: number});
        }

        numbers_cutoffs[y] = (last_cutoff,numbers.len());
        last_cutoff = numbers.len();
    }

    numbers_cutoffs[140]=(numbers.len(),numbers.len());

    'gearloop:for (gear_x, gear_y) in gears {
        let mut product = 1;
        let mut count = 0;

        
        let pos_numbers = &numbers[numbers_cutoffs[gear_y.saturating_sub(1)].0..numbers_cutoffs[gear_y+1].1];
        for n in pos_numbers {
            if (n.start.saturating_sub(1)) <= gear_x && (n.end + 1) >= gear_x {
                if count == 2 {
                    continue 'gearloop; // Gear went over 2, no reason to keep counting.
                }
                count += 1;
                product *= n.number;
            }
        }

        if count == 2 {
            sum += product;
        }
    }
    
    sum
}

#[bench]
fn bench_day3(b: &mut test::Bencher) {
    let file = fs::read_to_string("input").unwrap();
    
    b.iter(|| do_aoc(&file));
}