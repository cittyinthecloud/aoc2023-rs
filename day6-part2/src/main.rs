#![feature(test)]
extern crate test;

use memchr::memchr;
use std::fs;


fn main() {
    let input = fs::read_to_string("input").unwrap();
    let answer = do_aoc(&input);
    println!("{answer}");
}

fn do_aoc(input: &str) -> u64 {
    // input.as_bytes is free, implementation is just a transmute.
    let colon = memchr(b':', input.as_bytes()).unwrap();
    let input = &input.as_bytes()[colon..input.len()];

    let mut i = 0;
    let mut time = 0;
    loop {
        let b: Option<&u8> = input.get(i);
        if let Some(b) = b {
            // println!("{} {}", i, *b as char);
            let b = *b;
            if b == b'\n' {
                break;
            } else if u8_is_digit(b) {
                time = time * 10 + u8_to_u64_digit(b);
            }

            i += 1;
        } else {
            break;
        }
    }

    // println!("{time}");

    let colon = memchr(b':', &input[i..input.len()]).unwrap();
    let input = &input[i + colon..input.len()];
    i = 0;
    let mut dist = 0;
    loop {
        let b: Option<&u8> = input.get(i);
        if let Some(b) = b {
            let b = *b;
            if b == b'\n' {
                break;
            } else if u8_is_digit(b) {
                dist = dist * 10 + u8_to_u64_digit(b);
                // println!("{} {} {}", i, b as char, dist);
            }

            i += 1;
        } else {
            break;
        }
    }

    // println!("{dist}");

    let ftime = time as f64;
    let fdist = dist as f64;
    let sqrt = ((ftime*ftime) - (4f64 * fdist)).sqrt();
    let root_1 = (ftime + sqrt) / 2f64;
    let root_2 = (ftime - sqrt) / 2f64;

    integers_between_f64(root_1, root_2)
}

#[inline(always)]
fn u8_is_digit(b: u8) -> bool {
    return b >= b'0' && b <= b'9';
}

#[inline(always)]
fn u8_to_u64_digit(b: u8) -> u64 {
    return (b - b'0') as u64;
}
fn integers_between_f64(a: f64, b: f64) -> u64 {
    let x: f64;
    let y: f64;
    if a <= b {
        x = a;
        y = b;
    } else {
        x = b;
        y = a;
    }

    let trunc_x = x.trunc();
    let trunc_y = y.trunc();

    // println!("{x} {y} {trunc_x} {trunc_y}");

    if trunc_x == trunc_y {
        0
    } else {
        (trunc_y - trunc_x) as u64 - (trunc_y == y) as u64
    }
}

#[bench]
fn bench_day6(b: &mut test::Bencher) {
    let file = fs::read_to_string("input").unwrap();
    
    b.iter(|| test::black_box(do_aoc(&file)));
}