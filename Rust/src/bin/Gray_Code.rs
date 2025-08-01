use std::io;
use std::{fmt::Debug, str::FromStr};
fn main() {
    let n = read_num();
    let upper = fast_pow(2, n, usize::MAX);
    for i in 0..upper {
        println!("{:0n$b}", i ^ (i >> 1));
    }
}

fn fast_pow(mut base: usize, mut exp: usize, m: usize) -> usize {
    let mut res = 1;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            res = base * res % m;
        }
        base = base * base % m;
        exp = exp >> 1;
    }
    res
}

fn read_num<T>() -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().parse::<T>().unwrap()
}
