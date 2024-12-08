use std::fs;

const FILE: &str = "input.txt";
const TEST: &str = "test.txt";

fn main() {
    let reader = fs::read_to_string(FILE).expect("Could not read file");
    let table = reader.lines().collect::<Vec<_>>();
    let mut results = vec![];
    let mut nums: Vec<Vec<i64>> = vec![];
    let mut sum = 0;

    for line in table {
        let splits: Vec<_> = line.split(": ").collect();
        results.push(splits[0].parse::<i64>().unwrap());
        nums.push(splits[1].split_whitespace().into_iter().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>());
    }

    let mut last = -1;

    for i in 0..results.len() {
        for j in 0..=(max_ternary(nums[i].len() - 1)) {
            // println!("Ternary: {:?}", to_ternary(j, nums[i].len() - 1));
            if results[i] == do_op(nums[i].clone(), to_ternary(j, nums[i].clone().len() - 1)) && last != i as i64 {
                // println!("{}", i);
                sum += results[i];
                last = i as i64;
            }
        }
    }

    println!("{}", sum);

    // let mut n = vec![];
    // n.push(2);
    // n.push(2);
    // let mut o = vec![];
    // o.push(2);
    // println!("{:?}", do_op(n, o));
    // println!("results: {:?},\tnums: {:?}", results, nums);
}

fn to_binary(n: i64, len: usize) -> Vec<i64> {
    let mut nu = format!("{:b}", n).chars().into_iter().map(|x| x.to_digit(10).unwrap() as i64).collect::<Vec<i64>>();
    while nu.len() < len {
        nu.insert(0, 0);
    }
    return nu;
}

fn to_ternary(mut n: i64, len: usize) -> Vec<i64> {
    let mut nu = vec![];
    while n > 0 {
        nu.insert(0, n % 3);
        n /= 3;
    }
    while nu.len() < len {
        nu.insert(0, 0);
    }
    return nu;
}

fn max_binary(len: usize) -> i64 {
    let mut sum = 0;
    for i in 0..len {
        sum += 2_i64.pow(i as u32);
    }
    return sum;
}

fn max_ternary(len: usize) -> i64 {
    let mut sum = 0;
    for i in 0..len {
        sum += 2 * (3_i64.pow(i as u32));
    }
    return sum;
}

fn do_op(nums: Vec<i64>, op: Vec<i64>) -> i64 {
    let mut ans = 0;
    for i in 0..op.len() {
        if i == 0 {
            if op[i] == 1 {
                ans = nums[i] * nums[i+1];
            } else if op[i] == 0 {
                ans = nums[i] + nums[i+1];
            } else {
                // println!("trying {} || {}", )
                ans = (nums[i] * 10_i64.pow((format!("{}", nums[i+1]).chars().count()) as u32)) + nums[i+1];
            }
        } else {
            if op[i] == 1 {
                ans *= nums[i+1];
            } else if op[i] == 0 {
                ans += nums[i+1];
            } else {
                ans = (ans * 10_i64.pow((format!("{}", nums[i+1]).chars().count()) as u32)) + nums[i+1];
            }
        }
    }
    return ans;
}