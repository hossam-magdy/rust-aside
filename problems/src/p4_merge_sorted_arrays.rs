/**
// @see: https://www.udemy.com/course/master-the-coding-interview-data-structures-algorithms/learn/lecture/12309362#notes
fn main() {
    println!(
        "{:?}\n[0, 3, 4, 4, 6, 30, 31]",
        merge_sorted_arrays(&[0, 3, 4, 31], &[4, 6, 30])
    );
}
*/

pub fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut arr_merged = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;
    let len1 = arr1.len();
    let len2 = arr2.len();
    while i1 < len1 || i2 < len2 {
        if i2 >= len2 || arr1[i1] <= arr2[i2] {
            arr_merged.push(arr1[i1]);
            i1 = i1 + 1;
        } else {
            arr_merged.push(arr2[i2]);
            i2 = i2 + 1;
        }
    }

    return arr_merged;
}
