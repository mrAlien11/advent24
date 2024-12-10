use std::fs;

const FILE: &str = "input.txt";
const TEST: &str = "test.txt";

fn main() {
    let reader = fs::read_to_string(FILE).expect("Could not read file");
    let cs = reader.chars().collect::<Vec<_>>();
    let mut output = vec![];

    let mut id = 0;
    let mut ns = 0;
    for i in 0..cs.len() {
        let mut j = 0;
        if i % 2 == 0 {
            while j < cs[i].to_digit(10).unwrap() {
                output.push(format!("{}", id).chars().collect::<Vec<_>>()[0]);
                ns += 1;
                j += 1;
            }
            id += 1;
        } else {
            while j < cs[i].to_digit(10).unwrap() {
                output.push('.');
                j += 1;
            }
        }
    }

    let len = output.len();
    for i in 0..len {
        // println!("here");
        // if i == ns + 1 {
        //     break;
        // }
        if !output.get(0..ns).unwrap().contains(&'.') {
            break;
        }
        let swap = len - 1 - i;
        if output[swap] != '.' {
            // println!("here");
            for j in 0..len {
                if output[j] == '.' {
                    // println!("i: {}\tj: {}", swap, j);
                    output.swap(swap, j);
                    break;
                }
            }
        }
    }

    let mut sum: u64 = 0;
    let mut k = 0;
    while output[k] != '.' {
        sum += output[k].to_digit(10).unwrap() as u64 * (k) as u64;
        k += 1;
    }
    println!("{:?}", sum);
}
