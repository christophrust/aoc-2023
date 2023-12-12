use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};


fn main() {
    let re = Regex::new(r"[0-9]+").unwrap();
    let file = File::open("input.txt").unwrap();


    let res: i64 = io::BufReader::new(&file)
        .lines()
        .map(|x| {
            let x = x.unwrap();
            let d = re.find_iter(&x).map(|x| x.as_str().parse().unwrap()).collect();
            let s = x.split(' ').next().unwrap();
            permute(s, d)
        })
        .sum();

    println!("Part 1: {}", res);

    println!("Part 2: {}", res);
}



fn permute(s: &str, r: Vec<i64>) -> i64 {
    let re = Regex::new(r"[?#]+").unwrap();
    let inp: Vec<i64> = re.find_iter(&s).map(|x| x.as_str().len() as i64).collect();

    if r.len() == inp.len() {
        return inp.iter().zip(r.iter()).fold(1, |a, (i, r)| {
            a * (i - r + 1)
        })
    }

    println!("{} ---- {:?} ---  {:?}", s, r, inp);
    1
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        assert_eq!(permute(".??..??...?##.", vec![1,1,3]), 4);
        assert_eq!(permute("????.######..#####.", vec![1,6,5]), 4);
        assert_eq!(permute("???#?##???", vec![4,4]), 3);
    }
}
