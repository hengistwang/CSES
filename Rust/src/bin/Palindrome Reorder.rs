use std::{collections::BTreeMap, io};
fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim();
    let mut btmap = BTreeMap::new();
    for i in s.chars() {
        let e = btmap.entry(i).or_insert(0);
        *e += 1;
    }
    let mut flag = false;
    let mut center = 'a';
    for (k, v) in &mut btmap {
        if *v & 1 == 1 {
            if flag == false {
                flag = true;
                center = *k;
                *v -= 1;
            } else {
                println!("NO SOLUTION");
                return;
            }
        }
    }
    let mut res = String::new();
    for (k, v) in btmap {
        for _ in 0..(v / 2) {
            res.push(k);
        }
    }
    let right = res.clone().chars().rev().collect::<String>();
    if flag {
        res.push(center);
    }
    res.push_str(&right);
    println!("{}", res);
}
