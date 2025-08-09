use std::io;
use std::{fmt::Debug, str::FromStr};
const M: usize = (1e9 as usize) + 7;
fn main() {
    let n: usize = read_num();
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for i in 1..=n {
        for j in 1..=6 {
            if i >= j {
                dp[i] = (dp[i] + dp[i - j]) % M;
            }
        }
    }
    println!("{}", dp[n]);
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
