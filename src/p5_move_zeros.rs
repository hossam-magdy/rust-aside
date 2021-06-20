// @see: https://leetcode.com/problems/move-zeroes/description/
// @see: https://www.udemy.com/course/master-the-coding-interview-data-structures-algorithms/learn/lecture/12310382#notes
// Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
// Note that you must do this in-place without making a copy of the array.

/*
fn main() {
    println!("{:?}\n[1, 3, 12, 0, 0]", move_zeroes(&mut vec![0, 1, 0, 3, 12]));
}
*/

pub fn move_zeroes_immut(nums: &Vec<i32>) -> Vec<i32> {
    let mut zeros_count = 0;
    let mut new_nums: Vec<i32> = Vec::new();

    for i in 0..nums.len() {
        let &n = nums.get(i).unwrap();
        if n == 0 {
            zeros_count = zeros_count + 1;
        } else {
            new_nums.push(n)
        }
    }

    for _j in 0..zeros_count {
        new_nums.push(0);
    }

    return new_nums;
}

pub fn move_zeroes(nums: &mut Vec<i32>) -> Vec<i32> {
    for i in (0..nums.len()).rev() {
        let &n = nums.get(i).unwrap();
        if n == 0 {
            nums.remove(i);
            nums.push(0);
        }
    }

    return nums.clone();
}
