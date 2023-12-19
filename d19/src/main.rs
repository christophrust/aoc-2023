use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;



fn main() {

    let mut mp = HashMap::<String, Vec<(String, Option<(char,char,i64)>)>>::new();
    let re1 = Regex::new(r"([a-z]+)\{(.*)\}")
        .unwrap();
    let re2 = Regex::new(r"([a-zAR]+)([><]*)([0-9]*)[:]*([a-zRA]*)")
        .unwrap();
    let re3 = Regex::new(r"\{x=([0-9]*),m=([0-9]*),a=([0-9]*),s=([0-9]*)\}")
        .unwrap();

    let mut chk = false;

    let file = File::open("input.txt").unwrap();
    let res = io::BufReader::new(&file)
        .lines()
        .map(|x| {
            let x = x.unwrap();
            if &x == "" {
                // println!("{:?}", mp);
                chk = true;
            } else if !chk {
                let (_, [nm, rules]) = re1.captures(&x).map(|x| x.extract()).unwrap();
                let r = rules.split(',').map(|s| {
                    let (_,[l,cmp,n,t]) = re2.captures(&s).map(|x| x.extract()).expect(&format!("{}",s));
                    match cmp {
                        "" => (l.to_owned(), None),
                        _ => (t.to_owned(), Some((l.chars().nth(0).unwrap(),cmp.chars().nth(0).unwrap(), n.parse().unwrap())))
                    }
                }).collect();
                mp.insert(nm.to_owned(), r);
            } else {

                let (_,[x,m,a,s]) = re3.captures(&x).map(|x| x.extract()).expect(&format!("{}",x));
                let x = x.parse::<i64>().unwrap();
                let m = m.parse::<i64>().unwrap();
                let a = a.parse::<i64>().unwrap();
                let s = s.parse::<i64>().unwrap();
                let mut nm = "in";
                loop {
                    for rule in mp.get(nm).expect(&format!("{}", nm)) {
                        match rule {
                            (t, None) => {
                                if t == "A" {
                                    return x+m+a+s
                                }
                                if t == "R" {
                                    return 0
                                }
                                nm = t;
                                break;
                            }
                            (t, Some((f, cmp, n))) => {
                                let cmpf = match &f {
                                    'x' => x,
                                    'm' => m,
                                    'a' => a,
                                    _ => s,
                                };
                                if *cmp == '<' {
                                    if cmpf < *n {
                                        if t == "R" {
                                            return 0
                                        }
                                        if t == "A" {
                                            return x+m+a+s
                                        }
                                        nm = t;
                                        break;
                                    }
                                } else {
                                    if cmpf > *n {
                                        if t == "R" {
                                            return 0
                                        }
                                        if t == "A" {
                                            return x+m+a+s
                                        }
                                        nm = t;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            0_i64
        })
        .sum::<i64>();

    println!("Part 1: {}", res);


    let leafs = walk(&mp, "in", [(1, 4001);4]);

    let res = leafs
        .into_iter()
        .map(|x| {
            x.iter().fold(1, |a,i| a * (i.1 - i.0).max(0))
        })
        .sum::<i64>();
    println!("Part 2: {}", res);
}


fn walk(
    m: &HashMap<String, Vec<(String, Option<(char,char,i64)>)>>,
    entry: &str,
    reg: [(i64,i64);4],
) -> Vec<[(i64,i64);4]>
{
    let mut res = Vec::<[(i64,i64);4]>::new();
    let mut rg = reg;
    for cur in  m.get(entry).expect(&format!("{entry}")) {

        match &cur {
            (t, None) => {
                if t == "A" {
                    res.push(rg);
                } else if t == "R" {
                } else {
                    res.append(&mut walk(m, t, rg));
                }
            },
            (t, Some((f, cmp, n))) => {
                let i = match f {
                    'x' => 0,
                    'm' => 1,
                    'a' => 2,
                    's' => 3,
                    _ => unreachable!(),
                };
                match cmp {
                    '<' => {
                        let tmp = rg[i].1;
                        rg[i].1 = *n;
                        if t == "A" {
                            res.push(rg);
                        } else if t == "R" {
                        } else {
                            res.append(&mut walk(m, t, rg));
                        }
                        rg[i].1 = tmp;
                        rg[i].0 = *n;
                    },
                    '>' => {
                        let tmp = rg[i].0;
                        rg[i].0 = *n + 1;
                        if t == "A" {
                            res.push(rg);
                        } else if t == "R" {
                        } else {
                            res.append(&mut walk(m, t, rg));
                        }
                        rg[i].0 = tmp;
                        rg[i].1 = *n + 1;
                    },
                    _ => unreachable!(),
                }
            }
        }
    }
    res
}
