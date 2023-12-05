use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    let re = Regex::new(r"[0-9]+").unwrap();
    let re2 = Regex::new(r"([0-9]+) ([0-9]+) ([0-9]+)").unwrap();

    let file = File::open("input1.txt").unwrap();
    let mut bf = io::BufReader::new(&file).lines();

    let seeds: Vec<i64> = re
        .find_iter(
            &bf.next().unwrap().unwrap()
        )
        .filter_map(|x| x.as_str().parse::<i64>().ok())
        .collect();

    let mut maps = Vec::<Vec<(i64,i64,i64)>>::new();
    while let Some(Ok(l)) = bf.next() {
        if l == "" {
            maps.push(Vec::<(i64,i64,i64)>::new())
        }
        if let Some((_,[a,b,c])) = re2.captures(&l).map(|x| x.extract()) {
            let l = maps.last_mut().unwrap();

            l.push((
                a.parse::<i64>().unwrap(),
                b.parse::<i64>().unwrap(),
                c.parse::<i64>().unwrap()
            ));
        };
    }


    let res = seeds
        .iter()
        .fold(-1, |mut accum, &x| {
            let mut res = x;
            for m in maps.iter() {
                for (d, s, r) in m {
                    if res >= *s && res <= *s + *r {
                        res = d + res - s;
                        break;
                    }
                }
                // println!("{}", res);
            }
            // println!("-----------");
            if res < accum || accum == -1 {
                accum = res;
            }
            accum
        });
    println!("Part 1: {:?}", res);

    let res = seeds
        .chunks(2)
        .fold(-1, |mut accum,x| {

            let res = map_min_pos(x[0], x[1], &maps, 0);
            // print!("->{}\n", res);
            if res < accum || accum == -1 {
                accum = res;
            }

            accum
        });


    println!("Part 2: {:?}", res);

}


fn map_min_pos(s: i64, r: i64, mi: &Vec<Vec<(i64,i64,i64)>>, depth: usize) -> i64 {
    println!("Called with {}, {}, {}", s, r, depth);
    let mut res = -1;
    let mut c = Some((s,r));
    let mut dr = 0;
    let mut changed = false;

    while let Some((mut so, mut ro)) = c {

        for (d,s,r) in mi[depth].iter() {
            if so >= *s && so <= *s + *r {
                dr =  so + ro - *s - *r;
                if dr < 0 {
                    c = None;
                } else {
                    ro -= dr;
                    so += dr;
                    c = Some((so, ro));
                }
                so = *d + so - *s;
                changed = true;
                break;
            }
        }
        if !changed {

        }
        if depth + 1 < mi.len() {
            so = map_min_pos(so, ro, mi, depth + 1);
        }
        if so < res || res == -1 {
            res = so;
        }
    }



    return res;
}
