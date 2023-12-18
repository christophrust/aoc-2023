use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {

    let mut hm = HashSet::<(i64, i64)>::new();
    let mut rmin = 0;
    let mut cmin = 0;
    let mut rmax = 0;
    let mut cmax = 0;

    let file = File::open("input.txt").unwrap();
    let poly = io::BufReader::new(&file)
        .lines()
        .map(|x| {
            let x = x.unwrap();
            let mut x = x.split(' ');
            let d = match x.next().unwrap() {
                "R" => (0,1),
                "L" => (0,-1),
                "U" => (-1,0),
                "D" => (1,0),
                _ => unreachable!(),
            };
            let t = x.next().unwrap().parse().unwrap();
            (d,t)
        })
        .fold(vec![(0,0)], |mut a, ((d1,d2),t)| {
            let (l1,l2) = a.last().unwrap();
            for i in 1..=t {
                hm.insert((*l1 + i* d1, *l2 + i* d2));
            }
            cmin = cmin.min(*l2 + t * d2);
            cmax = cmax.max(*l2 + t * d2);
            rmin = rmin.min(*l1 + t* d1);
            rmax = rmax.max(*l1 + t* d1);
            a.push((*l1 + t* d1, *l2 + t * d2));
            a
        });
    for i in cmin..=cmax {
        for j in rmin..=rmax {
            if pt_in_poly((j,i), &poly) {
                hm.insert((j,i));
            }
        }
    }

    let res = hm.len();
    println!("Part 1: {}", res);


    let file = File::open("input.txt").unwrap();
    let res = io::BufReader::new(&file)
        .lines()
        .map(|x| {
            let x = x.unwrap();
            let mut x = x.split(' ');
            let d = match x.next().unwrap() {
                "R" => (1_i64,0_i64),
                "L" => (-1,0),
                "U" => (0,-1),
                "D" => (0,1),
                _ => unreachable!(),
            };
            let t = x.next().unwrap().parse::<i64>().unwrap();
            (d,t + 1)
        })
        .fold((0.0,(0_i64,0_i64)), |(a, (x1,y1)), ((d1,d2),t)| {
            // println!("{}-{}, {}", d1, d2, t);
            let x2 = x1 + t * d1;
            let y2 = y1 + t * d2;
            // println!("{},{}", x2, y2);
            (a + 0.5 * (y1 + y2) as f64 * (x1 - x2) as f64, (x2, y2))
        }).0;
     println!("Part 1a: {}", res);


    let re = regex::Regex::new(r"#([a-e0-9]{5})([0-3])").unwrap();

    let file = File::open("input.txt").unwrap();
    let res = io::BufReader::new(&file)
        .lines()
        .map(|x| {
            let x = x.unwrap();
            let (_,[t,d]) = re.captures(&x).map(|x| x.extract()).unwrap();
            let t = t.chars().rev().zip(0..5).fold(0, |a,(c, p)| a + c.to_digit(16).unwrap() as i64 * 16_i64.pow(p)) + 1;
            let d =  match d {
                "0" => (0,1),
                "2" => (0,-1),
                "1" => (1,0),
                "3" => (-1,0),
                _ => unreachable!(),
            };
            (d,t)
        })
        .fold((0.0,(0,0)), |(a, (x1,y1)), ((d1,d2),t)| {
            let x2 = x1 + t* d1;
            let y2 = y1 + t* d2;
            (a + 0.5 * (y1 + y2) as f64 * (x1 - x2) as f64, (x1, x2))
        }).0;
    println!("Part 2: {}", res);
}


fn pt_in_poly(pt: (i64,i64), poly: &Vec<(i64, i64)>) -> bool {

    let testx = pt.0;
    let testy = pt.1;

    let nvert = poly.len();
    let mut c = false;

    let mut j = nvert - 1;
    for i in 0..nvert {
        if ((poly[i].1 > testy) != (poly[j].1 > testy)) &&
             (testx < (poly[j].0 - poly[i].0) * (testy - poly[i].1) / (poly[j].1 - poly[i].1) + poly[i].0) {
            c = !c;
        }
        j = i;
    }
    c
}
