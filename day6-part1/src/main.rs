#![feature(test)]
extern crate test;

use memchr::memchr;
use std::fs;


fn main() {
    let input = fs::read_to_string("input").unwrap();
    let answer = do_aoc(&input);
    println!("{answer}");
    // let mut times = Vec::new();
    // for i in 0..1000000 {
    //     let start = Instant::now();
    //     let answer = do_aoc(&input);
    //     black_box(answer); // LLVM stop optimizing away my call.
    //     let end = Instant::now();

    //     times.push((end-start).as_nanos())
    //     // println!("{answer}");    
    // }
    // println!("{}",times.iter().sum::<u128>()/times.len() as u128)
}

fn do_aoc(input: &str) -> u32 {
    // input.as_bytes is free, implementation is just a transmute.
    let colon = memchr(b':', input.as_bytes()).unwrap();
    let input = &input.as_bytes()[colon..input.len()];
    let mut cases: [(Option<u32>, Option<u32>); 4] = [(None, None); 4];

    let mut i = 0;
    let mut cur_case: usize = 0;
    loop {
        let b: Option<&u8> = input.get(i);
        if let Some(b) = b {
            let b = *b;
            // println!("{}: {}",i, b as char);
            if b == b'\n' {
                break;
            } else if u8_is_digit(b) {
                if u8_option_is_digit(input.get(i + 1)) {
                    if u8_option_is_digit(input.get(i + 2)) {
                        if u8_option_is_digit(input.get(i + 3)) {
                            cases[cur_case].0 = Some(
                                ((u8_to_u32_digit(b) * 1000) + u8_to_u32_digit(input[i + 1]) * 100)
                                    + (u8_to_u32_digit(input[i + 2]) * 10)
                                    + u8_to_u32_digit(input[i + 3]),
                            );
                            cur_case += 1;
                            i += 4;
                            continue;
                        } else {
                            cases[cur_case].0 = Some(
                                ((u8_to_u32_digit(b) * 100) + u8_to_u32_digit(input[i + 1]) * 10)
                                    + u8_to_u32_digit(input[i + 2]),
                            );
                            cur_case += 1;
                            i += 3;
                            continue;
                        }
                    } else {
                        cases[cur_case].0 =
                            Some((u8_to_u32_digit(b) * 10) + u8_to_u32_digit(input[i + 1]));
                        cur_case += 1;
                        i += 2;
                        continue;
                    }
                } else {
                    cases[cur_case].0 = Some(u8_to_u32_digit(b));
                    cur_case += 1;
                }
            }

            i += 1;
        } else {
            break;
        }
    }

    let colon = memchr(b':', &input[i..input.len()]).unwrap();
    let input = &input[i + colon..input.len()];
    i = 0;
    cur_case = 0;
    loop {
        let b = input.get(i);
        if let Some(b) = b {
            let b = *b;
            if u8_is_digit(b) {
                if u8_option_is_digit(input.get(i + 1)) {
                    if u8_option_is_digit(input.get(i + 2)) {
                        if u8_option_is_digit(input.get(i + 3)) {
                            cases[cur_case].1 = Some(
                                ((u8_to_u32_digit(b) * 1000) + u8_to_u32_digit(input[i + 1]) * 100)
                                    + (u8_to_u32_digit(input[i + 2]) * 10)
                                    + u8_to_u32_digit(input[i + 3]),
                            );
                            cur_case += 1;
                            i += 4;
                            continue;
                        } else {
                            cases[cur_case].1 = Some(
                                ((u8_to_u32_digit(b) * 100) + u8_to_u32_digit(input[i + 1]) * 10)
                                    + u8_to_u32_digit(input[i + 2]),
                            );
                            cur_case += 1;
                            i += 3;
                            continue;
                        }
                    } else {
                        cases[cur_case].1 =
                            Some((u8_to_u32_digit(b) * 10) + u8_to_u32_digit(input[i + 1]));
                        cur_case += 1;
                        i += 2;
                        continue;
                    }
                } else {
                    cases[cur_case].1 = Some(u8_to_u32_digit(b));
                    cur_case += 1;
                }
            }
            i += 1;
        } else {
            break;
        }
    }

    let mut solution = 1;

    for case in cases {
        if let Some(time) = case.0 {
            let dist = case.1.unwrap();

            let ftime = time as f32;
            let fdist = dist as f32;

            let sqrt = ((ftime * ftime) - (4f32 * fdist)).sqrt();
            let root_1 = (ftime + sqrt) / 2f32;
            let root_2 = (ftime - sqrt) / 2f32;

            solution *= integers_between_f32(root_1, root_2);
        } else {
            break;
        }
    }

    // println!("{:?}", cases);
    return solution;
}

#[inline(always)]
fn u8_is_digit(b: u8) -> bool {
    return b >= b'0' && b <= b'9';
}

#[inline(always)]
fn u8_option_is_digit(b: Option<&u8>) -> bool {
    if let Some(b) = b {
        *b >= b'0' && *b <= b'9'
    } else {
        false
    }
}

#[inline(always)]
fn u8_to_u32_digit(b: u8) -> u32 {
    return (b - b'0') as u32;
}
fn integers_between_f32(a: f32, b: f32) -> u32 {
    let x: f32;
    let y: f32;
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
        (trunc_y - trunc_x) as u32 - (trunc_y == y) as u32
    }
}

#[bench]
fn bench_day6(b: &mut test::Bencher) {
    let file = fs::read_to_string("input").unwrap();
    
    b.iter(|| test::black_box(do_aoc(&file)));
}