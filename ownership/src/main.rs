use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut input = String::new();

    let mut ref1 = &mut input;
    let ref2 = &input;
    println!("{} {}", ref1, ref2);

    dummy_fn(&input);
    io::stdout().write_all("Enter a number : \n".as_bytes()).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let n = i32::from_str(&input.trim()).unwrap();
    let next = next(n);
    println!("The next is {}", next);
}

fn dummy_fn(s:&String){}

fn next(n:i32) -> i32{
    n+1
}
