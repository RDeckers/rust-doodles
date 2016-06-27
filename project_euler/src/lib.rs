#![feature(test)]
extern crate test;
pub mod primes;

pub fn triangle_number(n: u64) -> u64{//https://en.wikipedia.org/wiki/Triangular_number
  n*(n+1)/2
}

pub fn pyramid_number(n: u64) -> u64{//https://en.wikipedia.org/wiki/Square_pyramidal_number
  triangle_number(n)*(2*n+1)/3
}

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

macro_rules! impl_sqrt_for{
    ($T:ty) => (
        impl Sqrt for $T{
            #[inline]
            fn sqrt(self) -> Self{
                let mut cur  = self;
                let mut next = (cur+1)/2;
                while next-cur > 1 {
                    cur = next;
                    next = (cur + self/cur)/2;
                }
                next
            }
        }
    )
}

impl_sqrt_for!(i8);
impl_sqrt_for!(i16);
impl_sqrt_for!(i32);
impl_sqrt_for!(i64);
impl_sqrt_for!(isize);
impl_sqrt_for!(u8);
impl_sqrt_for!(u16);
impl_sqrt_for!(u32);
impl_sqrt_for!(u64);
impl_sqrt_for!(usize);





#[cfg(test)]
mod tests {
    use test::Bencher;
    use test::black_box;
    use super::primes::*;
    #[bench = "2"]
    fn bench_primes_old(b: &mut Bencher) {
        let mut primes = Primes::new();
        b.iter(||{
            let n = black_box(1_0_000);
            primes.fill_to(n)
        });
    }

    #[bench]
    fn bench_primes_new(b: &mut Bencher) {
        let mut primes = Primes::new();
        b.iter(||{
            let n = black_box(1_0_000);
            primes.fill_to_fast(n)
        });
    }
}
