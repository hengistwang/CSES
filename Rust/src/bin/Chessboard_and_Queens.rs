use std::io;
fn main() {
    let mut board: Vec<Vec<char>> = Vec::new();
    for _ in 0..8 {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        board.push(s.trim().chars().collect());
    }
    let mut col = vec![false; 32];
    let mut diag1 = vec![false; 32];
    let mut diag2 = vec![false; 32];
    let mut res = 0;
    search(0, &board, &mut col, &mut diag1, &mut diag2, &mut res);
    println!("{}", res);
}

fn search(
    x: usize,
    board: &Vec<Vec<char>>,
    col: &mut Vec<bool>,
    diag1: &mut Vec<bool>,
    diag2: &mut Vec<bool>,
    res: &mut u64,
) {
    if x == 8 {
        *res += 1;
        return;
    }
    for y in 0..8 {
        if col[y] || diag1[x + y] || diag2[x + 8 - y] || board[x][y] == '*' {
            continue;
        }
        col[y] = true;
        diag1[x + y] = true;
        diag2[x + 8 - y] = true;
        search(x + 1, board, col, diag1, diag2, res);
        col[y] = false;
        diag1[x + y] = false;
        diag2[x + 8 - y] = false;
    }
}
