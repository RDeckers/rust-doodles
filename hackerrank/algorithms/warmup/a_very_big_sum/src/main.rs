use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input);
    {
        let mut input = String::new();
        stdin.read_line(&mut input);
        let iterator = input.split_whitespace();
        let mut sum = 0u64;
        for i in iterator{
            let tmp : u64 = i.parse().unwrap();
            sum = sum + tmp;
        }
        println!("{}",sum);
    }
}
