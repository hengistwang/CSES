use std::io;
use std::{fmt::Debug, str::FromStr};
fn main() {
    let n: u64 = read_num();
    let m = 1e9 as u64 + 7;
    let res = fast_pow(2, n, m);
    println!("{}", res)
}

fn fast_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
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
