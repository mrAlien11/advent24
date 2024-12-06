use std::fs;

const FILE: &str = "input.txt";
const TEST: &str = "test.txt";

fn main() {
    let reader = fs::read_to_string(FILE).expect("Could not read file");
    let table = reader.lines().into_iter().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut pos: (i32, i32) = (0, 0);
    let mut current_dir: (i32, i32) = (-1, 0);
    let mut spaces: Vec<(i32, i32)> = vec![];
    
    for y in 0..table.len() {
        for x in 0..table[y].len() {
            if table[y][x] == '^' {
                pos = (y as i32, x as i32);
            }
        }
    }
    spaces.push(pos);

    while (pos.0 as usize)< table.len() - 1 && pos.0 > 0 && (pos.1 as usize) < table[0].len() - 1 && pos.1 > 0 {
        if table[(pos.0 + current_dir.0) as usize][(pos.1 + current_dir.1) as usize] == '#' {
            current_dir = rotate(current_dir);
        } else {
            pos = (pos.0 + current_dir.0, pos.1 + current_dir.1);
            if !spaces.contains(&pos) {
                spaces.push(pos);
            }
        }
    }
    println!("{}", spaces.len());
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