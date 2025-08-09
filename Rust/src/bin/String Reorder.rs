use std::{collections::BTreeMap, io, u8};

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim().bytes().collect::<Vec<u8>>();
    let mut bt = BTreeMap::new();
    for &i in &s {
        let e = bt.entry(i).or_insert(0);
        *e += 1;
    }

    let mut pre = u8::MAX;
    let mut res = Vec::new();
    for _ in 0..s.len() {
        let mut flag = false;
        for i in 0..26 {
            let key = b'A' + i;
            if bt.contains_key(&key) && key != pre {
                {
                    let v = bt.get_mut(&key).unwrap();
                    *v -= 1;
                }
                if check(&bt, s.len() - res.len() - 1) {
                    pre = key;
                    flag = true;
                    res.push(key);
                    let v = bt.get_mut(&key).unwrap();
                    if *v == 0 {
                        bt.remove(&key);
                    }
                    break;
                } else {
                    let v = bt.get_mut(&key).unwrap();
                    *v += 1;
                }
            }
        }
        if !flag {
            println!("-1");
            return;
        }
    }
    for i in res {
        print!("{}", i as char);
    }
    println!();
}

fn check(bt: &BTreeMap<u8, usize>, n: usize) -> bool {
    for (_, v) in bt {
        if *v > (n + 1) / 2 {
            return false;
        }
    }
    return true;
}
