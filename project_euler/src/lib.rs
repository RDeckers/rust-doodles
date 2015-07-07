pub mod primes;

pub fn triangle_number(n: u64) -> u64{//https://en.wikipedia.org/wiki/Triangular_number
  n*(n+1)/2
}

pub fn pyramid_number(n: u64) -> u64{//https://en.wikipedia.org/wiki/Square_pyramidal_number
  triangle_number(n)*(2*n+1)/3
}
