use std::fs;

 fn main() {
    println!("Hello Day 1!");
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
    println!("Part 1: {}", sum);

    // Part 2
    let mut sum = 0;
    for word in &splitted {
        let numbers = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        let num = find_first_in(word, numbers);
        let rev_numbers = numbers.map(|n| n.chars().rev().collect::<String>());
        let mut rev_str_numbers = ["";10];
        (0..10).for_each(|i| rev_str_numbers[i] = rev_numbers[i].as_str() );
        let num2 = find_first_in(word.chars().rev().collect::<String>().as_str(), rev_str_numbers);
        let conc = format!("{}{}", num, num2);
        sum += conc.parse::<i32>().unwrap();
    }
    println!("Part 2: {}", sum);
}

fn find_first_in(word: &str, numbers: [&str; 10]) -> usize {
    let mut num = 100;
    let mut first_real = 100;
    let mut i = 0;
    for c in word.chars() {
        if c.is_digit(10) {
            first_real = i;
            break;
        }
        i += 1;
    }

    let mut first_num;
    let mut last_first_num = 100;
    i = 0;
    for n in numbers {
        first_num = match word.find(n) {
            Some(v) => v,
            None => 100
        };
        if first_num != 100 && last_first_num > first_num {
            num = i;
            last_first_num = first_num;
        }
        i += 1;
    }
    if last_first_num > first_real {
        num = word.chars().nth(first_real).unwrap().to_digit(10).unwrap() as usize;
    }
    num
}