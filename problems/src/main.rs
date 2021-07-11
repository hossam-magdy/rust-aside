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
