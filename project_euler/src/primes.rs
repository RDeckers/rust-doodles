pub struct Primes{//How do we restrict Idx to integral values?
  backend: Vec<u64>,
}

impl Primes {
  fn get_next_prime(&mut self) -> u64{
    let last_prime = *(self.backend.last().unwrap());
    let mut test_prime = last_prime+1;
    while self.backend.iter().find(|&x| (test_prime % *x) == 0).is_some(){
      test_prime += 1;
    }
    self.backend.push(test_prime);
    test_prime
  }
  pub fn fill_to(&mut self, n: usize){
    while self.backend.len() < n{
      self.get_next_prime();
    }
  }
  pub fn fill_while_below(&mut self, max: u64){
    while *self.backend.last().unwrap() < max{
      self.get_next_prime();
    }
  }
  pub fn factorize(&mut self, mut to_factor: u64) -> Vec<(u64, u64)>{
    let mut factors = Vec::<(u64, u64)>::new();
    let mut index = 0;
    while to_factor != 1{
      if index >= self.backend.len(){
        self.get_next_prime();
      }
      let (divisor,mut count) = (self.backend[index], 0);
      while to_factor % divisor == 0{
        to_factor /= divisor;
        count += 1;
      }
      if count != 0{
        factors.push((divisor,count));
      }
      index += 1;
    }
    factors
  }
  pub fn new() -> Primes {
    Primes{
      backend: vec![2],
    }
  }
}
