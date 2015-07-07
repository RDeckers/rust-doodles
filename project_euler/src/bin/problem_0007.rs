extern crate projecteuler;
use projecteuler::primes::*;

fn main(){
  let mut primes = Primes::new();
  primes.fill_to(10001);
  println!("{}", primes.last().unwrap());
}
