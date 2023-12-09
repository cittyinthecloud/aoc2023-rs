#![feature(iter_collect_into)]
#![feature(test)]
extern crate test;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let answer = do_aoc(&input);
    println!("{answer}");
}

fn do_aoc(input: &str) -> i32 {
    let mut sum = 0;
    let mut nums: Vec<i32> = Vec::new();
    for line in input.lines() {
        line.split(' ').map(|str_num| str_num.parse::<i32>().unwrap()).rev().collect_into(&mut nums);

        let mut iter_len = nums.len()-1;

        loop {
            let mut finished = true;
            let mut first_num: Option<i32> = None;

            for i in 0..iter_len {

                let diff = nums[i+1] - nums[i];
                if finished {
                    if let Some(first_num) = first_num {
                        if diff != first_num {
                            finished = false;
                        }
                    } else {
                        first_num = Some(diff);
                    }
                }

                nums[i] = diff;
            }

            iter_len -= 1;

            if finished { break; }
        }

        // Why is the type annotation needed here lol.
        let ans: i32 = nums.iter().skip(iter_len).sum();
        // println!("{nums:?} {iter_len} {ans}");

        sum += ans;

        nums.clear();
    }
    // todo!()
    sum
}

#[bench]
fn bench_day9(b: &mut test::Bencher) {
    let file = fs::read_to_string("input").unwrap();

    b.iter(|| test::black_box(do_aoc(&file)));
}
