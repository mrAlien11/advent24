use std::fs;

const FILE: &str = "input.txt";
const TEST: &str = "test.txt";

fn main() {
    let reader = fs::read_to_string(FILE).expect("could not read file");

    let table = reader.lines().map(|x| x.split_whitespace().into_iter().map(|y| y.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut sum = 0;
    for line in table {
        let temp1 = line.clone();
        let len = temp1.len();
        if is_safe(line) {
            sum += 1;
        } else {
            let mut i = 0;
            let mut safe = false;
            while i < len && !safe {
                let mut temp = temp1.clone();
                temp.remove(i);
                if is_safe(temp) {
                    sum += 1;
                    safe = true;
                }
                i += 1; 
            }
        }
    }

    println!("{}", sum);
}

fn is_safe(table: Vec<i32>) -> bool {
    let mut increase = false;
    let mut safe = true;

    if table[1] > table[0] {
        increase = true;
    }
    let mut i = 0;
    
    while i < table.len() - 1 {
        if increase {
            if table[i+1] <= table[i] {
                safe = false;
            } else if table[i+1] - table[i] > 3 {
                safe = false;
            }
        } else {
            if table[i+1] >= table[i] {
                safe = false;
            } else if table[i] - table[i+1] > 3 {
                safe = false;
            }
        }
        i += 1;
    }

    return safe;
}