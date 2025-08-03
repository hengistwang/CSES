use std::collections::BTreeSet;
use std::io;
use std::ops::Bound::*;
use std::{fmt::Debug, str::FromStr};
fn main() {
    let _n: u64 = read_num();
    let xs: Vec<u64> = read_nums();
    let mut bs = BTreeSet::new();
    for i in xs {
        let mut flag = false;
        let mut target = 0;
        match bs.range((Unbounded, Included(i))).next_back() {
            Some(val) => {
                if val + 1 == i {
                    target = *val;
                    flag = true;
                }
            }
            None => {}
        }
        bs.insert(i);
        if flag {
            bs.remove(&target);
        }
    }
    println!("{}", bs.len());
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
