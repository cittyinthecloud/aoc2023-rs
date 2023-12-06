use std::fs;

use day5_part2::do_aoc;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let answer = do_aoc(&input);
    println!("{answer}");
}
