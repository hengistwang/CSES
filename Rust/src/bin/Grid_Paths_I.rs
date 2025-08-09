use std::io;
use std::{fmt::Debug, str::FromStr};
fn main() {
    let n: usize = read_num();
    let mut xs: Vec<Vec<char>> = Vec::with_capacity(n);
    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s: Vec<char> = s.chars().collect();
        xs.push(s);
    }
    let mut dp = vec![vec![0; n]; n];
    for i in 0..n {
        if xs[i][0] == '*' {
            break;
        }
        dp[i][0] = 1;
    }
    for i in 0..n {
        if xs[0][i] == '*' {
            break;
        }
        dp[0][i] = 1;
    }
    let m = 1e9 as i64 + 7;
    for i in 1..n {
        for j in 1..n {
            if xs[i][j] == '*' {
                continue;
            } else {
                dp[i][j] = (dp[i - 1][j] + dp[i][j - 1]) % m;
            }
        }
    }
    println!("{}", dp[n - 1][n - 1]);
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
