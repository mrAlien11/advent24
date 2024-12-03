use std::{fs, collections::HashMap};

const FILE: &str = "input.txt";
const TEST: &str = "test.txt";

fn main() {
    let mut table = HashMap::new();
    let reader = fs::read_to_string(FILE).expect("could not read");
    let lines = reader.lines().collect::<Vec<&str>>();
    let mut first: Vec<i32>= vec![];
    let mut second: Vec<i32> = vec![];
    for line in lines {
        let temp = line.split_whitespace().collect::<Vec<&str>>();
        first.push(temp[0].parse().unwrap());
        second.push(temp[1].parse().unwrap());

        let n: i32 = temp[1].parse().unwrap();

        if table.get(&n).is_none() {
            table.insert(n, 1);
        } else {
            table.insert(n, table.get(&n).unwrap() +1);
        }
    }
    let mut sum = 0;
    for number in first {
        // table.insert(number, 0);
        // for nums in &second {
        //     if nums == &number {
        //         table.insert(number, table.get(&number).unwrap() +1);
        //     }
        // }
        if table.get(&number).is_some() {
            sum += number * table.get(&number).unwrap();
        }
    }
    // first.sort();
    // second.sort();
    // // let mut i = 0;
    // // while i < first.len() {
    // //     if second[i] < first[i] {
    // //         sum += first[i] - second[i];
    // //     } else {
    // //         sum += second[i] - first[i];
    // //     }
    // //     i += 1;
    // // }
    // // println!("first: {:?}", first);
    // // println!("second: {:?}", second);
    // for number in first {
    //     let mut i = 0;
    //     let mut j = 0;
    //     let mut passed = false;
    //     let mut length = 0;
    //     // println!("number: {}", number);
    //     while i < second.len() {
    //         if number == second[i] && !passed {
    //             j = i;
    //             passed = true;
    //         }
    //         if second[i] > number && passed {
    //             length = i - j;
    //             i = second.len();
    //         }
    //         i += 1;
    //     }
    //     // println!("{}", length);
    //     sum += number * (length as i32);
    // }
    println!("{}", sum);
}
