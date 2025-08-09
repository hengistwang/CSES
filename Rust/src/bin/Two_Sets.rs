use std::io;
use std::{fmt::Debug, str::FromStr};
fn main() {
    let n: i64 = read_num();
    let sum = (1 + n) * n / 2;
    if sum & 1 == 1 {
        println!("NO");
        return;
    }
    let mid = sum / 2;
    let mut l = Vec::new();
    let mut r = Vec::new();
    let mut left_sum = 0;
    for i in (1..=n).rev() {
        if left_sum + i <= mid {
            l.push(i);
            left_sum += i;
        } else {
            r.push(i);
        }
    }
    println!("YES");
    println!("{}", l.len());
    for i in l {
        print!("{} ", i);
    }
    println!("\n{}", r.len());
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
