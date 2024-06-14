
use std::fs;
use std::thread;

const DEBUG: bool = false;

fn main()
{
      let input = fs::read_to_string("day05.in").expect("FILE ERROR");
      let splittedInput = input.split("\n");

      let seeds: &[usize] = &[858905075,56936593,947763189,267019426,206349064,252409474,660226451,92561087,752930744,24162055,75704321,63600948,3866217991,323477533,3356941271,54368890,1755537789,475537300,1327269841,427659734];
      // let mut seeds: &mut [usize] = &mut [79,14,55,13];
      let mut maps: Vec<Vec<[usize; 3]>> = Vec::new();
      let mut mapIndex = 0;
      let mut lineIndex = 0;

      for line in splittedInput {
            if line == "" {
                  continue;
            }
            if !line.chars().next().unwrap().is_digit(10) {
                  mapIndex += 1;
                  lineIndex = 0;
                  maps.push(Vec::new());
                  // maps[mapIndex-1].push(Vec::new());
            } else {
                  lineIndex += 1;
                  maps[mapIndex-1].push([0; 3]);
                  let mut lineStr = line.to_string();
                  lineStr.pop();
                  let numbers = lineStr.as_str().split(" ");
                  let mut i = 0;
                  for number in numbers {
                  maps[mapIndex-1][lineIndex-1][i] = number.parse::<usize>().unwrap();
                  i += 1;
                  }
            }
      }

      let mut i = 0;
      let mut threads = Vec::new();
      while i < seeds.len() {
            let ownMaps = maps.clone();
            threads.push(thread::spawn(move || {
                  let mut realSeeds = Vec::new();
                  let threadId = i;
                  for s in seeds[i]..seeds[i]+seeds[i+1] {
                        // if s % 10_000_000 == 0 { println!("{}: {}/{}", threadId, s, seeds[i]+seeds[i+1]); }
                        realSeeds.push(s);
                  }
                  println!("Thread {}: {} seeds", threadId, realSeeds.len());

    //     let mut i = 0;
    //     while i < seeds.len() {
    //         for map in &maps {
    //             for conv in map {
    //                 if seeds[i] >= conv[1] && seeds[i] < conv[1] + conv[2] {
    //                     seeds[i] = seeds[i] + conv[0] - conv[1];
    //                     break;
    //                 }
    //             }
    //             print!("{} => ", seeds[i]);
    //         }
    //         println!();
    //         i += 1;
    //     }
                  let mut i = 0;
                  while i < realSeeds.len() {
                        let seed = realSeeds[i];
                        if i % 1_000_000 == 0 { println!("Thread {}: {}%", threadId, i as f64 / realSeeds.len() as f64 * 100.0f64); }
                        for map in &ownMaps {
                              for conv in map {
                                    if seed >= conv[1] && seed < conv[1] + conv[2] {
                                          realSeeds[i] = seed + conv[0] - conv[1];
                                          break;
                                    }
                              }
                        }
                        i += 1;
                  }
                  println!("Thread {}: sorting", threadId);
                  realSeeds.sort();
                  println!("Thread {}: lowest seed: {}", threadId, realSeeds[0]);
            }));
            i += 2;
      }

      for thread in threads {
            thread.join().unwrap();
      }
}