// @see: https://leetcode.com/problems/maximum-subarray
// Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.

/*
use rust_workshop::p8_max_sub_array::max_sub_array1;

fn main() {
    assert_eq!(max_sub_array1(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);

    assert_eq!(max_sub_array1(vec![-3, 1, -2, 2]), 2);

    assert_eq!(max_sub_array1(vec![-3]), -3);
}
*/

use std::cmp::max;

pub fn max_sub_array_wrong_self_attempt(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    if nums.len() == 1 {
        return nums[0];
    }

    let mut sign_change_arr = Vec::new();
    let mut current_group_sum = nums[0];
    let mut current_group_sign = if nums[0] >= 0 { 1 } else { -1 };
    for i in 1..nums.len() {
        let sign = if nums[i] >= 0 { 1 } else { -1 };
        if current_group_sign != sign {
            sign_change_arr.push(current_group_sum);
            current_group_sum = nums[i];
            current_group_sign = sign;
        } else {
            current_group_sum = current_group_sum + nums[i];
        }
    }
    sign_change_arr.push(current_group_sum);

    println!("{:?}", sign_change_arr);

    if sign_change_arr.len() == 1 {
        if current_group_sign == 1 {
            return sign_change_arr[0];
        } else {
            let &tmp = nums.iter().max().unwrap();
            return tmp;
        }
    }

    let mut sum1 = sign_change_arr[0];
    let mut sum2 = sign_change_arr[1];
    let mut sum = if sum1 > sum2 { sum1 } else { sum2 };
    if sign_change_arr.len() == 2 {
        return sum;
    }
    for i in 2..sign_change_arr.len() {
        let sum_all3 = sign_change_arr[i] + sum2 + sum1;
        let &tmp = vec![sum1, sum2, sign_change_arr[i], sum, sum_all3]
            .iter()
            .max()
            .unwrap();
        sum = tmp;
        sum1 = sum2;
        sum2 = sign_change_arr[i];
    }
    return sum;
}

// using Kadane's algorithm/approach/method
pub fn max_sub_array1(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let mut current_sum = nums[0];
    let mut best_sum = nums[0];
    for i in 1..nums.len() {
        current_sum = max(nums[i], nums[i] + current_sum);
        best_sum = max(best_sum, current_sum);
    }
    return best_sum;
}

pub fn max_sub_array2(nums: Vec<i32>) -> i32 {
    nums.iter()
        .fold((0, std::i32::MIN), |(current, best), &num| {
            (0.max(current + num), best.max(current + num))
        })
        .1
}
