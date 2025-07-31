#![allow(dead_code)]
use std::{cmp::max, fmt::Debug, io, str::FromStr, usize};

fn main() {
    let _ : usize = read_num();
    let  xs:Vec<usize> = read_xs();
    let mut left_biggest = 0;
    let mut res = 0;
    for i in xs {
        left_biggest = max(left_biggest, i);
        if i < left_biggest {
            res += left_biggest - i;
        }
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

fn read_xs<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.split_whitespace()
        .map(|x| x.trim().parse::<T>().unwrap())
        .collect()
}
