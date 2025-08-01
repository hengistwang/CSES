use std::io;
use std::{fmt::Debug, str::FromStr};
fn main() {
    let n = read_num();
    for _ in 0..n {
        solve();
    }
}
fn solve() {
    let xs: Vec<u64> = read_nums();
    let mut n = xs[0];
    let a = xs[1];
    let b = xs[2];
    let total = a + b;
    if total > n {
        println!("NO");
        return;
    }
    if (a != 0 && b == 0) || (a == 0 && b != 0) {
        println!("NO");
        return;
    }
    let mut l = Vec::new();
    let mut r = Vec::new();
    for _ in 0..n - total {
        l.push(n);
        r.push(n);
        n -= 1;
    }
    for i in 1..=total {
        let mut cur = (i + b) % total;
        if cur == 0 {
            cur = total;
        }
        l.push(cur);
        r.push(i);
    }
    println!("YES");
    for i in l {
        print!("{} ", i);
    }
    println!();
    for i in r {
        print!("{} ", i);
    }
    println!();
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
