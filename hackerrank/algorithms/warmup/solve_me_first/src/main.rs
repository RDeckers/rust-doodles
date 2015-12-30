use std::io;
use std::io::prelude::*;

fn main() {

	let stdin = io::stdin();

	let a: i32 = stdin.lock().lines().next().unwrap().unwrap().trim().parse().unwrap();
	let b: i32 = stdin.lock().lines().next().unwrap().unwrap().trim().parse().unwrap();

	println!("{}", a + b);

}
