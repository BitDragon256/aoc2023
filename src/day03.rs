use std::fs;

const SIZE: usize = 141;
const DEBUG: bool = false;

fn main()
{
    let input = fs::read_to_string("day03.in").expect("FILE ERROR");
    let splittedInput = input.split("\n");

    let mut numberValue: Vec<u32> = Vec::new();
    let mut numberPosition: Vec<usize> = Vec::new();
    let mut numberLen: Vec<usize> = Vec::new();
    let mut isPart = [[false; SIZE]; SIZE];

    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut curNum = 0;
    for line in splittedInput
    {
        x = 0;
        for ch in line.chars()
        {
            if ch.is_digit(10) {
                if curNum > 0 {
                    let l = numberLen.len();
                    numberLen[l - 1] += 1;

                    curNum *= 10;
                } else {
                    numberLen.push(1);
                    numberPosition.push(x + y * SIZE)
                }

                curNum += ch.to_digit(10).unwrap();
            } else if ch != '.' && x != SIZE {
                isPart[x][y] = true;
            }
            if !ch.is_digit(10) {
                if curNum > 0 {
                    numberValue.push(curNum);
                    curNum = 0;
                }
            }
            x += 1;
        }
        if curNum > 0 {
            numberValue.push(curNum);
            curNum = 0;
        }
        y += 1;
    }

    // part 1
    let mut partSum: u32 = 0;
    {
        let mut i = 0;
        while i < numberValue.len()
        {
            let val: u32 = numberValue[i];
            let len: usize = numberLen[i];
            let pos: usize = numberPosition[i];

            if DEBUG {
                let x: usize = pos % SIZE;
                let y: usize = pos / SIZE;
                println!("Num {} | {} | {} | {}", val, is_part_num(isPart, len, pos), x, y+1);
            }

            if is_part_num(isPart, len, pos)
            {
                partSum += val;
            }

            i += 1;
        }
    }

    // part 2
    let mut gearSum: u32 = 0;
    {
        let mut partNum = [[0; SIZE]; SIZE];
        let mut partVals = [[[None; 3]; SIZE]; SIZE];

        let mut i = 0;
        while i < numberValue.len()
        {
            let val: u32 = numberValue[i];
            let len: usize = numberLen[i];
            let pos: usize = numberPosition[i];

            let part = get_part(isPart, len, pos);
            match part {
                Some(partPos) => {
                    let x: usize = partPos % SIZE;
                    let y: usize = partPos / SIZE;
                    partNum[x][y] += 1;
                    let mut j = 0;
                    while partVals[x][y][j].is_some() {
                        j += 1;
                    }
                    partVals[x][y][j] = Some(val);
                },
                None => {}
            }

            i += 1;
        }

        let mut y = 0;
        while y < SIZE {
            let mut x = 0;
            while x < SIZE {
                if partNum[x][y] == 2 {
                    gearSum += partVals[x][y][0].unwrap() * partVals[x][y][1].unwrap();
                }

                x += 1;
            }
            y += 1;
        }
    }
    
    println!("Part 1: {}", partSum);
    println!("Part 2: {}", gearSum);
}

fn is_part_num(isPart: [[bool; SIZE]; SIZE], len: usize, pos: usize) -> bool
{
    let x: usize = pos % SIZE;
    let y: usize = pos / SIZE;
    if x > 0 {
        if isPart[x-1][y] { return true; }
    }
    if x < SIZE - len {
        if isPart[x+len][y] { return true; }
    }
    if y > 0 {
        let mut i = 0;
        while i < len {
            if isPart[x+i][y-1] { return true; }
            i += 1;
        }
        if x > 0 {
            if isPart[x-1][y-1] { return true; }
        }
        if x < SIZE - len {
            if isPart[x+len][y-1] { return true; }
        }
    }
    if y < SIZE - 1 {
        let mut i = 0;
        while i < len {
            if isPart[x+i][y+1] { return true; }
            i += 1;
        }
        if x > 0 {
            if isPart[x-1][y+1] { return true; }
        }
        if x < SIZE - len {
            if isPart[x+len][y+1] { return true; }
        }
    }
    false
}

fn to_index(x: usize, y: usize) -> usize 
{
    return x + y * SIZE;
}
fn get_part(isPart: [[bool; SIZE]; SIZE], len: usize, pos: usize) -> Option<usize>
{
    let x: usize = pos % SIZE;
    let y: usize = pos / SIZE;
    if x > 0 {
        if isPart[x-1][y] { return Some(to_index(x-1, y)); }
    }
    if x < SIZE - len {
        if isPart[x+len][y] { return Some(to_index(x+len, y)); }
    }
    if y > 0 {
        let mut i = 0;
        while i < len {
            if isPart[x+i][y-1] { return Some(to_index(x+i, y-1)); }
            i += 1;
        }
        if x > 0 {
            if isPart[x-1][y-1] { return Some(to_index(x-1, y-1)); }
        }
        if x < SIZE - len {
            if isPart[x+len][y-1] { return Some(to_index(x+len, y-1)); }
        }
    }
    if y < SIZE - 1 {
        let mut i = 0;
        while i < len {
            if isPart[x+i][y+1] { return Some(to_index(x+i, y+1)); }
            i += 1;
        }
        if x > 0 {
            if isPart[x-1][y+1] { return Some(to_index(x-1, y+1)); }
        }
        if x < SIZE - len {
            if isPart[x+len][y+1] { return Some(to_index(x+len, y+1)); }
        }
    }
    None
}