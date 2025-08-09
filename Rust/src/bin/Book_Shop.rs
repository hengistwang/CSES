use std::io;
use std::{cmp::max, fmt::Debug, str::FromStr};
fn main() {
    let nx: Vec<usize> = read_nums();
    let n = nx[0];
    let x = nx[1];
    let prices: Vec<usize> = read_nums();
    let pages: Vec<usize> = read_nums();
    let mut dp = vec![0; x + 1];
    for i in 0..n {
        for j in (1..=x).rev() {
            if j >= prices[i] {
                dp[j] = max(dp[j], dp[j - prices[i]] + pages[i]);
            }
        }
    }
    println!("{}",dp[x]);
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
