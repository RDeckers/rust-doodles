fn triangle_number(n: i32) -> i32{
  n*(n+1)/2
}

fn main() {
  let brute_force = (1..1000)//All numbers below 1000
    .filter(|&x| (x%3)*(x%5) ==0)//which are multiples of 3 or 5
    .fold(0, |acc, item| acc + item);//sum them
  let mathy = 3*triangle_number(999/3)
              +5*triangle_number(999/5)
              -15*triangle_number(999/15);
  println!("{}, {}", mathy, brute_force);
}
