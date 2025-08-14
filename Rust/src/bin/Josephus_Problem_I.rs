use std::{collections::VecDeque, fmt::Debug, io, str::FromStr};

fn main() {
    let n: usize = read_num();
    let mut q = VecDeque::new();
    for i in 1..=n {
        q.push_back(i);
    }
    while !q.is_empty() {
        let front = q.front().unwrap().clone();
        q.pop_front();
        q.push_back(front);
        let front = q.front().unwrap().clone();
        q.pop_front();
        print!("{} ", front);
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
