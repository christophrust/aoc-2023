use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};


fn main() {
    let file = File::open("input.txt").unwrap();

    let mut cnt = -1_i64;
    let mut r = HashMap::<i64, i64>::new();
    let mut c = HashMap::<i64, i64>::new();

    let idcs = io::BufReader::new(&file)
        .lines()
        .map(|x| {
            cnt += 1;
            x
                .unwrap()
                .char_indices()
                .filter_map(|x| match x.1 {
                    '#' => {
                        if r.get(&cnt).is_none() {
                            r.insert(cnt, cnt);
                        }
                        if c.get(&(x.0 as i64)).is_none() {
                            c.insert(x.0 as i64, x.0 as i64);
                        }
                        Some((x.0 as i64, cnt))
                    },
                    _ => None,
                })
                .collect::<Vec<(i64, i64)>>()
        })
        .flatten()
        .collect::<Vec<(i64, i64)>>();

    let nr = r.iter().map(|(&k,_)| k).reduce(|a,b| if a < b {b} else {a}).unwrap();
    let nc = c.iter().map(|(&k,_)| k).reduce(|a,b| if a < b {b} else {a}).unwrap();


    let mut rs = 0_i64;
    let mut cs = 0_i64;
    for i in 0..(nr+1) {
        if let Some(v) = r.get_mut(&i) {
            *v += rs;
        } else {
            rs += 1;
        }
    }


    for j in 0..(nc+1) {
        if let Some(v) = c.get_mut(&j) {
            *v += cs;
        } else {
            cs += 1;
        }
    }

    let mut res = 0_i64;
    for i in 0..idcs.len() {
        for j in (i+1)..idcs.len() {
            let c1 = c.get(&idcs[i].0).unwrap();
            let r1 = r.get(&idcs[i].1).unwrap();
            let c2 = c.get(&idcs[j].0).unwrap();
            let r2 = r.get(&idcs[j].1).unwrap();
            res += (c1-c2).abs() + (r1-r2).abs();
        }
    }

    println!("Part 1: {}", res);

    let mut rs = 0_i64;
    let mut cs = 0_i64;
    for i in 0..(nr+1) {
        if let Some(v) = r.get_mut(&i) {
            *v += rs;
        } else {
            rs += 999998;
        }
    }


    for j in 0..(nc+1) {
        if let Some(v) = c.get_mut(&j) {
            *v += cs;
        } else {
            cs += 999998;
        }
    }

    let mut res = 0_i64;
    for i in 0..idcs.len() {
        for j in (i+1)..idcs.len() {
            let c1 = c.get(&idcs[i].0).unwrap();
            let r1 = r.get(&idcs[i].1).unwrap();
            let c2 = c.get(&idcs[j].0).unwrap();
            let r2 = r.get(&idcs[j].1).unwrap();
            res += (c1-c2).abs() + (r1-r2).abs();
        }
    }

    println!("Part 2: {}", res);
}
