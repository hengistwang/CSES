use std::io;
use std::ops::Bound::{Included, Unbounded};
use std::{collections::BTreeMap, fmt::Debug, str::FromStr};
fn main() {
    let _nm: Vec<usize> = read_nums();
    let temp: Vec<usize> = read_nums();
    let mut prices = BTreeMap::new();
    for i in temp {
        let e = prices.entry(i).or_insert(0);
        *e += 1;
    }
    let ms: Vec<usize> = read_nums();
    for i in ms {
        let mut key = 0;
        let mut flag = false;
        let upper = prices.range_mut((Unbounded, Included(i))).next_back();
        match upper {
            Some((price, count)) => {
                print!("{} ", price);
                *count -= 1;
                if *count == 0 {
                    flag = true;
                    key = *price;
                }
            }
            None => print!("-1 "),
        }
        if flag {
            prices.remove(&key);
        }
    }
    println!();
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
