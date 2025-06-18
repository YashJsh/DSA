use std::{f32::MAX_EXP, i32};

pub fn min_max(mut nums : Vec<i32>, p : i32) -> i32{
    let n = nums.len();
    nums.sort();
    let mut i = 0;
    let mut max_diff = nums[n-1] - nums[0];

    let mut result = i32::MAX;
    
    while i <= max_diff {
        let mid = i + (max_diff - i )/2 ;
        println!("Mid is : {}", mid);
        if isValid(&nums, mid, p, n) {
            result = mid;
            max_diff = mid - 1;
        }else {
            i = mid + 1;
        }
    }
    println!("Result is : {}", result);
    result
}

fn isValid(nums : &Vec<i32>, mid : i32, p : i32, n : usize) -> bool{
    let mut i  = 0;
    let mut count_pairs = 0;

    while i < n-1 {
        if nums[i + 1] - nums[i] <= mid {
            count_pairs += 1;
            i += 2;
        } else{
            i += 1;
        }
    }
    count_pairs >= p
}