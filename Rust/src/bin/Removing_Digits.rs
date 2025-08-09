use std::io;
use std::{cmp::min, fmt::Debug, str::FromStr};
fn main() {
    let n: usize = read_num();
    let mut dp = vec![n + 1; n + 1];
    dp[0] = 0;
    for i in 1..=n {
        let mut temp = i;
        while temp != 0 {
            let d = temp % 10;
            temp = temp / 10;
            dp[i] = min(dp[i], dp[i - d] + 1);
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
