
use std::fs;

const DEBUG: bool = false;

fn main()
{
    let input = fs::read_to_string("day05.in").expect("FILE ERROR");
    let splittedInput = input.split("\n");

    let mut seeds: &mut [usize] = &mut [858905075,56936593,947763189,267019426,206349064,252409474,660226451,92561087,752930744,24162055,75704321,63600948,3866217991,323477533,3356941271,54368890,1755537789,475537300,1327269841,427659734];
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

    // part 1
    // {
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
    // }
    // seeds.sort();
    // for seed in seeds {
    //     println!("{}", seed);
    // }

    // for map in &maps {
    //     for conv in map {
    //         println!("{} {} {}", conv[0], conv[1], conv[2]);
    //     }
    // }

    // part 2
    let mut i = 0;
    let mut seedRanges: Vec<[usize; 2]> = Vec::new();
    while i < seeds.len() {
        seedRanges.push([seeds[i], seeds[i+1]]);
        i += 2;
    }

    let mut ranges: Vec<[usize; 2]> = Vec::new();
    i = 0;
    let len = seedRanges.len();
    while i < len {
        ranges.push(seedRanges[i]);
        i += 1;
    }

    let mut mapIndex = 0;
    // for map in &maps {
    while mapIndex < maps.len() {
        let map = &maps[mapIndex];
        mapIndex += 1;
        if map.len() == 0 {
            continue;
        }
        if DEBUG {
            println!("map({}) len: {}", mapIndex - 1, map.len());
        }
        //println!("next map({}) len: {}", mapIndex, maps[mapIndex].len());

        let mut rangeLen = ranges.len();

        // merge ranges
        let mut aId = 0;
        let mut newRanges = Vec::new();
        for range in ranges {
            if !newRanges.contains(&range) {
                newRanges.push(range);
            }
        }
        // while aId < rangeLen {
        //     let mut bId = 0;
        //     while bId <= aId {
        //         if ranges[aId][0] == ranges[bId][0] {
        //             ranges.push([ranges[aId][0], max(ranges[aId][1], ranges[bId][1])]);
        //         }
        //         bId += 1;
        //     }
        //     aId += 1;
        // }
        // ranges = ranges[rangeLen..ranges.len()].to_vec();
        ranges = newRanges;

        rangeLen = ranges.len();
        let mut j = 0;
        // print!("len: {} | ", rangeLen);
        while j < rangeLen {
            let mut convIndex = 0;
            let range = ranges[j];
            // for conv in map.iter() {
            let mut found = false;
            while convIndex < map.len() {
                //println!("next map({}) len: {}", mapIndex, maps[mapIndex].len());
                let conv = map[convIndex];
                convIndex += 1;
                let srcStart = conv[1];
                let srcLen = conv[2];
                let dstStart = conv[0];

                if DEBUG {
                    print!("src start: {}, src len: {}, dst start: {}, rangeStart: {}, rangeLen: {}", srcStart, srcLen, dstStart, range[0], range[1]);
                }

                // range is not in map
                if range[0] + range[1] <= srcStart || range[0] >= srcStart + srcLen {
                    if DEBUG {
                        println!();
                    }
                    continue;
                }
                if DEBUG {
                    println!(" | in");
                }
                // println!("src start: {}, src len: {}, dst start: {}, rangeStart: {}, rangeLen: {}", srcStart, srcLen, dstStart, range[0], range[1]);
                // range is completely in map
                if range[0] >= srcStart && range[0] + range[1] <= srcStart + srcLen {
                    ranges.push([range[0] - srcStart + dstStart, range[1]]);
                }
                // map is completely in range
                else if range[0] <= srcStart && range[0] + range[1] > srcStart + srcLen {
                    ranges.push([range[0], srcStart - range[0]]);
                    ranges.push([dstStart, srcLen]);
                    ranges.push([srcStart + srcLen, range[0] + range[1] - (srcStart + srcLen)]);
                }
                // range leaves map on the right
                else if range[0] >= srcStart {
                    let newInnerLen = srcStart + srcLen - range[0];
                    let newOuterLen = range[1] - newInnerLen; // range[0] + range[1] - (srcStart + srcLen)
                    ranges.push([range[0] - srcStart + dstStart, newInnerLen]);
                    ranges.push([srcStart + srcLen, newOuterLen]);
                }
                // range leaves map on the left
                else if range[0] + range[1] < srcStart + srcLen {
                    let newOuterLen = srcStart - range[0];
                    let newInnerLen = range[0] + range[1] - srcStart;
                    ranges.push([range[0], newOuterLen]);
                    ranges.push([dstStart, newInnerLen]);
                }
                else {
                    println!("why WHY");
                }
                found = true;
                break;
                // println!("len: {}", ranges.len());
            }
            if !found {
                ranges.push(range);
            }
            j += 1;
        }
        //if DEBUG {
            println!("{} {}", rangeLen, ranges.len());
        //}
        ranges = ranges[rangeLen..ranges.len()].to_vec();

    }

    // merge ranges
    let mut aId = 0;
    let mut newRanges = Vec::new();
    for range in &ranges {
        if !newRanges.contains(&range) {
            newRanges.push(range);
        }
    }

    i = 0;
    while i < ranges.len() {
        let mut j = 0;
        while j < i {
            if ranges[i][0] > ranges[j][0] {
                let mut temp = ranges[i];
                ranges[i] = ranges[j];
                ranges[j] = temp;
            }
            j += 1;
        }
        i += 1;
    }
    
    for range in ranges {
        println!("start: {}, len: {}", range[0], range[1]);
    }
}