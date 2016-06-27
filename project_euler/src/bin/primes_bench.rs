extern crate projecteuler;
use projecteuler::primes::*;
extern crate time;
use time::PreciseTime;

fn main(){
    let n = 100_000;
    {
        let mut primes = Primes::new();
        let start = PreciseTime::now();
        primes.fill_to_fast(n);
        let end = PreciseTime::now();
        println!("{} seconds to generate {} primes.\nLast prime: {}", start.to(end), n, primes.last().unwrap());
    }
    {
        let mut primes_2 = Primes::new();
        let start = PreciseTime::now();
        primes_2.fill_to(n);
        let end = PreciseTime::now();
        println!("{} seconds to generate {} primes.\nLast prime: {}", start.to(end), n, primes_2.last().unwrap());
    }
}
