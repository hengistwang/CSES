use std::{collections::BTreeSet, io, vec};
fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s: Vec<char> = s.trim().chars().collect();
    let mut selected: Vec<bool> = vec![false; s.len()];
    let mut res = BTreeSet::new();
    let mut permutation = Vec::new();
    permut(&s, &mut selected, &mut res, &mut permutation);
    println!("{}", res.len());
    for i in res {
        println!("{}", i);
    }
}

fn permut(
    xs: &Vec<char>,
    selected: &mut Vec<bool>,
    res: &mut BTreeSet<String>,
    permutation: &mut Vec<char>,
) {
    if permutation.len() == xs.len() {
        res.insert(permutation.iter().collect::<String>());
        return;
    }
    for (i, val) in xs.iter().enumerate() {
        if selected[i] == true {
            continue;
        }
        selected[i] = true;
        permutation.push(*val);
        permut(xs, selected, res, permutation);
        permutation.pop();
        selected[i] = false;
    }
}
