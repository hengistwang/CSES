use std::{cmp::max, collections::HashSet, fmt::Debug, io, str::FromStr};

fn main() {
    let _: usize = read_num();
    let xs: Vec<usize> = read_xs();
    let mut st = HashSet::new();
    let mut j = 0;
    let mut res = 0;
    for i in 0..xs.len() {
        while j < xs.len() && st.insert(xs[j]) {
            j += 1;
        }
        res = max(res, st.len());
        st.remove(&xs[i]);
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
