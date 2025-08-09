use std::io;
use std::{fmt::Debug, str::FromStr};
fn main() {
    let n = read_num();
    for _ in 0..n {
        fun();
    }
}

fn fun() {
    let xs: Vec<u64> = read_nums();
    let a = xs[0];
    let b = xs[1];
    let sum = a + b;
    if a == 0 && b == 0 {
        println!("YES");
        return;
    }
    if a == 0 || b == 0 || sum % 3 != 0 || a * 2 < b || b * 2 < a {
        println!("NO");
        return;
    }
    println!("YES");
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
fn read_nums<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.split_ascii_whitespace()
        .map(|s| s.trim().parse::<T>().unwrap())
        .collect::<Vec<T>>()
}
