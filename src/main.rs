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
