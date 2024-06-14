use std::fs;

use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos {
    x: i32,
    y: i32
}

const UP: Pos = Pos { x: 0, y: -1 };
const DOWN: Pos = Pos { x: 0, y: 1 };
const LEFT: Pos = Pos { x: -1, y: 0 };
const RIGHT: Pos = Pos { x: 1, y: 0 };

impl ops::Add for Pos {
    type Output = Self;

    fn add(self, other: Pos) -> Pos {
        Pos { x: self.x + other.x, y: self.y + other.y }
    }
}
impl ops::Sub for Pos {
    type Output = Self;

    fn sub(self, other: Pos) -> Pos {
        Pos { x: self.x - other.x, y: self.y - other.y }
    }
}

fn get(lines: &mut Vec<Vec<char>>, pos: Pos) -> &mut char {
    println!("{} {}", pos.x, pos.y);
    &mut lines[pos.y as usize][pos.x as usize]
}
fn make_pos(x: usize, y: usize) -> Pos {
    Pos { x: x as i32, y: y as i32 }
}

fn main() {
    let input = fs::read_to_string("day11_t.in").expect("Something went wrong reading the file");
    let mut lines = input
                        .split("\n").collect::<Vec<&str>>()
                        .iter().map(|s|
                                    s.chars().collect::<Vec<char>>()
                                    )
                        .collect::<Vec<Vec<char>>>();
    let SIZE = make_pos ( lines[0].len(), lines.len() );

    let mut pos = Pos { x: 0, y: 0 };
    while pos.x < SIZE.x {
        pos.y = 0;
        let mut empty = true;
        while pos.y < SIZE.y {
            if *get(&mut lines, pos) == '#' {
                empty = false;
                break;
            }
            pos.y += 1;
        }
        if empty {
            pos.y = 0;
            while pos.y < SIZE.y {
                *get(&mut lines, pos) = '%';
                pos.y += 1;
            }
        }
        pos.x += 1;
    }

    pos.y = 0;
    while pos.y < SIZE.y {
        pos.x = 0;
        let mut empty = true;
        while pos.x < SIZE.x {
            if *get(&mut lines, pos) == '#' {
                empty = false;
                break;
            }
            pos.x += 1;
        }
        if empty {
            pos.x = 0;
            while pos.x < SIZE.x {
                *get(&mut lines, pos) = '%';
                pos.x += 1;
            }
        }
        pos.y += 1;
    }

    pos.y = 0;
    while pos.y < SIZE.y {
        pos.x = 0;
        while pos.x < SIZE.x {
            print!("{}", *get(&mut lines, pos));
            pos.x += 1;
        }
        pos.y += 1;
    }
}
