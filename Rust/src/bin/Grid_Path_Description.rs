use std::io;

const N: usize = 7;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s: Vec<u8> = s.trim().bytes().collect();
    let mut res = 0;
    let mut visited = vec![vec![false; N + 2]; N + 2];
    for i in 0..=N + 1 {
        visited[0][i] = true;
        visited[i][0] = true;
        visited[N + 1][i] = true;
        visited[i][N + 1] = true;
    }
    dfs(&mut res, &mut visited, &s, 1, 1, 0);
    println!("{}", res);
}

fn dfs(res: &mut usize, visited: &mut Vec<Vec<bool>>, s: &Vec<u8>, x: usize, y: usize, i: usize) {
    if i == N * N - 1 || (x == N && y == 1) {
        if i == N * N - 1 && x == N && y == 1 {
            *res += 1;
        }
        return;
    }
    if visited[x - 1][y] && visited[x + 1][y] && !visited[x][y - 1] && !visited[x][y + 1] {
        return;
    }
    if visited[x][y - 1] && visited[x][y + 1] && !visited[x - 1][y] && !visited[x + 1][y] {
        return;
    }

    visited[x][y] = true;
    if s[i] == b'U' || s[i] == b'?' {
        if !visited[x - 1][y] {
            dfs(res, visited, s, x - 1, y, i + 1);
        }
    }
    if s[i] == b'D' || s[i] == b'?' {
        if !visited[x + 1][y] {
            dfs(res, visited, s, x + 1, y, i + 1);
        }
    }
    if s[i] == b'L' || s[i] == b'?' {
        if !visited[x][y - 1] {
            dfs(res, visited, s, x, y - 1, i + 1);
        }
    }
    if s[i] == b'R' || s[i] == b'?' {
        if !visited[x][y + 1] {
            dfs(res, visited, s, x, y + 1, i + 1);
        }
    }
    visited[x][y] = false;
}
