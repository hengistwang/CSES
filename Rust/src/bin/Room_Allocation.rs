use std::cmp::max;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Bound::*;
use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let n: usize = read_num();
    let mut leave: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    let mut rooms = 0;
    let mut res = Vec::new();
    let mut arrivals = Vec::new();
    for i in 0..n {
        let ab: Vec<usize> = read_xs();
        let a = ab[0];
        let b = ab[1];
        arrivals.push((a, b, i));
    }
    arrivals.sort_by_key(|x| x.0);
    for (a, b, i) in arrivals {
        let room;
        let mut del = 0;
        let flag;
        match leave.range((Unbounded, Excluded(a))).next() {
            Some((leavetime, room_number)) => {
                flag = true;
                del = *leavetime;
                room = *room_number.first().unwrap();
            }
            None => {
                flag = false;
                room = rooms + 1;
            }
        }
        let e = leave.entry(b).or_insert(BTreeSet::new());
        e.insert(room);
        if flag {
            let v = leave.get_mut(&del).unwrap();
            v.remove(&room);
            if v.is_empty() {
                leave.remove(&del);
            }
        }
        res.push((room, i));
        rooms = max(rooms, room);
    }
    println!("{}", rooms);
    res.sort_by_key(|x| x.1);
    for i in res {
        print!("{} ", i.0);
    }
    println!();
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
