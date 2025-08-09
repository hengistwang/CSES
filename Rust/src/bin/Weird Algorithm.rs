use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let mut n: usize = read_num();
    loop {
        print!("{} ", n);
        if n == 1 {
            break;
        }
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = n * 3 + 1;
        }
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
