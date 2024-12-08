use std::{fs, collections::HashMap};

const FILE: &str = "input.txt";
const TEST: &str = "test.txt";

fn main() {
    let reader = fs::read_to_string(FILE).expect("Could not read file");
    let mut table = reader.lines().into_iter().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut pos: (i32, i32) = (0, 0);
    let mut t = (0,0);
    // let mut current_dir: (i32, i32) = (-1, 0);
    // let mut spaces: Vec<(i32, i32)> = vec![];
    
    for y in 0..table.len() {
        for x in 0..table[y].len() {
            if table[y][x] == '^' {
                t = (y as i32, x as i32);
            }
        }
    }
    // spaces.push(pos);

    let mut sum = 0;

    for i in 0..table.len() {
        for j in 0..table[i].len() {
            pos = t;
            let mut current_dir = (-1, 0);
            let mut m: HashMap<((i32, i32), (i32, i32)), i32> = HashMap::new();

            // let mut spaces = vec![];
            let mut looped = false;
            let mut changed = false;
            if table[i][j] != '#' {
                table[i][j] = 'O';
                changed = true;
            }
            //println!("{:?}", table);
            while (pos.0 as usize)< table.len() - 1 && pos.0 > 0 && (pos.1 as usize) < table[0].len() - 1 && pos.1 > 0 && !looped {
                if table[(pos.0 + current_dir.0) as usize][(pos.1 + current_dir.1) as usize] == '#' ||
                table[(pos.0 + current_dir.0) as usize][(pos.1 + current_dir.1) as usize] == 'O' {
                    current_dir = rotate(current_dir);
                } else {
                    pos = (pos.0 + current_dir.0, pos.1 + current_dir.1);
                    // m.entry(pos)
                    //         .or_insert_with(Vec::new)
                    //         .push(current_dir);
                    // m.insert(pos, current_dir);
                    // if !spaces.contains(&pos) {
                        // spaces.push(pos);
                    // } else {
                    let key = (pos, current_dir);
                    //println!("{:?}", key);
                        if m.contains_key(&key) {
                            // println!("{:?}", m);
                            sum += 1;
                            // println!("{:?}", (i, j));
                            looped = true;
                        }
                        m.insert(key,1);
                    // }
                }
            }
            if changed {
                table[i][j] = '.';
            }
        }
    }
    println!("{}", sum);
}

fn rotate(mut current_dir: (i32, i32)) -> (i32, i32) {
    if current_dir == (-1, 0) {
        current_dir = (0, 1);
    } else if current_dir == (0, 1) {
        current_dir = (1, 0);
    } else if current_dir == (1, 0) {
        current_dir = (0, -1);
    } else {
        current_dir = (-1, 0);
    }
    current_dir
}
