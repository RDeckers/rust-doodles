use std::io;
use std::io::prelude::*;

fn main() {
  let stdin = io::stdin();
  let line = stdin.lock().lines().next().unwrap().unwrap();
  let line2 = stdin.lock().lines().next().unwrap().unwrap();
    // Removes the new lines /n from the end
  let num_str1 = line.trim();
  let num_str2 = line2.trim();
    // Convert to an int32
  let num1 = num_str1.parse::<i32>().unwrap();
  let num2 = num_str2.parse::<i32>().unwrap();
    // Print the sum
    println!("{}", num1 + num2 );
}
