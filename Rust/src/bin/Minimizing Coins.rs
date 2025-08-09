use std::io;
use std::{cmp::min, fmt::Debug, str::FromStr};
fn main() {
    let nx: Vec<usize> = read_nums();
    let x = nx[1];
    let coins: Vec<usize> = read_nums();
    let mut dp = vec![x + 1; x + 1];
    dp[0] = 0;
    for i in 1..=x {
        for c in coins.iter() {
            if i >= *c {
                dp[i] = min(dp[i], dp[i - c] + 1);
            }
        }
    }
    if dp[x] == x + 1 {
        println!("-1");
    } else {
        println!("{}", dp[x]);
    }
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
