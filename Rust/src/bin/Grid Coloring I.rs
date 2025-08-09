use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let xs: Vec<usize> = read_xs();
    let cs = [['A', 'B'], ['C', 'D']];
    let n = xs[0];
    let m = xs[1];
    for i in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s: Vec<char> = s.trim().chars().collect();
        for j in 0..m {
            if s[j] < 'C' {
                print!("{}", cs[1][(i + j) & 1]);
            } else {
                print!("{}", cs[0][(i + j) & 1]);
            }
        }
        println!();
    }
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
