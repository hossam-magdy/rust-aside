// @see: https://leetcode.com/problems/rotate-array/description/
// Given an array, rotate the array to the right by k steps, where k is non-negative.

/*
use rust_workshop::p7_rotate_array::rotate;

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    rotate2(&mut v, 3);
    assert_eq!(v, vec![4, 5, 6, 1, 2, 3]);
    println!("{:?}", v);

    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    rotate2(&mut v, 3);
    assert_eq!(v, vec![5, 6, 7, 1, 2, 3, 4]);
    println!("{:?}", v);

    let mut v = vec![1, 2];
    rotate2(&mut v, 2);
    assert_eq!(v, vec![1, 2]);
    println!("{:?}", v);

    let mut v = vec![1, 2, 3, 4, 5, 6];
    rotate2(&mut v, 4);
    assert_eq!(v, vec![3, 4, 5, 6, 1, 2]);
    println!("{:?}", v);
}
*/

use std::usize;

// Time: O(n), Space: O(n)
pub fn rotate1(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let rotate_by = (k % len as i32) as usize;
    let mut removed = vec![];

    for i in 0..rotate_by {
        let &n = nums.get(len - i - 1).unwrap();
        removed.push(n);
        nums.remove(len - i - 1);
    }

    for i in removed {
        nums.insert(0, i)
    }
}

// Time: O(n), Space: O(1)
pub fn rotate2(nums: &mut Vec<i32>, k_in: i32) {
    let len = nums.len();
    let k = k_in as usize % len;
    let mut n_moves = 0;
    let mut i_src;
    let mut i_dest;
    let mut remained;
    let mut tmp;
    if len <= 1 || k <= 0 {
        return ();
    }
    for j in 0..k {
        i_src = j;
        i_dest = (i_src + k) % len;
        remained = nums[i_src];
        loop {
            tmp = nums[i_dest];
            nums[i_dest] = remained;
            remained = tmp;
            i_src = i_dest;
            i_dest = (i_dest + k) % len;
            n_moves = n_moves + 1;
            if i_src == j || n_moves == len {
                break;
            }
        }
        if n_moves == len {
            return ();
        }
    }
}
