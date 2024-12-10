use std::fs;

const FILE: &str = "input.txt";
const TEST: &str = "test.txt";

fn main() {
    // let reader = fs::read_to_string(TEST).expect("Could not read file");
    // let cs = reader.chars().collect::<Vec<_>>();
    // let mut output = vec![];
    // let mut frees = vec![];
    // let mut t = vec![];
    // t.push(-1);

    // let mut id = 0;
    // for i in 0..cs.len() {
    //     let mut j = 0;
    //     if i % 2 == 0 {
    //         let mut temp = vec![];
    //         while j < cs[i].to_digit(10).unwrap() {
    //             temp.push(id);
    //             j += 1;
    //         }
    //         output.push((temp, false));
    //         id += 1;
    //     } else {
    //         frees.push((output.len() as i32, cs[i].to_digit(10).unwrap() as i32));
    //         while j < cs[i].to_digit(10).unwrap() {
    //             output.push((t.clone(), false));
    //             j += 1;
    //         }
    //     }
    // }

    // // for i in 0..output.len() {
    // //     println!("{:?}", output[i].0);
    // // }

    // let mut fre = frees.clone();

    // // for i in 0..output.len() {
    // //     print!("{:?}", output[i].0);
    // // }
    // // print!("\n");
    // // println!("{:?}", fre);
    // for i in (0..output.len()).rev() {
    //     if output[i].0[0] != -1 && !output[i].1{
    //         // println!("{:?}", output[i].0);
    //         for k in 0..fre.len() {
    //             if fre[k].1 >= output[i].0.len() as i32 && (fre[k].0 as usize) < i{
    //                 // println!("{:?}", output[i]);
    //                 output[i].1 = true;
    //                 let ln = output[i].0.len();
    //                 for j in 0..ln {
    //                     if j == 0 {
    //                         output.swap(fre[k].0 as usize, i); // Swap first element
    //                     } else {
    //                         let target_index = i + j;
    //                         output.remove(fre[k].0 as usize+ j);
    //                         for l in (k)..fre.len() {
    //                             if fre[l].0 > 0 {
    //                                 fre[l].0 -= 1;
    //                             }
    //                         }
    //                         if target_index < output.len() {
    //                             output.insert(target_index-1, (t.clone(), false));
    //                         } else {
    //                             output.push((t.clone(), false));
    //                         }
    //                     }
    //                 }
    //                 fre[k].0 += output[fre[k].0 as usize].0.len() as i32;
    //                 // println!("free.1: {},\t output[i].len(): {}", free.1, output[free.0 - 1].0.len());
    //                 fre[k].1 -= output[fre[k].0 as usize].0.len() as i32 + 1;
    //                 break;
    //             }
    //         }
    //         // for i in 0..output.len() {
    //         //     print!("{:?}", output[i].0);
    //         // }
    //         // print!("\n");
    //         // println!("{:?}", fre);
    //     }
    // }
    
    // // for i in 0..output.len() {
    // //     print!("{:?}", output[i].0);
    // // }
    // let mut sum: u64 = 0;
    // let mut pos = 0;
    // for out in output {
    //     if out.0[0] != -1 {
    //         for file in out.0 {
    //             // println!("file: {} * pos: {}", file, pos);
    //             sum += (file as u64)*pos;
    //             pos += 1;
    //         }
    //     } else {
    //         pos += 1;
    //     }
    // }
    // println!("{:?}", sum);
}


fn part_1() {
    let reader = fs::read_to_string(TEST).expect("Could not read file");
    let cs = reader.chars().collect::<Vec<_>>();
    let mut output = vec![];

    let mut id = 0;
    let mut ns = 0;
    for i in 0..cs.len() {
        let mut j = 0;
        if i % 2 == 0 {
            while j < cs[i].to_digit(10).unwrap() {
                output.push(id);
                ns += 1;
                j += 1;
            }
            id += 1;
        } else {
            while j < cs[i].to_digit(10).unwrap() {
                output.push(-1);
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
        if !output.get(0..ns).unwrap().contains(&-1) {
            break;
        }
        let swap = len - 1 - i;
        if output[swap] != -1 {
            // println!("here");
            for j in 0..len {
                if output[j] == -1 {
                    // println!("i: {}\tj: {}", swap, j);
                    output.swap(swap, j);
                    break;
                }
            }
        }
    }

    let mut sum: u64 = 0;
    let mut k = 0;
    while output[k] != -1 {
        sum += output[k] as u64 * (k) as u64;
        k += 1;
    }
    println!("{:?}", sum);
}