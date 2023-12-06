#![feature(test)]
extern crate test;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let answer = do_aoc(&input);
    println!("{answer}");
}

fn do_aoc(input: &str) -> u64 {
    let mut lines = input.lines();

    let first_line = lines.next().expect("Can't parse an empty file");
    let mut seeds: Vec<u64> = first_line
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<u64>().expect("Seeds should be valid numbers"))
        .collect();

    lines.nth(1); // Skip 2 lines.

    let mut ranges: Vec<(u64,u64,u64)> = Vec::new();
    for line in lines {
        if line.is_empty() {
            'elementloop:for ele in seeds.iter_mut() {
                for (dest_range_start, src_range_start, range_len) in ranges.iter().rev() {
                    if (*src_range_start..(*src_range_start)+(*range_len)).contains(ele) {
                        *ele = dest_range_start + (*ele - src_range_start);
                        continue 'elementloop;
                    }    
                }
            }
            ranges.clear();
            continue;
        }
        let first = line.as_bytes()[0];
        if !(first >= b'0' && first <= b'9') {
            continue;
        }

        let mut split: std::str::Split<'_, char> = line.split(' ');
        let dest_range_start = split.next().unwrap().parse::<u64>().unwrap();
        let src_range_start = split.next().unwrap().parse::<u64>().unwrap();
        let range_len = split.next().unwrap().parse::<u64>().unwrap();
        ranges.push((dest_range_start,src_range_start,range_len));
    }

    'elementloop:for ele in seeds.iter_mut() {
        for (dest_range_start, src_range_start, range_len) in ranges.iter().rev() {
            if (*src_range_start..(*src_range_start)+(*range_len)).contains(ele) {
                //println!("{ele} into {}", dest_range_start+(*ele - src_range_start));
                *ele = dest_range_start + (*ele - src_range_start);
                continue 'elementloop;
            }    
        }
    }
    
    *seeds.iter().min().unwrap()
}

#[bench]
fn bench_day5(b: &mut test::Bencher) {
    let file = fs::read_to_string("input").unwrap();
    
    b.iter(|| do_aoc(&file));
}