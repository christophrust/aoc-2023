use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use prime_factorization::Factorization;

fn main() {

    let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
    let file = File::open("input.txt").unwrap();
    let mut lines = io::BufReader::new(&file)
        .lines();

    let directions:Vec<char> = lines.next().unwrap().unwrap().chars().collect();

    lines.next();


    let m = lines
        .into_iter()
        .map(|x| {
            let x = x.unwrap();
            let (_,[n,l,r]) = re.captures(&x).map(|x| x.extract()).unwrap();
            let  x = (n.to_owned(),(l.to_owned(),r.to_owned()));
            x
        })
        .collect::<HashMap<String,(String,String)>>();

    let mut cnt = 0;
    let mut s = "AAA".to_string();
    loop {
        for d in directions.iter() {
            cnt += 1;
            match d {
                'R' => {
                    s = m.get(&s).unwrap().1.clone();
                },
                _ =>{
                    s = m.get(&s).unwrap().0.clone();
                },
            }
            if s == "ZZZ".to_string() {
                break;
            }
        }
        if s == "ZZZ".to_string() {
            break;
        }
    }
    println!("Part 1: {}", cnt);



    let curr_nodes: Vec<String> = m.iter().filter_map(|(n,_)| {
        if n.chars().nth(2).unwrap() == 'Z' {
            Some(n.clone())
        } else {
            None
        }
    })
        .collect();

    let steps: Vec<Vec<u64>> = curr_nodes
        .into_iter()
        .map(|mut x| {
            let mut cnt = 0;
            loop {
                for d in directions.iter() {
                    cnt += 1;
                    match d {
                        'R' => {
                            x = m.get(&x).unwrap().1.clone();
                        },
                        _ =>{
                            x = m.get(&x).unwrap().0.clone();
                        },
                    }
                    if x.chars().nth(2).unwrap() == 'Z' {
                        break;
                    }
                }
                if x.chars().nth(2).unwrap() == 'Z' {
                    break;
                }
            }
            Factorization::<u64>::run(cnt).factors
        })
        .collect();

    let res: u64 = steps
        .into_iter()
        .fold(Vec::<u64>::new(), |mut a, mut i| {
            while let Some(last) = i.pop() {
                if !a.contains(&last) {
                    a.push(last);
                }
            }
            //a.sort_unstable();
            a
        })
        .into_iter()
        .product();


    println!("Part 2: {:?}", res);
}
