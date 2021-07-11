// @see: https://leetcode.com/problems/contains-duplicate/description/
// Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

/*
use rust_workshop::p6_contains_duplicate::contains_duplicate;

fn main() {
    println!(
        "{:?}: true: [1,2,3,1]",
        contains_duplicate(vec![1, 2, 3, 1])
    );
    println!(
        "{:?}: false: [1,2,3,4]",
        contains_duplicate(vec![1, 2, 3, 4])
    );
    println!(
        "{:?}: true: [1,1,1,3,3,4,3,2,4,2]",
        contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2])
    );
    println!("{:?}", contains_duplicate(vec![]));
    println!("{:?}", contains_duplicate(vec![1, 1]));
}
*/

use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for n in nums {
        if set.contains(&n) {
            return true;
        }
        set.insert(n);
    }
    return false;
}
