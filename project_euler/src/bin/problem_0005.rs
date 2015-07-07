extern crate projecteuler;
use projecteuler::primes::*;
use std::collections::HashMap;

fn main(){
  let mut result_factorized = HashMap::<u64,u64>::new();
  let mut primes = Primes::new();

  for i in 1..21{//TODO: Make this a LCM function.
    let factorization = primes.factorize(i);
    for factor in factorization{
      let current_factor = match result_factorized.get(&factor.0){
        Some(i) => {*i},
        _ => {0}
      };
      if current_factor < factor.1{
        result_factorized.insert(factor.0, factor.1);
      }
    }
  }

  let result = {
    let mut result = 1;
    for x in result_factorized{
      result *= x.0.pow(x.1 as u32);//gather all the prime exponents together
    }
    result
    };
  println!("{}", result);
}
