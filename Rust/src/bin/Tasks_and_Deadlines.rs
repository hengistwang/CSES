use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let n: i64 = read_num();
    let mut xs = Vec::new();
    for _ in 0..n {
        let td: Vec<i64> = read_xs();
        let t = td[0];
        let d = td[1];
        xs.push((t, d));
    }
    xs.sort_by_key(|x| x.0);
    let mut res = 0;
    let mut cur = 0;
    for (t, d) in xs {
        cur += t;
        res += d - cur;
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
