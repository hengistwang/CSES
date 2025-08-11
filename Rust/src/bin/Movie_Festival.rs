use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let n: usize = read_num();
    let mut timeline = Vec::new();
    for _ in 0..n {
        let temp: Vec<usize> = read_xs();
        timeline.push((temp[0], temp[1]));
    }
    timeline.sort_unstable_by(|a, b| {
        if a.1 == b.1 {
            return a.0.cmp(&b.0);
        } else {
            return a.1.cmp(&b.1);
        }
    });
    let mut cur = 0;
    let mut res = 0;
    for (start, end) in timeline {
        if start >= cur {
            res += 1;
            cur = end;
        }
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
