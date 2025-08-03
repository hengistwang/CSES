use std::io;
use std::{fmt::Debug, str::FromStr};
fn main() {
    let nmk: Vec<usize> = read_nums();
    let n = nmk[0];
    let m = nmk[1];
    let k = nmk[2] as i64;
    let mut ns: Vec<i64> = read_nums();
    let mut ms: Vec<i64> = read_nums();
    ns.sort_unstable();
    ms.sort_unstable();
    let mut i = 0;
    let mut j = 0;
    let mut res = 0;
    while i < n && j < m {
        let l = ns[i] - k;
        let r = ns[i] + k;
        if ms[j] < l {
            j += 1;
        } else if ms[j] > r {
            i += 1;
        } else {
            j += 1;
            i += 1;
            res += 1;
        }
    }
    println!("{}", res);
}

fn read_nums<T>() -> Vec<T>
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
