use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;


fn main() {

    let re = Regex::new(r"([AKQJT2-9]{5}) ([0-9]+)").unwrap();
    let file = File::open("input.txt").unwrap();

    let mut hands: Vec<([usize;6], u32, String)> = io::BufReader::new(&file)
        .lines()
        .filter_map(|x| {
            let x = x.unwrap();
            if let Some((_,[h, w])) = re.captures(&x).map(|x| x.extract()) {
                Some((score_hand(h), w.parse::<u32>().unwrap(), h.to_string()))
            } else {
            None
            }
        })
        .collect();
    hands.sort_by_key(|x| x.0);
    let mut cnt = 1;

    let res = hands.into_iter().fold(0, |mut c,(s,b,h)| {
        println!("h: {}, b: {}, s: {:?}, rank: {}", h, b, s, cnt);
        c += b * cnt;
        cnt += 1;
        c
    });


    println!("Part 1: {:?}", res);
}


fn card_value(c: &char) -> usize {
    match &&c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        x => x.to_digit(10).unwrap() as usize,
    }
}



fn collect_hand(h: &str) -> usize {
    let mut hm = h
        .chars()
        .fold(HashMap::<char, usize>::new(), |mut m, c| {
            if let Some(v) = m.get_mut(&c) {
                *v += 1;
            } else {
                m.insert(c,1);
            }
            m
        });
    let nj = match hm.remove(&'J') {
        Some(x) => x,
        None => 0,
    };
    let mut m = hm
        .iter()
        .map(|(_,&v)| {
            v
        })
        .collect::<Vec<usize>>();

    m.sort_unstable_by(|a, b| b.cmp(&a));
    if m.len() > 0{
        m[0] += nj;
    } else {
        m = vec![nj];
    }
    match m[0] {
        5 => 7,
        4 =>  6,
        3 =>  match m[1] {
                2 =>  5,
                _ =>  4,
            },
        2 => match m[1] {
            2 =>  3,
            _ =>  2,
        },
        _ => 1,
    }
}

fn score_hand(h: &str) -> [usize; 6] {
    let mut res = [0;6];

    res[0] = collect_hand(h);
    let mut i = 1;
    h.chars()
     .for_each(|x| {
         res[i] = card_value(&x);
         i += 1;
     });

    return res;
}
