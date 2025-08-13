use std::collections::BTreeMap;
use std::{fmt::Debug, io, ops::Bound::*, str::FromStr};

fn main() {
    let _: usize = read_num();
    let xs: Vec<usize> = read_xs();
    let mut btmap: BTreeMap<usize, usize> = BTreeMap::new();
    for i in xs {
        let mut flag = false;
        let mut key = 0;
        match btmap.range_mut((Excluded(i), Unbounded)).next() {
            Some((k, v)) => {
                *v -= 1;
                if *v == 0 {
                    key = *k;
                    flag = true;
                }
            }
            None => {}
        }
        if flag {
            btmap.remove(&key);
        }
        let e = btmap.entry(i).or_insert(0);
        *e += 1;
    }
    let mut res = 0;
    for (_, v) in btmap {
        res += v;
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
