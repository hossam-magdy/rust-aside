use std::collections::HashSet;

/* check whether 2 arrays contains_common_number
fn main() {
    println!("{}", contains_common_number1(&[1, 2, 3], &[4, 5, 6]));
    println!("{}", contains_common_number1(&[1, 2, 4], &[4, 5, 6]));
    println!("{}", contains_common_number2(&[1, 2, 3], &[4, 5, 6]));
    println!("{}", contains_common_number2(&[1, 2, 4], &[4, 5, 6]));
}
*/

pub fn contains_common_number1(s1: &[i32], s2: &[i32]) -> bool {
    for i in 0..s1.len() {
        // in s1 {
        for j in 0..s2.len() {
            // in s1 {
            // for j in s2 {
            if s1[i] == s2[j] {
                return true;
            }
        }
    }

    return false;
}

pub fn contains_common_number2(s1: &[i32], s2: &[i32]) -> bool {
    let mut set1 = HashSet::new();

    for i in 0..s1.len() {
        set1.insert(s1[i]);
    }

    for j in 0..s2.len() {
        if set1.contains(&s2[j]) {
            return true;
        };
    }

    return false;
}
