use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let n: i64 = read_num();
    for i in 1..=n {
        fun(i);
    }
}

fn fun(n: i64) {
    let res = (n * n * ((n * n) - 1)) / 2 - 4 * (n - 1) * (n - 2);
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
