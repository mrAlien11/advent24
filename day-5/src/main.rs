use std::{collections::HashMap, fs};

const FILE: &str = "input.txt";
const TEST: &str = "test.txt";

fn main() {
    let reader = fs::read_to_string(FILE).expect("Could not read file");
    let temp = reader.split("\n\n").collect::<Vec<_>>();

    let rules = temp[0].split("\n").into_iter().map(|x| x.split("|").into_iter().map(|y| y.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let updates = temp[1].split("\n").into_iter().map(|x| x.split(",").into_iter().map(|y| y.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let checker = updates.clone();

    let mut table: HashMap<i32, Vec<i32>> = HashMap::new();
    rules.iter().for_each(|x| {
        table.entry(x[0])
            .or_insert_with(Vec::new) 
            .push(x[1]); 
    });
    let bins = incorrects(updates, &table);
    let mut sum = 0;
    for i in 0..bins.len() {
        if bins[i] == 1 {
            sum += make_good(checker[i].clone(), &table)[(checker[i].len()-1)/2]
        }
    }
    // println!("{:?}", bins);
    println!("{}", sum);
    // println!("{:?}", table);
    // println!("{:?}", updates);
}

fn compare(a: Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![];
    for n in a {
        for nu in b {
            if n == *nu && !ans.contains(&n) {
                ans.push(*nu);
            }
        }
    }
    return ans;
}

fn incorrects(updates: Vec<Vec<i32>>, table: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut bins = vec![];
    updates.clone().iter().for_each(|_x| bins.push(0));
    let mut k = 0;

    for update in updates {
        if !is_good(update, table.clone()).0 {
            bins[k] = 1;
        }
        k += 1;
    }
    return bins;
}

fn make_good(mut update: Vec<i32>, table: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let (is_good_value, (i, j)) = is_good(update.clone(), table.clone());

    if !is_good_value {
        update.swap(i, j);
        update = make_good(update, table);
    }
    
    return update;
}

fn is_good(update: Vec<i32>, table: HashMap<i32, Vec<i32>>) -> (bool, (usize, usize)) { 
    for i in 0..update.len() {
        if table.contains_key(&update[i]) {
            let arr = table.get(&update[i]).unwrap();
            let compared = compare(update.clone(), arr);
            for c in compared {
                if update.contains(&c) && !((update.iter().position(|&r| r == c).unwrap()) > i){
                    return (false, (i, update.iter().position(|&r| r == c).unwrap()));
                }
            }
        }
    }
    return (true, (0, 0));
}