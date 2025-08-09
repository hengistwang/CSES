use std::io;
use std::{fmt::Debug, str::FromStr};
fn main() {
    let mut n: u64 = read_num();
    let mut res = 0;
    while n > 0 {
        res += n / 5;
        n /= 5;
    }
    println!("{}", res);
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
