use std::io;

fn solve() {
    let mut line = String::new();
    io::stdin().read_line(&mut line);
    let v:Vec<i64> = line.
        split_ascii_whitespace().
        map(|x| x.parse::<i64>().unwrap()).
        collect();
    println!("{}", v.iter().sum::<i64>()-2*v[0]+1);
}


fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n);
    n.pop();
    let n:i32 = n.parse().unwrap();
    //println!("{}", n);
    for _ in 0..n {
        solve();
    }
}
