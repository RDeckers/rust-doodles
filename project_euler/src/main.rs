extern crate projecteuler;
use projecteuler::primes::*;

fn main() {
  let mut primes = Primes::new();
  let factorization = primes.factorize(600851475143);
  println!("{}", factorization.last().unwrap().0);
}
