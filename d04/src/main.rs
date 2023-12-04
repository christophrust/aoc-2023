use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashMap;

fn main() {

    let re = Regex::new(r"[0-9]+").unwrap();

    let file = File::open("input.txt").unwrap();
    let res : u32 = io::BufReader::new(&file)
        .lines()
        .filter_map(|x| {
            let x = x.ok()?;
            let mut s = x.split(':');
            s.next();
            let x = s.next().unwrap();
            let mut s = x.split('|');
            let l = s.next().unwrap();
            let r = s.next().unwrap();

            let ll : HashMap<&str,()> = re
                .find_iter(l)
                .map(|x|
                     (
                         x.as_str(),
                         ()
                     )
                )
                .collect();
            let pw = re.find_iter(r).filter(|x| ll.get(&x.as_str()).is_some()).count();
            if pw == 0 {
                return Some(0)
            }
            Some(2_u32.pow(pw as u32 - 1))
        })
        .sum();
    println!("Part 1: {}", res);

    // part2
    let file = File::open("input.txt").unwrap();
    let mut i = 0_usize;
    let res = io::BufReader::new(&file)
        .lines()
        .filter_map(|x| {
            let x = x.ok()?;
            let mut s = x.split(':');
            s.next();
            let x = s.next().unwrap();
            let mut s = x.split('|');
            let l = s.next().unwrap();
            let r = s.next().unwrap();

            let ll : HashMap<&str,()> = re
                .find_iter(l)
                .map(|x|
                     (
                         x.as_str(),
                         ()
                     )
                )
                .collect();
            let pw = re.find_iter(r).filter(|x| ll.get(&x.as_str()).is_some()).count();
            if pw == 0 {
                return Some(0)
            }
            Some(pw as u32)
        })
        .fold((0_u32, vec![1_u32;200]), |mut accum, item| {
            if item > 0 {
                for j in 0..item as usize {
                    accum.1[j + i + 1] += 1 * accum.1[i];
                }
            }
            accum.0 += accum.1[i];
            i += 1;
            accum
        });

    println!("Part 2: {}", res.0);
}
