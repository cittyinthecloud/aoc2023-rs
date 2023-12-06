#![feature(iter_array_chunks)]

use itertools::Itertools;
use rayon::prelude::*;

pub fn do_aoc(input: &str) -> u32 {
    let seeds: Vec<_> = input
        .lines()
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<u32>().expect("Seeds should be valid numbers"))
        .array_chunks::<2>()
        .map(|[r1, r2]| r1..r2 + r1)
        .collect();

    let sections: Vec<Vec<(u32, u32, u32)>> = input
        .split("\n\n")
        .skip(1)
        .map(|section| {
            section
                .lines()
                .skip(1)
                .map(|line| {
                    let x: (u32, u32, u32) = line
                        .split(' ')
                        .take(3)
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    x
                })
                .collect()
        })
        .collect_vec();

    const LANES: usize = 16;
    *(0..u32::MAX)
        .array_chunks::<LANES>()
        .par_bridge()
        .find_first(|loc| {
            for l in loc {
                let mut seed = *l;

                'sectionloop: for section in sections.iter().rev() {
                    for range in section {
                        if (range.0..range.0 + range.2).contains(&seed) {
                            seed = (seed - range.0) + range.1;
                            continue 'sectionloop;
                        }
                    }
                }

                for seed_range in &seeds {
                    if seed_range.contains(&seed) {
                        return true;
                    }
                }
            }
            false
        })
        .unwrap()
        .iter()
        .find(|loc| {
            let mut seed = **loc;

            'sectionloop: for section in sections.iter().rev() {
                for range in section {
                    if (range.0..range.0 + range.2).contains(&seed) {
                        seed = (seed - range.0) + range.1;
                        continue 'sectionloop;
                    }
                }
            }

            for seed_range in &seeds {
                if seed_range.contains(&seed) {
                    return true;
                }
            }
            false
        })
        .unwrap()
}
