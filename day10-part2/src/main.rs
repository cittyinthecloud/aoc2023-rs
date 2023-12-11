#![feature(test)]
extern crate test;
use memchr::memchr;
use ndarray::Array;
use std::fs;

#[derive(Debug)]
struct Animal {
    x: usize,
    y: usize,
    dir: Direction,
}

impl Animal {
    fn do_move(&mut self, maze: &[&[u8]]) {
        let cur_pipe = maze[self.y][self.x];

        match cur_pipe {
            b'|' => {
                if self.dir == Direction::North {
                    self.y -= 1;
                } else {
                    self.y += 1;
                }
            }
            b'-' => {
                if self.dir == Direction::West {
                    self.x -= 1;
                } else {
                    self.x += 1;
                }
            }
            b'L' => {
                if self.dir == Direction::West {
                    self.y -= 1;
                    self.dir = Direction::North;
                } else {
                    self.x += 1;
                    self.dir = Direction::East;
                }
            }
            b'J' => {
                if self.dir == Direction::East {
                    self.y -= 1;
                    self.dir = Direction::North;
                } else {
                    self.x -= 1;
                    self.dir = Direction::West;
                }
            }
            b'7' => {
                if self.dir == Direction::East {
                    self.y += 1;
                    self.dir = Direction::South;
                } else {
                    self.x -= 1;
                    self.dir = Direction::West
                }
            }
            b'F' => {
                if self.dir == Direction::West {
                    self.y += 1;
                    self.dir = Direction::South
                } else {
                    self.x += 1;
                    self.dir = Direction::East;
                }
            }
            _ => panic!("Nonsensical pipe {}", cur_pipe),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    #[inline(always)]
    fn bump(&self, coords: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (coords.0, coords.1 - 1),
            Direction::South => (coords.0, coords.1 + 1),
            Direction::West => (coords.0 - 1, coords.1),
            Direction::East => (coords.0 + 1, coords.1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Case {
    NoChar,
    BlockingChar,
    NonBlockingChar,
}
fn main() {
    let input = fs::read_to_string("input").unwrap();
    let answer = do_aoc(&input);
    println!("{answer}");
}

fn do_aoc(input: &str) -> u32 {
    let maze: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let start = maze
        .iter()
        .enumerate()
        .map(|(y, line)| (y, line, memchr(b'S', line)))
        .find(|(_, _, x)| x.is_some())
        .unwrap();

    let start_y = start.0;
    let start_x = start.2.unwrap();

    let start_dir: Direction;
    let mut found_first = false;
    let mut s_should_count = false;
    'direction_search: {
        if start_x != 0 && open_to_east(maze[start_y][start_x - 1]) {
            found_first = true;
        }

        if open_to_west(maze[start_y][start_x + 1]) {
            if found_first {
                start_dir = Direction::East;
                break 'direction_search;
            } else {
                found_first = true;
            }
        }

        if open_to_north(maze[start_y + 1][start_x]) && found_first {
            start_dir = Direction::South;
            break 'direction_search;
        }

        if start_y != 0 && open_to_south(maze[start_y - 1][start_x]) {
            s_should_count = true;
            start_dir = Direction::North;
            break 'direction_search;
        }

        panic!("Start didn't have two directions to go?")
    }

    let start_pos_1 = start_dir.bump((start_x, start_y));

    let mut animal = Animal {
        x: start_pos_1.0,
        y: start_pos_1.1,
        dir: start_dir,
    };

    let rows = maze.len();
    let cols = maze[0].len();

    let mut has_char = Array::from_elem((cols, rows), Case::NoChar);

    if s_should_count {
        has_char[[start_x, start_y]] = Case::BlockingChar;
    } else {
        has_char[[start_x, start_y]] = Case::NonBlockingChar;
    }

    loop {
        let cur = maze[animal.y][animal.x];
        has_char[[animal.x, animal.y]] = if cur == b'|' || cur == b'J' || cur == b'L' {
            Case::BlockingChar
        } else {
            Case::NonBlockingChar
        };
        animal.do_move(&maze);

        if animal.x == start_x && animal.y == start_y {
            break;
        }
    }

    // Treat this as a point-in-polygon problem, where the top part of each
    // character cell is where the point is. If we cross an odd number of
    // vertical lines on our way through the cell, then we are inside of the
    // shape, otherwise we are outside of it.

    // See https://en.wikipedia.org/wiki/Point_in_polygon

    let mut inside: u32 = 0;
    for y in 0..rows {
        let mut hits: u32 = 0;
        for x in 0..cols {
            match has_char[[x, y]] {
                Case::NoChar => {
                    if hits % 2 == 1 {
                        inside += 1;
                    }
                }
                Case::BlockingChar => hits += 1,
                Case::NonBlockingChar => {}
            }
        }
    }
    inside
}

fn open_to_east(c: u8) -> bool {
    return c == b'-' || c == b'L' || c == b'F';
}

fn open_to_west(c: u8) -> bool {
    return c == b'-' || c == b'J' || c == b'7';
}

fn open_to_north(c: u8) -> bool {
    return c == b'|' || c == b'J' || c == b'L';
}

fn open_to_south(c: u8) -> bool {
    return c == b'|' || c == b'7' || c == b'F';
}

#[bench]
fn bench_day10(b: &mut test::Bencher) {
    let file = fs::read_to_string("input").unwrap();

    b.iter(|| test::black_box(do_aoc(&file)));
}
