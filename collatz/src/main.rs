use num_bigint::BigUint;
use num_traints::{Zero, One};
use std::io;

fn main() {
    let mut line = String::new;
    io::stdin()
        .read_line(&mut line)
        .expect("Could not read Integer!");
    let mut n: BigUint = BigUint(line.trim())
        .unwrap();
    println!("{}", n);
}
