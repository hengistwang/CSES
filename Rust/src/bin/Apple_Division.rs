use std::cmp::min;
use std::io;
use std::{fmt::Debug, str::FromStr};
fn main() {
    let n: usize = read_num();
    let xs: Vec<i64> = read_nums();
    let total: i64 = xs.iter().sum();
    let mut res = i64::MAX;
    let mut subsets = 0;
    gen_subsets(n, &xs, 0, &mut subsets, total, &mut res);
    println!("{}", res);
}

fn gen_subsets(n: usize, xs: &Vec<i64>, i: usize, subsets: &mut i64, sum: i64, res: &mut i64) {
    if i == n {
        *res = min(*res, (sum - *subsets - *subsets).abs());
        return;
    }
    *subsets += xs[i];
    gen_subsets(n, xs, i + 1, subsets, sum, res);
    *subsets -= xs[i];
    gen_subsets(n, xs, i + 1, subsets, sum, res);
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
