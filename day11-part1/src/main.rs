#![feature(test)]
extern crate test;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let answer = do_aoc(&input);
    println!("{answer}")
}

fn do_aoc(input: &str) -> u64 {
    let lines: Vec<&[u8]> = input.lines().map(|x| x.as_bytes()).collect();
    let mut galaxies = Vec::new();

    let mut y: usize = 0;
    for line in &lines {
        let mut found_galaxy = false;
        for (x, ele) in line.iter().enumerate() {
            if *ele == b'#' {
                found_galaxy = true;

                galaxies.push((x, y));
            }
        }
        if !found_galaxy {
            y += 2;
        } else {
            y += 1;
        }
    }

    let line_len = lines[0].len();
    let mut offset = 0;

    'colloop: for x in 0..line_len {
        for c in lines.iter().map(|line| line[x]) {
            if c == b'#' {
                continue 'colloop;
            }
        }
        for ele in galaxies.iter_mut() {
            if ele.0 > x + offset {
                ele.0 += 1;
            }
        }
        offset += 1;
    }

    galaxies
        .iter()
        .enumerate()
        .map(|(i, (x1, y1))| {
            galaxies
                .iter()
                .skip(i + 1)
                .map(|(x2, y2)| x2.abs_diff(*x1) + y2.abs_diff(*y1))
                .sum::<usize>()
        })
        .sum::<usize>() as u64
}

#[bench]
fn bench_day11(b: &mut test::Bencher) {
    let file = fs::read_to_string("input").unwrap();

    b.iter(|| test::black_box(do_aoc(&file)));
}
