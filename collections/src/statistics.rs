use std::collections::HashMap;
use std::io;

// 1. Given a list of integers, use a vector and return the
// mean (the average value), median (when sorted, the value
// in the middle position), and mode (the value that occurs
// most often; a hash map will be helpful here) of the list.

fn get_numbers() -> Vec<u32> {
    println!("Enter an integer to add it to the list, or 'q' to stop.");
    let mut v: Vec<u32> = Vec::new();
    loop {
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        // User requests to stop inputting numbers
        match num.trim() {
            "q" => break,
            _ => {}
        }

        let num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not an integer. Try again");
                continue;
            }
        };

        v.push(num);
        println!("{:?}", v);
    }

    v
}

fn get_mean(nums: &Vec<u32>) -> f32 {
    let sum: u32 = nums.iter().sum();
    return sum as f32 / nums.len() as f32;
}

fn get_median(nums: &Vec<u32>) -> f32 {
    let mut v = nums.clone();
    v.sort();

    let mid_index = v.len() / 2;
    if v.len() % 2 == 0 {
        return get_mean(&vec![v[mid_index], v[mid_index - 1]]);
    } else {
        return v[v.len() / 2] as f32;
    }
}

fn get_mode(nums: &Vec<u32>) -> u32 {
    let mut frequencies = HashMap::new();
    let mut highest_frequency = 0;
    let mut most_frequent_num = 0;
    for num in nums {
        let frequency = frequencies.entry(*num).or_insert(0);
        *frequency += 1;

        // Update most frequently appearing number
        if *frequency > highest_frequency {
            most_frequent_num = *num;
            highest_frequency = match frequencies.get(num) {
                Option::Some(n) => *n,
                Option::None => 0,
            };
        }
    }

    most_frequent_num
}

fn compute_stats(v: &Vec<u32>) {
    println!("Final vector: {:?}", v);
    println!("Mean: {}", get_mean(&v));
    println!("Median: {}", get_median(&v));
    println!("Mode: {}", get_mode(&v));
}

pub fn driver() {
    let v = get_numbers();
    compute_stats(&v);
}
