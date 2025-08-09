use std::io;
use std::{collections::BTreeMap, fmt::Debug, str::FromStr};
fn main() {
    let nt: Vec<i64> = read_nums();
    let _n = nt[0];
    let t = nt[1];
    let xs: Vec<i64> = read_nums();
    let mut map = BTreeMap::new();
    for (i, val) in xs.iter().enumerate() {
        let key = t - val;
        if map.contains_key(&key) {
            println!("{} {}", map[&key] + 1, i + 1);
            return;
        } else {
            map.insert(val, i);
        }
    }
    println!("IMPOSSIBLE");
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
