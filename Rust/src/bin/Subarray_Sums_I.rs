use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let nx: Vec<usize> = read_xs();
    let target = nx[1];
    let xs: Vec<usize> = read_xs();
    let mut start = 0;
    let mut end = 0;
    let mut res = 0;
    let mut sumvalue = 0;
    while end < xs.len() {
        while end < xs.len() && sumvalue < target {
            sumvalue += xs[end];
            end += 1;
        }
        while sumvalue > target && start <= end {
            sumvalue -= xs[start];
            start += 1;
        }
        if sumvalue == target {
            res += 1;
            sumvalue -= xs[start];
            start += 1;
        }
    }
    println!("{}", res);
}

fn read_xs<T>() -> Vec<T>
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
