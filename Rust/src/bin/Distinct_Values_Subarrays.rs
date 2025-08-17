use std::{collections::HashSet, fmt::Debug, io, str::FromStr};

fn main() {
    let _: usize = read_num();
    let xs: Vec<usize> = read_xs();
    let mut hs = HashSet::new();
    let mut j = 0;
    let mut res = 0;
    for (i, x) in xs.iter().enumerate() {
        while j < xs.len() && !hs.contains(&xs[j]) {
            hs.insert(xs[j]);
            j += 1;
        }
        res += j - i;
        hs.remove(&x);
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
    s.split_whitespace()
        .map(|x| x.trim().parse::<T>().unwrap())
        .collect()
}
