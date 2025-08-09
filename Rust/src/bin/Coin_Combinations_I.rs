use std::io;
use std::{fmt::Debug, str::FromStr};
const M: usize = (1e9 as usize) + 7;
fn main() {
    let nx: Vec<usize> = read_nums();
    let n = nx[1];
    let coins: Vec<usize> = read_nums();
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for i in 1..=n {
        for j in coins.iter() {
            if i >= *j {
                dp[i] = (dp[i] + dp[i - j]) % M;
            }
        }
    }
    println!("{}", dp[n]);
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
