use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let _: usize = read_num();
    let mut xs: Vec<usize> = read_xs();
    xs.sort_unstable();
    let largest = *xs.last().unwrap();
    let sum: usize = xs.iter().sum();
    let pre = sum - largest;
    if pre < largest {
        println!("{}", largest * 2);
    } else {
        println!("{}", sum);
    }
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
