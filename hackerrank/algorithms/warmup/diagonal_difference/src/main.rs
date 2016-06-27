use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input);
    let count : usize = input.trim().parse().unwrap();
    let mut sum_a = 0i64;
    let mut sum_b = 0i64;
    for i in 0..count{
        let mut input = String::new();
        stdin.read_line(&mut input);
        let vec : Vec<_> = input.split_whitespace().collect();
        let tmp_a : i64 = vec[i].parse().unwrap();
        let tmp_b : i64 = vec[count-i-1].parse().unwrap();
        sum_a = sum_a + tmp_a;
        sum_b = sum_b + tmp_b;
    }
    println!("{}", (sum_a-sum_b).abs());
}
