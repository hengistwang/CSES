use std::{cmp::min, io};

fn main() {
    let mut s1 = String::new();
    io::stdin().read_line(&mut s1).unwrap();
    let s1 = s1.trim().bytes().collect::<Vec<u8>>();
    let mut s2 = String::new();
    io::stdin().read_line(&mut s2).unwrap();
    let s2 = s2.trim().bytes().collect::<Vec<u8>>();
    let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
    for i in 0..s1.len() + 1 {
        dp[i][0] = i;
    }
    for i in 0..s2.len() + 1 {
        dp[0][i] = i;
    }
    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            let mut temp = 0;
            if s1[i - 1] != s2[j - 1] {
                temp = 1;
            }
            dp[i][j] = min(min(dp[i - 1][j], dp[i][j - 1]) + 1, dp[i - 1][j - 1] + temp);
        }
    }
    println!("{}", dp[s1.len()][s2.len()]);
}
