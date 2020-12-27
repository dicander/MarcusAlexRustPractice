use std::io;
use std::cmp::Ordering;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: u32 = n.trim().parse().unwrap();
    let mut last = String::new();
    let mut current = String::new();
    let mut answer = 0;
    for _ in 0..n {
        current = String::new();
        io::stdin().read_line(&mut current);
        current.pop();
        match current.cmp(&last) {
            Ordering::Equal => answer += 1,
            _ => {}
        }
        //println!("*{}={}*", last, current);
        last = current.clone();
    }
    println!("{}", answer);
    //println!("{}", n);
}
