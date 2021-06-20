use rust_workshop::p4_merge_sorted_arrays::merge_sorted_arrays;

fn main() {
    println!(
        "{:?}\n[0, 3, 4, 4, 6, 30, 31]",
        merge_sorted_arrays(&[0, 3, 4, 31], &[4, 6, 30])
    );
}
