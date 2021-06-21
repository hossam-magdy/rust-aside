use rust_workshop::p7_rotate_array::rotate;

fn main() {
    let mut v = vec![1, 2, 3, 1];
    rotate(&mut v, 5);
    assert_eq!(v, vec![1, 1, 2, 3]);
    println!("{:?}", v);
}
