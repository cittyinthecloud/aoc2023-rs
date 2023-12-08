#![feature(test)]
extern crate test;
use memchr::memchr;
use std::fs;
struct NumberBitSet([u64; 2]);

impl NumberBitSet {
    #[inline(always)]
    fn new() -> Self {
        Self([0, 0])
    }

    #[inline(always)]
    fn contains(&mut self, i: u8) -> bool {
        self.0[((i as usize & 64) != 0) as usize] & (1 << (i & 63)) != 0
    }

    #[inline(always)]
    fn set(&mut self, i: u8) {
        self.0[((i as usize & 64) != 0) as usize] |= 1 << (i & 63);
    }

    #[inline(always)]
    fn clear(&mut self) {
        self.0 = [0, 0];
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();

    println!("{}", do_aoc(&input));
}

fn do_aoc(input: &str) -> i32 {
    let mut sum = 0;
    let mut bit_set = NumberBitSet::new();
    for line in input.lines() {
        let mut num = 0;
        let mut parsing_our_card = false;
        let mut res = 0;
        //println!("{line}");

        let memchr = memchr(b':', line.as_bytes()).unwrap();
        for b in line[memchr + 1..line.len()].bytes() {
            if !parsing_our_card {
                if b == b'|' {
                    //println!("end of winning numbers");
                    parsing_our_card = true;
                } else if b == b' ' {
                    if num != 0 {
                        //println!("adding {} to set", num);
                        bit_set.set(num);
                        num = 0;
                    }
                } else if b >= b'0' && b <= b'9' {
                    //println!("num: {}, d: {}", num, b as char);
                    num = num * 10 + (b - b'0');
                }
            } else if b == b' ' {
                if bit_set.contains(num) {
                    res = 1.max(res * 2)
                }
                num = 0;
            } else if b >= b'0' && b <= b'9' {
                num = num * 10 + (b - b'0');
            }
        }

        if bit_set.contains(num) {
            res = 1.max(res * 2)
        }
        //println!("{res}");
        sum += res;
        bit_set.clear();
    }
    sum
}

#[bench]
fn bench_day4(b: &mut test::Bencher) {
    let file = fs::read_to_string("input").unwrap();

    b.iter(|| do_aoc(&file));
}
