use num_bigint::BigUint;
//use num_traits::{One};
//use std::io;
use std::io::{self, Write};

fn main() {
    let mut s = String::new();
    print!("{:10} : ",0);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut s)
        .expect("Could not read Integer!");
    //println!("{} was read as a string", s);
    let mut n: BigUint = BigUint::parse_bytes(s.trim().as_bytes(),10).unwrap();
    let mut i = 0;
    let one = BigUint::parse_bytes("1".as_bytes(), 10).unwrap();
    let two = BigUint::parse_bytes("2".as_bytes(), 10).unwrap();
    let three = BigUint::parse_bytes("3".as_bytes(), 10).unwrap();
    while n != one {
        if &n % &two == one {
            n = &three*&n + &one;
        } else {
            n = &n / &two;
        }
        i = i+1;
        println!("{:10} : {:10}", i, n);
    }
}
