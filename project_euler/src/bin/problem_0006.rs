extern crate projecteuler;
use projecteuler::*;

fn main(){
  println!("{}", triangle_number(100).pow(2)-pyramid_number(100));
}
