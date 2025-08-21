use std::{collections::VecDeque, fmt::Debug, io, str::FromStr};

fn main() {
    let _: usize = read_num();
    let nx: Vec<usize> = read_xs();
    let mut st = VecDeque::new();
    for (i, v) in nx.iter().enumerate() {
        while !st.is_empty() && nx[*st.back().unwrap()] >= *v {
            st.pop_back();
        }
        if st.is_empty() {
            print!("0 ");
        } else {
            print!("{} ", *st.back().unwrap() + 1);
        }

        st.push_back(i);
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
