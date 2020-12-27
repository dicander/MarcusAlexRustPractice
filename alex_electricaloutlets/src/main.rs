use std::io;

fn main() {
    
    let mut num_lines : String = String::new();

    io::stdin()
        .read_line(&mut num_lines)
        .expect("Failed to read line");
    
    let num_lines : i32 = num_lines.trim().parse().unwrap();
    
    for _ in 0 .. num_lines {
        let mut line : String = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let result : i32 = line.split_ascii_whitespace()
            .skip(1)
            .map(|s| s.parse::<i32>().unwrap() - 1)
            .sum::<i32>() + 1;
        println!("{}", result);
    }
}
