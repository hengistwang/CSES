use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let nt: Vec<usize> = read_xs();
    let t = nt[1];
    let mut xs: Vec<usize> = read_xs();
    xs.sort_unstable();
    let mut right = t * xs.last().unwrap();
    let mut left = 0;
    while left <= right {
        let mid = left + (right - left) / 2;
        if check(&xs, t, mid) == true {
            if check(&xs, t, mid - 1) == false {
                println!("{}", mid);
                return;
            } else {
                right = mid - 1;
            }
        } else {
            left = mid + 1;
        }
    }
}
fn check(xs: &Vec<usize>, t: usize, n: usize) -> bool {
    let mut sum = 0;
    for i in xs {
        sum += n / i;
    }
    sum >= t
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
