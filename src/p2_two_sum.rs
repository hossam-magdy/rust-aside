// indices of two numbes in array which sum is `target`
// @see: https://leetcode.com/problems/two-sum/description/

use std::collections::HashMap;

/*
fn main() {
    println!(
        "output:   {:?}\nexpected: {:?}\n",
        rust_workshop::p2_two_sum::two_sum2(vec![2, 7, 11, 15], 9),
        vec![0, 1]
    );
    println!(
        "output:   {:?}\nexpected: {:?}\n",
        rust_workshop::p2_two_sum::two_sum2(vec![3, 2, 4], 6),
        vec![1, 2]
    );
    println!(
        "output:   {:?}\nexpected: {:?}\n",
        rust_workshop::p2_two_sum::two_sum2(vec![3, 3], 6),
        vec![0, 1]
    );
}
*/

pub fn two_sum1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    return Vec::new();
}

pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut set = HashMap::new();

    for i in 0..nums.len() {
        let compl = target - nums[i];
        if set.contains_key(&compl) {
            let &compl_index = set.get(&compl).unwrap();
            return vec![compl_index as i32, i as i32];
        }
        set.insert(nums[i], i);
    }

    return Vec::new();
}
