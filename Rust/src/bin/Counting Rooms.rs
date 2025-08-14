use std::io;
use std::{fmt::Debug, str::FromStr};
fn main() {
    let nm: Vec<usize> = read_nums();
    let n = nm[0];
    let m = nm[1];
    let mut rooms = Vec::with_capacity(n);
    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        rooms.push(s.into_bytes());
    }
    let mut res = 0;
    for i in 0..n {
        for j in 0..m {
            if rooms[i][j] == b'.' {
                res += 1;
                dfs(&mut rooms, i, j, n, m);
            }
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

fn dfs(rooms: &mut Vec<Vec<u8>>, i: usize, j: usize, n: usize, m: usize) {
    if rooms[i][j] == b'.' {
        rooms[i][j] = b'#';
        if i + 1 < n {
            dfs(rooms, i + 1, j, n, m);
        }
        if i - 1 < n {
            dfs(rooms, i - 1, j, n, m);
        }
        if j + 1 < m {
            dfs(rooms, i, j + 1, n, m);
        }
        if j - 1 < m {
            dfs(rooms, i, j - 1, n, m);
        }
    }
}
