use std::collections::HashSet;
use std::io;
fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.clear();
    io::stdin().read_line(&mut s).unwrap();
    let st = s
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.trim().parse::<u64>().unwrap())
        .collect::<HashSet<u64>>();
    println!("{}", st.len());
}
