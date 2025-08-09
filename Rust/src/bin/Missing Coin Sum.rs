use std::io;
use std::{fmt::Debug, str::FromStr};
fn main() {
    let _n: u64 = read_num();
    let mut xs: Vec<u64> = read_nums();
    xs.sort_unstable();
    let mut res = 1;
    for i in xs {
        if i > res {
            println!("{}", res);
            return;
        }
        res += i;
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
