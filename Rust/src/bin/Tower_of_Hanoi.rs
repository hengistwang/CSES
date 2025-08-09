use std::io;
use std::{fmt::Debug, str::FromStr};
fn main() {
    let n = read_num();
    let mut res = Vec::new();
    fun(n, 1, 3, 2, &mut res);
    println!("{}", res.len());
    for (i, j) in res {
        println!("{} {}", i, j);
    }
}
fn fun(n: u64, src: u64, dst: u64, aux: u64, res: &mut Vec<(u64, u64)>) {
    if n == 1 {
        res.push((src, dst));
        return;
    }
    fun(n - 1, src, aux, dst, res);
    res.push((src, dst));
    fun(n - 1, aux, dst, src, res);
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
