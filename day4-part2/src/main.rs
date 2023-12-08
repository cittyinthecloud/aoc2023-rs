#![feature(test)]
extern crate test;
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

fn do_aoc(input: &str) -> u32 {
    let mut cards: [u32; 256] = [0; 256];
    cards[0] = 1;

    // Per line
    let mut bit_set = NumberBitSet::new();
    let mut line_num: usize = 0;
    let mut num: u8 = 0;
    let mut parsing_our_card = false;
    let mut finds = 0;
    let mut past_colon = false;

    for b in input.bytes() {
        if b == b':' {
            past_colon = true;
        } else if b == b'\n' {
            if bit_set.contains(num) {
                cards[line_num + finds + 1] += cards[line_num];
            }
            bit_set.clear();
            num = 0;
            finds = 0;
            parsing_our_card = false;
            line_num += 1;
            cards[line_num] += 1;
            past_colon = false;
        } else if past_colon {
            if !parsing_our_card {
                if b == b'|' {
                    parsing_our_card = true;
                } else if b == b' ' {
                    if num != 0 {
                        bit_set.set(num);
                        num = 0;
                    }
                } else if b >= b'0' && b <= b'9' {
                    num = num * 10 + (b - b'0');
                }
            } else if b == b' ' {
                if bit_set.contains(num) {
                    finds += 1;
                    cards[line_num + finds] += cards[line_num];
                }
                num = 0;
            } else if b >= b'0' && b <= b'9' {
                num = num * 10 + (b - b'0');
            }
        }
    }

    // LLVM please

    const LANES: usize = 8;
    let chunks = cards.chunks_exact(16);

    let partial_sums = chunks.fold([0u32; LANES], |mut acc, chunk| {
        for i in 0..LANES {
            acc[i] += chunk[i];
        }
        acc
    });

    partial_sums.iter().sum()
}

#[bench]
fn bench_day4(b: &mut test::Bencher) {
    let file = fs::read_to_string("input").unwrap();

    b.iter(|| do_aoc(&file));
}
