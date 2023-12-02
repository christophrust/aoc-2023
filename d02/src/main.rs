use std::fs::File;
use std::io::{self, BufRead};
extern crate regex;
use regex::Regex;

fn main() {
    let re_cube = Regex::new(r"([0-9]*) (red|blue|green)").unwrap();

    let file = File::open("input.txt").unwrap();
    let mut g = 0_u32 ;
    let mut check = true;

    let res: u32 = io::BufReader::new(&file)
        .lines()
        .filter_map(|x| {
            let x = x.unwrap();
            check = true;
            for (_,[n,c]) in re_cube.captures_iter(&x).map(|x| x.extract()) {
                let n = n.parse::<u32>().unwrap();
                match c {
                    "red" => if n > 12 {check = false;},
                    "green" => if n > 13 {check = false;},
                    "blue" => if n > 14 {check = false;},
                    _ => unreachable!(),
            }
            }
            g +=1;
            if check {
                Some(g)
            } else {
                None
            }
        })
        .sum();

    println!("Part 1: {}", res);


    // part 2
    let file = File::open("input.txt").unwrap();

    let res: u32 = io::BufReader::new(&file)
        .lines()
        .map(|x| {
            let x = x.unwrap();
            let (mut r,mut g,mut b) = (0,0,0);
            for (_,[n,c]) in re_cube.captures_iter(&x).map(|x| x.extract()) {
                let n = n.parse::<u32>().unwrap();
                match c {
                    "red" => if n > r {r = n;},
                    "green" => if n > g {g = n;},
                    "blue" => if n > b {b = n;},
                    _ => unreachable!(),
                }
            }
            r * g * b
        })
        .sum();

    println!("Part 2: {}", res);
}
