use std::fs;

const DEBUG: bool = false;

fn win(card: String) -> u32 {
    let mut winningNumbers = Vec::new();
    let mut ownNumbers = Vec::new();

    let nums = card.split(" ");
    let mut leftSide = true;
    for num in nums {
        if num == "" { continue; }
        if num == "|" {
            leftSide = false;
        }
        match num.parse::<i32>() {
            Ok(number) => {
                if leftSide { winningNumbers.push(number); }
                else { ownNumbers.push(number); }
            },
            Err(_) => {}
        }
    }

    let mut count = 0;
    for ownNum in ownNumbers {
        //print!("{} ", ownNum);
        if winningNumbers.contains(&ownNum) {
            count += 1;
        }
    }

    return count;
}

fn main()
{
    let input = fs::read_to_string("day04_t.in").expect("FILE ERROR");
    let splittedInput = input.split("\n");
    let mut indexableLines = Vec::new();
    for line in splittedInput {
        indexableLines.push(line);
    }

    // part 1
    let mut sum = 0;
    for line in &indexableLines {
        let count = win(line.to_string());
        let mut points = 0;
        if count > 0 {
            points = 1;
            for i in 1..count {
                points *= 2;
            }
        }
        sum += points;
    }

    // part 2
    let mut remaining = Vec::new();
    let mut index: usize = 1;
    let mut instances = 0;
    for i in 1..indexableLines.len()+1 {
        remaining.push(i);
    }
    while !remaining.is_empty() {
        index = remaining.pop().unwrap() as usize;
        let mut line = indexableLines[index-1];
        let mut count = win(line.to_string()) as usize;
        instances += 1;
        if count > 0 {
            for i in index+1..(count+index+1 as usize) {
                remaining.push(i);
            }
        }
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", instances);
}