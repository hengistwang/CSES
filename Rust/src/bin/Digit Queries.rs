use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let n: usize = read_num();
    for _ in 0..n {
        fun();
    }
}

fn fun() {
    let mut n: usize = read_num();
    if n < 10 {
        println!("{}", n);
        return;
    }
    let mut base = 9;
    let mut digits = 1;
    while n > base * digits {
        n -= base * digits;
        base *= 10;
        digits += 1;
    }
    let index = n % digits;
    let mut res = 10_usize.pow((digits - 1) as u32) + (n - 1) / digits;
    if index != 0 {
        res = res / 10_usize.pow((digits - index) as u32);
    }
    println!("{}", res % 10);
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
