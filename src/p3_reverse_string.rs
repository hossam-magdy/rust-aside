/**
// @see: https://www.udemy.com/course/master-the-coding-interview-data-structures-algorithms/learn/lecture/12308764#notes
fn main() {
    println!("{}", reverse_string1("abc"));
    println!("{}", reverse_string2("abc"));
}
*/

pub fn reverse_string1(s: &str) -> String {
    // let str = s.to_string();
    let len = s.len();
    let mut out = String::new();
    for i in (0..len).rev() {
        out.push_str(&s[i..(i + 1)])
    }

    return out;
}

pub fn reverse_string2(s: &str) -> String {
    let len = s.len();
    let mut chars = s.chars();
    let mut out = String::new();
    for _i in 0..len {
        out.push(chars.nth_back(0).unwrap())
    }

    return out;
}
