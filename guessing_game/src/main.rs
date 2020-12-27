use std::io;    


fn main() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("This is the end of the line and it has not even begun!");
    line.pop();
    let n = line.chars().count();
    //println!("{}", n);
    print!("h");
    let n_e = 2*(n-2);
    for _ in 0..n_e {
        print!("e");
    }
    println!("y");
}
