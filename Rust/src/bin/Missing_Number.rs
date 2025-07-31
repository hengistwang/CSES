use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let n: usize = read_num();
    let mut xs: Vec<usize> = read_xs();
    xs.sort_unstable();
    for (i, x) in xs.iter().enumerate() {
        if *x != i + 1 {
            println!("{}", i + 1);
            return;
        }
    }
    println!("{}", n);
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
