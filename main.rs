use std::collections::HashMap;

fn median(nums: &Vec<i32>) -> f64 {
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    let len = sorted_nums.len();
    if len % 2 == 0 {
        (sorted_nums[len / 2 - 1] as f64 + sorted_nums[len / 2] as f64) / 2.0
    } else {
        sorted_nums[len / 2] as f64
    }
}

fn mode(nums: &Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    for num in nums {
        *counts.entry(*num).or_insert(0) += 1;
    }

    let mut max_count = 0;
    let mut mode = 0;
    for (num, count) in counts {
        if count > max_count {
            max_count = count;
            mode = num;
        }
    }
    mode
}

fn main() {
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
    let median_value = median(&numbers);
    let mode_value = mode(&numbers);

    println!("中位数： {}", median_value);
    println!("众数： {}", mode_value);
}