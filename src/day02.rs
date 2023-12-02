use std::fs;

const maxRed: i32 = 12;
const maxGreen: i32 = 13;
const maxBlue: i32 = 14;
fn check_game(red: i32, green: i32, blue: i32) -> bool {
    red <= maxRed && green <= maxGreen && blue <= maxBlue
}

const DEBUG: bool = false;

fn main() {
    let input = fs::read_to_string("day02.in").expect("File Error");
    let splitted = input.split("\n");

    // part 1
    let mut validGames = 0;
    // part 2
    let mut powerSum = 0;

    let mut gameId = 1;
    let mut index = 0;

    // part 1
    for gamein in splitted {
        let mut game = gamein.to_string();
        if index != 99 {
            game.pop();
        }
        index += 1;
        let sets = game.split(";");
        let mut valid = true;

        if DEBUG {
            print!("{}: ", gameId);
        }

        let mut minRed = 0;
        let mut minGreen = 0;
        let mut minBlue = 0;

        for inset in sets {
            let mut repl = inset.replace(";", "");
            repl = repl.replace(",", "");
            let set = &repl;
            let s = set.split(" ");
            
            let mut red: i32 = 0;
            let mut green: i32 = 0;
            let mut blue: i32 = 0;

            let mut lastNum: Option<i32> = None;
            let mut lastName: Option<&str> = None;
            for part in s {
                match part.parse::<i32>() {
                    Err(_) => lastName = Some(part),
                    Ok(v) => lastNum = Some(v)
                }

                // if gameId == 29 && lastNum.is_some() {
                //     println!("lastName: {}. {}, lastNum: {}", part, lastName.is_some(), lastNum.unwrap());
                // }

                if lastName.is_some() {
                    match lastName.unwrap() {
                        "red" => red = lastNum.unwrap(),
                        "green" => green = lastNum.unwrap(),
                        "blue" => blue = lastNum.unwrap(),
                        _ => {}
                    }
                    lastName = None;
                    lastNum = None;
                }
            }
            if DEBUG {
                print!("{} red {} green {} blue | ", red, green, blue);
            }
            if !check_game(red, green, blue) {
                valid = false;
            }

            if red > minRed { minRed = red; }
            if green > minGreen { minGreen = green; }
            if blue > minBlue { minBlue = blue; }
        }
        if DEBUG {
            println!("{} | {} red {} green {} blue", valid, minRed, minGreen, minBlue);
        }

        if valid {
            validGames += gameId;
        }
        gameId += 1;

        powerSum += minRed * minGreen * minBlue;
    }

    println!("Part 1: {}", validGames);
    println!("Part 2: {}", powerSum);
}