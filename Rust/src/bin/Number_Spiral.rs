use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let n: usize = read_num();
    for _ in 0..n {
        fun();
    }
}

fn fun() {
    let xs: Vec<usize> = read_xs();
    let x = xs[0];
    let y = xs[1];
    let res;
    if x > y {
        if x % 2 == 0 {
            res = x * x - y + 1;
        } else {
            res = (x - 1) * (x - 1) + 1 + y - 1;
        }
    } else {
        if y % 2 == 0 {
            res = (y - 1) * (y - 1) + 1 + x - 1;
        } else {
            res = y * y - x + 1;
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
