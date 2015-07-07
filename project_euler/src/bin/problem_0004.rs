fn is_palindrome(number: u32, base: u32) -> bool{
  let mut vec = Vec::<u32>::new();
  let mut divisor = 1;
  while (number / divisor) != 0 {
    vec.push((number/divisor)%base);
    divisor *= base;
  }
  for i in 0..vec.len()/2{
    if vec[i] != vec[vec.len()-1-i]{
      return false
    }
  }
  true
}

fn find_largest_palindrome() -> u32{
  let mut largest = 0;
  for i in (100..1000).rev(){
    for j in (i..1000).rev(){
      if i*j <= largest{
        break;
      }
      if is_palindrome(i*j, 10){
        largest = i*j;
      }
    }
  }
  return largest;
}

fn main() {
  println!("{}", find_largest_palindrome());
}
