use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let n: usize = read_num();
    if n == 2 || n == 3 {
        println!("NO SOLUTION");
        return;
    }
    let mut cur = 2;
    while cur <= n {
        print!("{} ", cur);
        cur += 2;
    }
    cur = 1;
    while cur <= n {
        print!("{} ", cur);
        cur += 2;
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
