use std::fs;

const FILE: &str = "input.txt";
const TEST: &str = "test.txt";

fn main() {
    let reader = fs::read_to_string(FILE).expect("Could not read file");
    let table = reader.lines().collect::<Vec<_>>();
    let mut sum = 0;

    for i in 0..table.len() {
        // println!("{}", table[i]);
        for j in 0..table[i].chars().count() {
            // println!("{:?}", table[i].chars().nth(j));
            if table[i].chars().nth(j).unwrap() == 'M'{
                // down
                if i <= table.len() - 3 && j <= table[i].chars().count() - 3 && table[i+2].chars().nth(j).unwrap() == 'M' && table[i].chars().nth(j+2).unwrap() == 'S' && table[i+2].chars().nth(j+2).unwrap() == 'S' && table[i+1].chars().nth(j+1).unwrap() == 'A' {
                    sum += 1;
                }
                // right
                if i <= table.len() - 3 && j <= table[i].chars().count() - 3 && table[i].chars().nth(j+2).unwrap() == 'M' && table[i+2].chars().nth(j).unwrap() == 'S' && table[i+2].chars().nth(j+2).unwrap() == 'S' && table[i+1].chars().nth(j+1).unwrap() == 'A' {
                    sum += 1;
                }
                // right up
                if i >= 2 && j <= table[i].chars().count() - 3 && table[i].chars().nth(j+2).unwrap() == 'M' && table[i-2].chars().nth(j).unwrap() == 'S' && table[i-2].chars().nth(j+2).unwrap() == 'S' && table[i-1].chars().nth(j+1).unwrap() == 'A' {
                    sum += 1;
                }
                // down left
                if i <= table.len() - 3 && j >= 2 && table[i+2].chars().nth(j).unwrap() == 'M' && table[i].chars().nth(j-2).unwrap() == 'S' && table[i+2].chars().nth(j-2).unwrap() == 'S' && table[i+1].chars().nth(j-1).unwrap() == 'A' {
                    sum += 1;
                }
            }
        }
    }
    println!("{}", sum);
    // println!('{:?}', table[2]);
}

fn part_1() {
    let reader = fs::read_to_string(FILE).expect("Could not read file");
    let table = reader.lines().collect::<Vec<_>>();
    let mut sum = 0;

    for i in 0..table.len() {
        for j in 0..table[i].chars().count() {
            // println!("{}", table[i]);
            if table[i].chars().nth(j).unwrap() == 'X' {
                // up
                if i >= 3 {
                    // println!("up");
                    if table[i-1].chars().nth(j).unwrap() == 'M' && table[i-2].chars().nth(j).unwrap() == 'A' && table[i-3].chars().nth(j).unwrap() == 'S' {
                        sum += 1;
                    }
                }
                // up left
                if i >= 3 && j >= 3 {
                    // println!("up left");
                    if table[i-1].chars().nth(j-1).unwrap() == 'M' && table[i-2].chars().nth(j-2).unwrap() == 'A' && table[i-3].chars().nth(j-3).unwrap() == 'S' {
                        sum += 1;
                    }
                }
                // up right
                if i >= 3 && j <= table[i].chars().count() - 4 {
                    // println!("up right");
                    if table[i-1].chars().nth(j+1).unwrap() == 'M' && table[i-2].chars().nth(j+2).unwrap() == 'A' && table[i-3].chars().nth(j+3).unwrap() == 'S' {
                        sum += 1;
                    }
                }
                // right
                if j <= table[i].chars().count() - 4 {
                    // println!("right");
                    if table[i].chars().nth(j+1).unwrap() == 'M' && table[i].chars().nth(j+2).unwrap() == 'A' && table[i].chars().nth(j+3).unwrap() == 'S' {
                        sum += 1;
                    }
                }
                // left
                if j >= 3 {
                    // println!("left");
                    if table[i].chars().nth(j-1).unwrap() == 'M' && table[i].chars().nth(j-2).unwrap() == 'A' && table[i].chars().nth(j-3).unwrap() == 'S' {
                        sum += 1;
                    }
                }
                // down left
                if i <= table.len() - 4 && j >= 3 {
                    // println!("down left");
                    if table[i+1].chars().nth(j-1).unwrap() == 'M' && table[i+2].chars().nth(j-2).unwrap() == 'A' && table[i+3].chars().nth(j-3).unwrap() == 'S' {
                        sum += 1;
                    }
                }
                // down right
                if i <= table.len() - 4 && j <= table[i].chars().count() - 4 {
                    // println!("down right");
                    if table[i+1].chars().nth(j+1).unwrap() == 'M' && table[i+2].chars().nth(j+2).unwrap() == 'A' && table[i+3].chars().nth(j+3).unwrap() == 'S' {
                        sum += 1;
                    }
                }
                // down
                if i <= table.len() - 4 {
                    // println!("down");
                    if table[i+1].chars().nth(j).unwrap() == 'M' && table[i+2].chars().nth(j).unwrap() == 'A' && table[i+3].chars().nth(j).unwrap() == 'S' {
                        sum += 1;
                    }
                }
            }
        }
    }

    println!("{}", sum);
    // println!('{:?}', table[2]);
}