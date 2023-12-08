use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;


fn main() {

    let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
    let file = File::open("input.txt").unwrap();
    let mut lines = io::BufReader::new(&file)
        .lines();

    let directions:Vec<char> = lines.next().unwrap().unwrap().chars().collect();;

    lines.next();


    let m = lines
        .into_iter()

}
