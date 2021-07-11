// @see: https://www.udemy.com/course/master-the-coding-interview-data-structures-algorithms/learn/lecture/12314712#overview
/**
use rust_workshop::p9_first_recurring_char::first_recurring_char;

fn main() {
    assert_eq!(
        first_recurring_char(vec![2, 5, 1, 2, 3, 5, 1, 2, 4]),
        Some(2)
    );

    assert_eq!(
        first_recurring_char(vec![2, 1, 1, 2, 3, 5, 1, 2, 4]),
        Some(1)
    );

    assert_eq!(first_recurring_char(vec![2, 3, 4, 5]), None);

    assert_eq!(
        first_recurring_char(vec!['a', 'b', 'c', 'b', 'a']),
        Some('b')
    );
}
*/
pub fn first_recurring_char<T: Ord>(arr: Vec<T>) -> Option<T> {
    let mut occured_chars = Vec::new();
    for e in arr {
        if occured_chars.contains(&e) {
            return Some(e);
        }
        occured_chars.push(e);
    }

    return None;
}
