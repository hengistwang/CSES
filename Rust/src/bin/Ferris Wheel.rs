use std::io;
use std::{fmt::Debug, str::FromStr};
fn main() {
    let nk: Vec<usize> = read_nums();
    let n = nk[0];
    let k = nk[1];
    let mut xs: Vec<usize> = read_nums();
    xs.sort_unstable();
    let mut i = 0;
    let mut j = xs.len() - 1;
    let mut res = 0;
    while i <= j && i < n && j < n {
        let sum = xs[i] + xs[j];
        if sum <= k {
            i += 1;
            j -= 1;
        } else {
            j -= 1;
        }
        res += 1;
    }
    println!("{}", res);
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
