use std::fs::File;
use std::io::{self, BufRead};

fn main() {

    let file = File::open("input.txt").unwrap();
    let (a,l) = io::BufReader::new(&file)
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
            (d,t)
        })
        .fold(((0.0, 0.0),(0_i64,0_i64)), |((a, l), (x1,y1)), ((d1,d2),t)| {
            let x2 = x1 + t * d1;
            let y2 = y1 + t * d2;
            ((a + 0.5 * (y1 + y2) as f64 * (x1 - x2) as f64, l + t as f64), (x2, y2))
        }).0;
    let res = a.abs() + 0.5* l + 1.0;
    println!("Part 1a: {}", res);


    let re = regex::Regex::new(r"#([a-f0-9]{5})([0-3])").unwrap();

    let file = File::open("input.txt").unwrap();
    let (a,l) = io::BufReader::new(&file)
        .lines()
        .map(|x| {
            let x = x.unwrap();
            let (_,[t,d]) = re.captures(&x).map(|x| x.extract()).unwrap();
            let t = t.chars().rev().zip(0..5).fold(0, |a,(c, p)| a + c.to_digit(16).unwrap() as i64 * 16_i64.pow(p));
            let d =  match d {
                "0" => (0,1),
                "2" => (0,-1),
                "1" => (1,0),
                "3" => (-1,0),
                _ => unreachable!(),
            };
            (d,t)
        })
        .fold(((0.0, 0.0),(0_i64,0_i64)), |((a, l), (x1,y1)), ((d1,d2),t)| {
            let x2 = x1 + t * d1;
            let y2 = y1 + t * d2;
            ((a + 0.5 * (y1 + y2) as f64 * (x1 - x2) as f64, l + t as f64), (x2, y2))
        }).0;
    let res = a.abs() + 0.5* l + 1.0;
    println!("Part 2: {}", res);
}
