use std::{cmp::max, io};

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim().as_bytes();
    let mut pre = s[0];
    let mut res = 0;
    let mut counter = 0;
    for i in s {
        if *i == pre {
            counter += 1;
        } else {
            res = max(res, counter);
            pre = *i;
            counter = 1;
        }
    }
    res = max(res, counter);
    println!("{}", res);
}
