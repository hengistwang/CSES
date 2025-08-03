use std::io;
use std::{cmp::max, fmt::Debug, str::FromStr};
fn main() {
    let _n: usize = read_num();
    let xs: Vec<i64> = read_nums();
    let mut res = xs[0];
    let mut cur = 0;
    for i in xs {
        cur = max(i, cur + i);
        res = max(res, cur);
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
