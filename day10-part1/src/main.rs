#![feature(test)]
use std::fs;
use memchr::memchr;
extern crate test;

#[derive(Debug)]
struct Animal {
    x: usize,
    y: usize,
    dir: Direction,
    // last_steps: Vec<(usize, usize, char, Direction)>
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
            },
            b'-' => {
                if self.dir == Direction::West {
                    self.x -= 1;
                } else {
                    self.x += 1;
                }
            },
            b'L' => {
                if self.dir == Direction::West {
                    self.y -= 1;
                    self.dir = Direction::North;
                } else {
                    self.x += 1;
                    self.dir = Direction::East;
                }
            },
            b'J' => {
                if self.dir == Direction::East {
                    self.y -= 1;
                    self.dir = Direction::North;
                } else {
                    self.x -= 1;
                    self.dir = Direction::West;
                }
            },
            b'7' => {
                if self.dir == Direction::East {
                    self.y += 1;
                    self.dir = Direction::South;
                } else {
                    self.x -= 1;
                    self.dir = Direction::West
                }
            },
            b'F' => {
                if self.dir == Direction::West {
                    self.y += 1;
                    self.dir = Direction::South
                } else {
                    self.x += 1;
                    self.dir = Direction::East;
                }
            }
            _ => panic!("Nonsensical pipe {}", cur_pipe)
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
            Direction::North => (coords.0, coords.1-1),
            Direction::South => (coords.0, coords.1+1),
            Direction::West => (coords.0-1, coords.1),
            Direction::East => (coords.0+1, coords.1),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let answer = do_aoc(&input);
    println!("{answer}");
}

fn do_aoc(input: &str) -> u32 {
    let maze: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let start = maze.iter().enumerate().map(|(y, line)| {
        (y, line, memchr(b'S', line))
    }).find(|(_, _, x)| x.is_some()).unwrap();

    let start_y = start.0;
    let start_x = start.2.unwrap();
    
    let mut start_dir_1 = None;
    let start_dir_2: Direction;

    'direction_search: {
        if start_x != 0 && open_to_east(maze[start_y][start_x-1]) {
            start_dir_1 = Some(Direction::West)
        } 
        
        if open_to_west(maze[start_y][start_x+1]) {
            if let Some(_) = start_dir_1 {
                start_dir_2 = Direction::East;
                break 'direction_search;
            } else {
                start_dir_1 = Some(Direction::East)
            }
        }

        if open_to_north(maze[start_y+1][start_x]) {
            if let Some(_) = start_dir_1 {
                start_dir_2 = Direction::South;
                break 'direction_search;
            } else {
                start_dir_1 = Some(Direction::South)
            }
        }

        if start_y != 0 && open_to_south(maze[start_y-1][start_x]) {
            start_dir_2 = Direction::North;
            break 'direction_search;
        }

        panic!("Start didn't have two directions to go?")
    }

    let start_dir_1 = start_dir_1.unwrap();

    let start_pos_1 = start_dir_1.bump((start_x, start_y));   
    let start_pos_2 = start_dir_2.bump((start_x, start_y));   

    let mut animal_1 = Animal {x: start_pos_1.0, y: start_pos_1.1, dir: start_dir_1};
    let mut animal_2 = Animal {x: start_pos_2.0, y: start_pos_2.1, dir: start_dir_2};

    let mut steps = 1;

    loop {
        animal_1.do_move(&maze);
        animal_2.do_move(&maze);
        steps += 1;

        if animal_1.x == animal_2.x && animal_1.y == animal_2.y {
            break;
        }
    }

    steps
}

fn open_to_east(c: u8) -> bool {
    return c==b'-' || c == b'L' || c == b'F'
}

fn open_to_west(c: u8) -> bool {
    return c==b'-' || c == b'J' || c == b'7'
}

fn open_to_north(c:u8) -> bool {
    return c==b'|' || c == b'J' || c == b'L'
}

fn open_to_south(c:u8) -> bool {
    return c==b'|' || c == b'7' || c == b'F'
}

#[bench]
fn bench_day10(b: &mut test::Bencher) {
    let file = fs::read_to_string("input").unwrap();

    b.iter(|| test::black_box(do_aoc(&file)));
}
