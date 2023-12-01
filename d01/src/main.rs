use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {

    // part 1
    let file = File::open("input.txt")
        .unwrap();

    let s : u32 = io::BufReader::new(&file)
        .lines()
        .into_iter()
        .filter_map(|x| {
            let x = x.ok()?;

            let mut res = 0_u32;
            let mut c = x.chars();

            while let Some(l) =  c.next() {
                if l.is_ascii_digit() {
                    res += l.to_digit(10).unwrap() * 10;
                    break;
                };
            }
            let mut c = x.chars();
            while let Some(r) = c.next_back() {
                if r.is_ascii_digit() {
                    res += r.to_digit(10).unwrap();
                    break;
                }
            }

            Some(res)
        })
        .sum();

    println!("Result of part 1: {}", s);

    // part 2
    //
    let s : u32 = io::BufReader::new(&file)
        .lines()
        .into_iter()
        .filter_map(|x| {
            let x = x.ok()?;
            Some(first_digit(&x)* 10 + last_digit(&x))
        })
        .sum();

    println!("Result of part 2: {}", s);
}




fn first_digit(s: &String) -> u32 {

    let s = s.as_str();

    for i in 0..s.len() {

    }
}

fn last_digit(s: &String) -> u32 {

}
