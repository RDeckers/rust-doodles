//F(n+2) = F(n+1) + F(n)
//  = F(n-1)+2F(n) = even iff F(n-1) = even. so skep ahead three from every even
//  = F(n-1) + 2*(F(n-1)+F(n-2))
//  = 3F(n-1) + 2*F(n-2)
//  = 3F(n-1) + F(n-2)+F(n-3)+F(n-4);
//  = 4F(n-1) + F(n-4).
// let F(n-4) bet the first even number, 2, and F(n-1) = 8.

fn main() {
  let (mut prev, mut cur, mut acumulator) =  (2,8, 2);
  while cur < 4000000{
    let tmp = cur;
    acumulator += tmp;
    cur = 4*tmp+prev;
    prev = tmp;
  }
  println!("{}", acumulator);
}
