use std::collections::VecDeque;
use std::{fmt::Debug, str::FromStr, vec};
use std::{io, u32};
fn main() {
    let n: usize = read_num();
    let mut board = vec![vec![u32::MAX; n]; n];
    board[0][0] = 0;
    let mut q = VecDeque::new();
    q.push_back((0, 0));
    while !q.is_empty() {
        let (i, j) = q.pop_front().unwrap();
        let cur = board[i][j] + 1;
        if j >= 2 {
            if i >= 1 {
                if board[i - 1][j - 2] == u32::MAX {
                    board[i - 1][j - 2] = cur;
                    q.push_back((i - 1, j - 2));
                }
            }
            if i + 1 < n {
                if board[i + 1][j - 2] == u32::MAX {
                    board[i + 1][j - 2] = cur;
                    q.push_back((i + 1, j - 2));
                }
            }
        }
        if j + 2 < n {
            if i >= 1 {
                if board[i - 1][j + 2] == u32::MAX {
                    board[i - 1][j + 2] = cur;
                    q.push_back((i - 1, j + 2));
                }
            }
            if i + 1 < n {
                if board[i + 1][j + 2] == u32::MAX {
                    board[i + 1][j + 2] = cur;
                    q.push_back((i + 1, j + 2));
                }
            }
        }
        if i >= 2 {
            if j >= 1 {
                if board[i - 2][j - 1] == u32::MAX {
                    board[i - 2][j - 1] = cur;
                    q.push_back((i - 2, j - 1));
                }
            }
            if j + 1 < n {
                if board[i - 2][j + 1] == u32::MAX {
                    board[i - 2][j + 1] = cur;
                    q.push_back((i - 2, j + 1));
                }
            }
        }
        if i + 2 < n {
            if j >= 1 {
                if board[i + 2][j - 1] == u32::MAX {
                    board[i + 2][j - 1] = cur;
                    q.push_back((i + 2, j - 1));
                }
            }
            if j + 1 < n {
                if board[i + 2][j + 1] == u32::MAX {
                    board[i + 2][j + 1] = cur;
                    q.push_back((i + 2, j + 1));
                }
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            print!("{} ", board[i][j]);
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
