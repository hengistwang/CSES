use std::ops::Bound::*;
use std::{collections::BTreeMap, fmt::Debug, io, str::FromStr};

fn main() {
    let nx: Vec<usize> = read_xs();
    let n = nx[0];
    let xs: Vec<usize> = read_xs();
    let mut btmap = BTreeMap::new();
    btmap.insert(n, 1);
    let mut btrange = BTreeMap::new();
    btrange.insert(0, n);
    for i in xs {
        let lr = btrange.range((Unbounded, Excluded(i))).next_back().unwrap();
        let l = *lr.0;
        let r = *lr.1;
        let len = r - l;
        btrange.remove(&l);
        btmap.entry(len).and_modify(|x| *x -= 1);
        if btmap[&len] == 0 {
            btmap.remove(&len);
        }

        btrange.insert(l, i);
        btrange.insert(i, r);
        let e = btmap.entry(i - l).or_insert(0);
        *e += 1;
        let e = btmap.entry(r - i).or_insert(0);
        *e += 1;
        println!("{}", btmap.last_entry().unwrap().key());
    }
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
