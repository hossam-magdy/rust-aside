use rust_workshop::p5_move_zeros::move_zeroes;

fn main() {
    println!(
        "{:?}\n[1, 3, 12, 0, 0]",
        move_zeroes(&mut vec![0, 1, 0, 3, 12])
    );
    println!("{:?}\n[0]", move_zeroes(&mut vec![0]));
}
