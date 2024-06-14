use std::fs;

 fn main() {
    let input = fs::read_to_string("day01.txt").expect("Something went wrong reading the file");
    let splitted = input.split("\n").collect::<Vec<&str>>();

    // Part 1
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;
    let mut sum = 0;
    for word in &splitted {
        for c in word.chars() {
            if c.is_digit(10) {
                if first.is_none() {
                    first = Some(c);
                }
                last = Some(c);
            }
        }
        match (first, last) {
            (Some(f), Some(l)) => sum += format!("{}{}", f, l).parse::<i32>().unwrap(),
            _ => ()
        }
        first = None;
    }
    //println!("Part 1: {}", sum);

    // Part 2
    let numbers = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let rev_numbers = numbers.map(|n| n.chars().rev().collect::<String>());
    let mut rev_str_numbers = ["";10];
    (0..10).for_each(|i| rev_str_numbers[i] = rev_numbers[i].as_str() );

    let mut sum = 0;
    for word in &splitted {
        let first_str_num = find_first_in(word, numbers);
        let last_num = find_first_in(word.chars().rev().collect::<String>().as_str(), rev_str_numbers);
        let conc = format!("{}{}", first_str_num, last_num);
        sum += conc.parse::<i32>().unwrap();
        // println!("{}", conc);
    }
    // println!("Part 2: {}", sum);
    println!("{}", sum);
}

fn find_first_in(word: &str, numbers: [&str; 10]) -> u32 {
    let mut num: Option<u32> = None;
    let mut first_char_num: Option<u32> = None;
    let mut i = 0;
    for c in word.chars() {
        if c.is_digit(10) {
            first_char_num = Some(i);
            break;
        }
        i += 1;
    }

    let mut first_str_num: Option<u32>;
    let mut best_first_str_num: Option<u32> = None;
    i = 0;
    for n in numbers {
        first_str_num = match word.find(n) {
            Some(v) => Some(v as u32),
            _ => None
        };
        if best_first_str_num.is_none() || first_str_num.is_some() && first_str_num < best_first_str_num {
            num = Some(i);
            best_first_str_num = first_str_num;
        }
        i += 1;
    }
    if best_first_str_num.is_none() || first_char_num.is_some() && first_char_num < best_first_str_num {
        num = Some(word.chars().nth(first_char_num.unwrap() as usize).unwrap().to_digit(10).unwrap());
    }
    match num {
        Some(n) => n,
        None => 0
    }
}
