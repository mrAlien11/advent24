use std::{collections::HashMap, fs, vec};

const FILE: &str = "input.txt";
const TEST: &str = "test.txt";
const TEST2: &str = "test2.txt";

fn main() {
    let reader = fs::read_to_string(FILE).expect("Could not read file");
    let table = reader.lines().into_iter().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut nodes = HashMap::new();
    let mut antinodes: Vec<(i32, i32)> = vec![];

    for i in 0..table.len() {
        for j in 0..table[i].len() {
            if table[i][j] != '.' {
                nodes.entry(table[i][j])
                    .or_insert_with(Vec::new)
                    .push((i, j));   
            }
        }
    }

    for key in nodes.keys() {
        for i in 0..nodes.get(key).unwrap().len() {
            let tester = nodes.get(key).unwrap()[i];
            for j in 0..nodes.get(key).unwrap().len() {
                if nodes.get(key).unwrap().len() > 1 && !antinodes.contains(&(tester.0 as i32, tester.1 as i32)) {
                    antinodes.push((tester.0 as i32, tester.1 as i32));
                }
                if j == i {
                    continue;
                }
                let test = nodes.get(key).unwrap()[j];
                let mut pos = (0, 0);
                let distance_y = tester.0 as i32 - test.0 as i32;
                let distance_x = tester.1 as i32 - test.1 as i32;
                pos.0 = tester.0 as i32 + distance_y;
                pos.1 = tester.1 as i32 + distance_x;
                while pos.0 >= 0 && pos.0 <= (table.len() - 1) as i32 && pos.1 >= 0 && pos.1 <= (table[0].len() - 1) as i32 {
                    if !antinodes.contains(&pos) {
                        antinodes.push(pos);
                    }
                    pos.0 += distance_y;
                    pos.1 += distance_x;
                }
            }
        }
    }

    let ans = antinodes.len();
    // for antinode in antinodes {
    //     table[antinode.0 as usize][antinode.1 as usize] = '#';
    // }
    // println!("{:?}", nodes);
    // table.iter().for_each(|x| {
    //     print!("\n");
    //     x.iter().for_each(|y| print!("{}", y));
    // });
    println!("{}", ans);
}

fn part_1() {
    let reader = fs::read_to_string(FILE).expect("Could not read file");
    let table = reader.lines().into_iter().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut nodes = HashMap::new();
    let mut antinodes: Vec<(i32, i32)> = vec![];

    for i in 0..table.len() {
        for j in 0..table[i].len() {
            if table[i][j] != '.' {
                nodes.entry(table[i][j])
                    .or_insert_with(Vec::new)
                    .push((i, j));   
            }
        }
    }

    for key in nodes.keys() {
        for i in 0..nodes.get(key).unwrap().len() {
            let tester = nodes.get(key).unwrap()[i];
            for j in 0..nodes.get(key).unwrap().len() {
                if j == i {
                    continue;
                }
                let test = nodes.get(key).unwrap()[j];
                let mut pos = (0, 0);
                let distance_y = tester.0 as i32 - test.0 as i32;
                let distance_x = tester.1 as i32 - test.1 as i32;
                pos.0 = tester.0 as i32 + distance_y;
                pos.1 = tester.1 as i32 + distance_x;
                if pos.0 >= 0 && pos.0 <= (table.len() - 1) as i32 && pos.1 >= 0 && pos.1 <= (table[0].len() - 1) as i32 && !antinodes.contains(&pos) {
                    antinodes.push(pos);
                }
            }
        }
    }

    let ans = antinodes.len();
}