use std::io;
use std::{collections::HashSet, fmt::Debug, str::FromStr, vec};
fn main() {
    let n: usize = read_num();
    let mut board = vec![vec![0; n]; n];
    let mut col = vec![HashSet::new(); n];
    let mut row = vec![HashSet::new(); n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                board[i][j] = 0;
                row[i].insert(0);
                col[j].insert(0);
                continue;
            }
            let mut cur = 0;
            loop {
                if row[i].contains(&cur) || col[j].contains(&cur) {
                    cur += 1;
                } else {
                    break;
                }
            }
            board[i][j] = cur;
            row[i].insert(cur);
            col[j].insert(cur);
        }
    }
    for i in board {
        for j in i {
            print!("{} ", j);
        }
        println!();
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
