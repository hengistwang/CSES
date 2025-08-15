use std::{
    fmt::Debug,
    io,
    mem::{self},
    str::FromStr,
};

fn main() {
    let nm: Vec<usize> = read_xs();
    let n = nm[0];
    let m = nm[1];
    let mut xs: Vec<usize> = read_xs();
    let mut index = vec![0; n + 2];
    for (i, val) in xs.iter().enumerate() {
        index[*val] = i + 1;
    }
    index[n + 1] = n + 1;
    let mut res = 1;
    for i in 1..=n {
        if index[i - 1] > index[i] {
            res += 1;
        }
    }
    for _ in 0..m {
        let ab: Vec<usize> = read_xs();
        let a = ab[0];
        let b = ab[1];
        let mut ia = xs[a - 1];
        let mut ib = xs[b - 1];
        xs.swap(a - 1, b - 1);

        if ia > ib {
            mem::swap(&mut ia, &mut ib);
        }
        update(&mut res, &mut index, ia, ib);
        println!("{}", res);
    }
}

fn update(res: &mut i32, index: &mut Vec<usize>, ia: usize, ib: usize) {
    let mut before = 0;
    if index[ia - 1] > index[ia] {
        before += 1;
    }
    if index[ia + 1] < index[ia] {
        before += 1;
    }
    if ia + 1 != ib {
        if index[ib - 1] > index[ib] {
            before += 1;
        }
    }
    if index[ib + 1] < index[ib] {
        before += 1;
    }
    index.swap(ia, ib);
    let mut cur = 0;
    if index[ia - 1] > index[ia] {
        cur += 1;
    }
    if index[ia + 1] < index[ia] {
        cur += 1;
    }
    if ia + 1 != ib {
        if index[ib - 1] > index[ib] {
            cur += 1;
        }
    }
    if index[ib + 1] < index[ib] {
        cur += 1;
    }
    if before >= cur {
        *res -= before - cur;
    } else {
        *res += cur - before;
    }
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
