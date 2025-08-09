use std::cmp::max;
use std::io;
use std::{fmt::Debug, str::FromStr};
fn main() {
    let mut arrival: Vec<usize> = Vec::new();
    let mut leave: Vec<usize> = Vec::new();
    let n: usize = read_num();
    for _ in 0..n {
        let temp = read_nums();
        arrival.push(temp[0]);
        leave.push(temp[1]);
    }
    arrival.sort_unstable();
    leave.sort_unstable();
    let mut i = 0;
    let mut j = 0;
    let mut cur = 0;
    let mut res = 0;
    while i < arrival.len() && j < leave.len() {
        if arrival[i] < leave[j] {
            cur += 1;
            i += 1;
        } else {
            cur -= 1;
            j += 1;
        }
        res = max(res, cur);
    }
    println!("{}", res);
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
