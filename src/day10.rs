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

fn mooooove(c: char, lastPos: Pos, pos: Pos) -> Pos {
    let dir = pos - lastPos;
    match c {
        '|' => if dir == UP { pos + UP } else { pos + DOWN },
        '-' => if dir == RIGHT { pos + RIGHT } else { pos + LEFT },
        'L' => if dir == LEFT { pos + UP } else { pos + RIGHT },
        'J' => if dir == RIGHT { pos + UP } else { pos + LEFT },
        'F' => if dir == LEFT { pos + DOWN } else { pos + RIGHT },
        '7' => if dir == RIGHT { pos + DOWN } else { pos + LEFT },
        _ => pos
    }
}

fn main() {
    let input = fs::read_to_string("day10.in").expect("Something went wrong reading the file");
    let mut lines = input.split("\n").collect::<Vec<&str>>().iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    
    let mut pos: Pos = Pos { x: 0, y: -1 };
    let mut found = false;
    for line in &lines {
        pos.x = 0;
        for &c in line {
            if c == 'S' {
                found = true;
                break;
            }
            pos.x += 1;
        }
        pos.y += 1;
        if found { break; }
    }

    let mut lastPos: Pos = pos;
    let mut oPos: Pos = pos;
    let mut oLastPos: Pos = oPos;

    lines[pos.y as usize][pos.x as usize] = '%';
    lines[oPos.y as usize][oPos.x as usize] = '%';

    oPos = oPos + RIGHT;
    pos = pos + DOWN;

    let mut count = 0;
    
    while lines[pos.y as usize][pos.x as usize] != '%' {
        let lC = lines[pos.y as usize][pos.x as usize];
        let oLC = lines[oPos.y as usize][oPos.x as usize];
        let lP = pos;
        let oLP = oPos;

        lines[pos.y as usize][pos.x as usize] = '%';
        lines[oPos.y as usize][oPos.x as usize] = '%';

        pos = mooooove(lC, lastPos, pos);
        oPos = mooooove(oLC, oLastPos, oPos);

        lastPos = lP;
        oLastPos = oLP;

        count += 1;
    }

    println!("Part 1: {}", count);

    /*
    for line in &lines {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
    */

    let mut oldLines = input.split("\n").collect::<Vec<&str>>().iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    pos = Pos { x: 0, y: 0 };
    while (pos.y as usize) < lines.len() {
        pos.x = 0;
        while (pos.x as usize) < lines[pos.y as usize].len() {
            if lines[pos.y as usize][pos.x as usize] != '%' {
                oldLines[pos.y as usize][pos.x as usize] = '.';
            }
            pos.x += 1;
        }
        pos.y += 1;
    }
    lines = oldLines;

    let mut tileCount = 0;
    for line in &mut lines {
        let mut edgeCount = 0;
        let mut isLine = false;
        let mut up = false;
        for c in line.iter_mut() {
            if *c == '|' {
                edgeCount += 1;
            } else if *c == 'F' || *c == 'L' || *c == 'S' {
                edgeCount += 1;
                isLine = true;
                if *c == 'L' {
                    up = true;
                } else {
                    up = false;
                }
            } else if *c == '7' || *c == 'J' {
                if *c == '7' && up || *c == 'J' && !up {
                    
                } else {
                    edgeCount += 1;
                }
                isLine = false;
            }

            if !isLine && *c != '|' && *c != '7' && *c != 'J'{
                if edgeCount % 2 == 1 {
                    tileCount += 1;
                    *c = '1';
                } else {
                    *c = '0';
                }
            }
        }
    }

    println!("Part 2: {}", tileCount);

    /*
    for line in &lines {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
    */
}
