use std::fs;

const FILE: &str = "input.txt";
const TEST: &str = "test.txt";

fn main() {
    let reader = fs::read_to_string(FILE).expect("could not read file");
    // let table = reader.chars();
    let mut table1 = reader;
    let mut sum = 0;
    let mut go = true;

    while table1.as_str().find("mul(").is_some() {
        let mut table2 = table1.clone();
        // println!("{:?}", table2);
        while (table2.as_str().find("do()").is_some() && table2.as_str().find("do()").unwrap() < table2.as_str().find("mul(").unwrap()) || (table2.as_str().find("don't()").is_some() && table2.as_str().find("don't()").unwrap() < table2.as_str().find("mul(").unwrap()) {
            // println!("got here");
            if table2.as_str().find("do()").is_some() && (!table2.as_str().find("don't()").is_some() || table2.as_str().find("do()").unwrap() < table2.as_str().find("don't()").unwrap()) {
                // println!("got here1");
                go = true;
                table2 = table2.split_off(table2.as_str().find("do()").unwrap() + 4);
            } else if table2.as_str().find("don't()").is_some() {
                // println!("got here2");
                go = false;
                table2 = table2.split_off(table2.as_str().find("don't()").unwrap() + 7);
            }
        }
        let result = part_1(table1.clone());
        if go {
            sum += result.0;
        }
        table1 = result.1;
    }
    //println!("{}", table1);

    println!("{}", sum);
}

fn part_1(mut table1: String) -> (i32, String) {
    let mut product = 0;
    let (first, second) = table1.as_str().split_at(table1.as_str().find("mul(").unwrap() + 4);
        let mut len = table1.as_str().find("mul(").unwrap() + 4;
        if second.find(")").is_some() && second.find(")").unwrap() <= 7 && second.find(",").is_some() && second.find(",").unwrap() < second.find(")").unwrap() {
            if second.find(",").unwrap() <= 3 {
                //println!("{},,,,{}", second, second.find(",").unwrap());
                // let (mut num1, second1) = second.split_at(second.find(",").unwrap());
                let seconds = second.split_once(",").unwrap();
                let num1 = seconds.0;
                // let (num2, temp) = seconds.1.split_at(seconds.1.find(")").unwrap());
                let thirds = seconds.1.split_once(")").unwrap();
                let num2 = thirds.0;
                // println!("{}, {}", num1, num2);
                let mut good1 = true;
                let mut good2 = true;
                for c in num1.chars() {
                    if !c.is_ascii_digit() {
                        good1 = false;
                    }
                }
                for c in num2.chars() {
                    if !c.is_ascii_digit() {
                        good2 = false;
                    }
                }
                if good1 && good2 {
                    product = num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
                    len += num1.len() + 1 + num2.len() + 1;
                }
            }
        }
        return (product, table1.split_off(len));
}