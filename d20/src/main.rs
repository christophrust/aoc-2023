use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;



fn main() {

    let re = Regex::new(r"([a-z]+)\{(.*)\}")
        .unwrap();

    let file = File::open("input.txt").unwrap();
    let res = io::BufReader::new(&file)
        .lines()
        .count();

    println!("Part 1: {}", res);
}
