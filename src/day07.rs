use std::fs;
use std::convert::TryInto;
use std::collections::HashMap;

fn count_of(cards: [char; 5]) -> Vec<u32> {
      let mut known = vec!(cards[0]);
      let mut count = vec!(0);
      for b in cards {
            let index = known.iter().position(|&r| r == b);
            match index {
                  Some(i) => count[i] += 1,
                  None => {
                      known.push(b);
                      count.push(1);
                  }
            }
      }
      count
}
fn some_oak(cards: [char; 5], c: u32) -> bool {
    let count = count_of(cards);
      count.iter().any(|&r| r == c)
}
fn fioak(cards: [char; 5]) -> bool {
    some_oak(cards, 5)
}
fn fooak(cards: [char; 5]) -> bool {
    some_oak(cards, 4)
}
fn thoak(cards: [char; 5]) -> bool {
    let c = count_of(cards);
    c.iter().any(|&r| r == 3) && c.iter().filter(|&r| r == &1).count() == 2
}
fn fh(cards: [char; 5]) -> bool {
    let c = count_of(cards);
    c.iter().filter(|&r| r == &3).count() == 1 && c.iter().filter(|&r| r == &2).count() == 1
}
fn tp(cards: [char; 5]) -> bool {
    let count = count_of(cards);
    count.iter().filter(|&r| r >= &2).count() == 2
}
fn op(cards: [char; 5]) -> bool {
    let count = count_of(cards);
      count.iter().filter(|&r| r >= &2).count() == 1
}
fn hc(cards: [char; 5]) -> bool {
    let count = count_of(cards);
      count.iter().filter(|&r| r == &1).count() == 5
}
fn power_of(cards: [char; 5]) -> u32 {
    if fioak(cards) { 7000 }
    else if fooak(cards) { 6000 }
    else if fh(cards) { 5000 }
    else if thoak(cards) { 4000 }
    else if tp(cards) { 3000 }
    else if op(cards) { 2000 }
    else { 1000 }
}
fn joker_power_of(mut cards: [char; 5]) -> u32 {
    if !cards.iter().any(|c| c == &'.') { return power_of(cards); }
    let mut highest = 0;
    let allChars = "0123456789BDEF".chars();
    let i = cards.iter().position(|r| r == &'.').unwrap();
    let old = cards;
    let mut s: String = cards.iter().collect();
    println!("checking {} with:", s);
    for c in allChars.clone() {
        cards[i] = c;
        while cards.iter().filter(|&r| r == &'.').count() > 0 {
            let j = cards.iter().position(|&r| r == '.').unwrap();
            cards[j] = c;
        }
        println!("{}", s);
        s = cards.iter().collect();
        let pow = power_of(cards);
        if pow > highest { highest = pow; }
        println!("{}: {}", c, pow);
        cards = old;
    }
    return highest;
}
fn greater(a: [char; 5], b: [char; 5]) -> bool {
    for i in 0..5 {
        if a[i] == b[i] {
            continue;
        }
        else if a[i] > b[i] {
            return true;
        }
        else if a[i] < b[i] {
            return false;
        }
    }
    return true;
}

fn main() {
    let input = fs::read_to_string("day07.in").expect("FILE ERROR");
    let splittedInput = input.split("\n");

    let mut charMap = HashMap::new();
    charMap.insert('A', 'F');
    charMap.insert('K', 'E');
    charMap.insert('Q', 'D');
    charMap.insert('J', '.');
    charMap.insert('T', 'B');

    let mut hands: Vec<[char; 5]> = Vec::new();
    let mut stringHands: Vec<String> = Vec::new();
    let mut cardPowers: Vec<u32> = Vec::new();
    let mut cardBids: Vec<u32> = Vec::new();

    for line in splittedInput {
        let mut strLine = line.to_string();
        strLine.pop();
        if strLine.len() == 0 { continue; }
        let splitLine: Vec<&str> = strLine.split(" ").collect();
        let mut cards: Vec<char> = splitLine[0].chars().collect();
        for i in 0..cards.len() {
            cards[i] = *charMap.get(&cards[i]).unwrap_or(&cards[i]);
        }

        hands.push(cards.clone().try_into().unwrap());
        stringHands.push(splitLine[0].to_string());
        let bid = splitLine[1].parse::<u32>().unwrap();
        cardBids.push(bid);

        let power = joker_power_of(cards.try_into().unwrap());
        cardPowers.push(power);
        // println!("{}: {}", splitLine[0], power);
    }

    let mut finalPowers: Vec<u32> = Vec::new();
    for i in 0..cardPowers.len() {
        let mut pow = 1;
        for j in 0..cardPowers.len() {
            if i == j {
                continue;
            }
            if cardPowers[i] > cardPowers[j] {
                pow += 1;
                // println!("{}({}) is a stronger hand than {}({})", stringHands[i], cardPowers[i], stringHands[j], cardPowers[j]);
            }
            else if cardPowers[i] == cardPowers[j] {
                if greater(hands[i], hands[j]) {
                    pow += 1;
                    // println!("{} has a stronger card than {}", stringHands[i], stringHands[j]);
                }
            }
        }
        finalPowers.push(pow);
    }
    let mut sum = 0;
    for i in 0..finalPowers.len() {
        sum += finalPowers[i] * cardBids[i];
    }
    let mut indices: Vec<usize> = (0..finalPowers.len()).collect();
    indices.sort_by(|a, b| finalPowers[*a].cmp(&finalPowers[*b]));
    for i in indices {
        println!("{}: {} | {} | {}", finalPowers[i], cardBids[i], stringHands[i], cardPowers[i]);
    }
    println!("Part 2: {}", sum);
}
