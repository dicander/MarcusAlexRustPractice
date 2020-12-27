use std::io;
use std::io::Read;


fn main() -> io::Result<()> {

    //let lines : Vec<String> =  io::stdin().read

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;
    
    let contents: Vec<char> = buffer
        .lines()
        .skip(1)
        .map(|s| s.chars().next().unwrap())
        .collect();

    //println!("{:?}", contents);

    let result : i32 = contents.iter()
        .zip(contents.iter().skip(1))
        .map(|(prev, curr)| (prev == curr) as i32)
        .sum();

    println!("{}", result);

    Ok(())
}




