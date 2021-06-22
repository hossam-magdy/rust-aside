use rust_workshop::p8_max_sub_array::max_sub_array1;

fn main() {
    assert_eq!(max_sub_array1(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);

    assert_eq!(max_sub_array1(vec![-3, 1, -2, 2]), 2);

    assert_eq!(max_sub_array1(vec![-3]), -3);
}
