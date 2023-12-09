use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;


fn main() {

    let re = Regex::new(r"-*[0-9]+").unwrap();
    let file = File::open("input.txt").unwrap();
    let res: i64 = io::BufReader::new(&file)
        .lines()
        .into_iter()
        .map(|x| {
            let x = x.unwrap();
            let seq = re.find_iter(&x).map(|x| x.as_str().parse::<i64>().unwrap()).collect();
            extrapolate_sequence_end(seq)
        })
        .sum();


    println!("Part 1: {}", res);

    let file = File::open("input.txt").unwrap();
    let res: i64 = io::BufReader::new(&file)
        .lines()
        .into_iter()
        .map(|x| {
            let x = x.unwrap();
            let seq = re.find_iter(&x).map(|x| x.as_str().parse::<i64>().unwrap()).collect();
            extrapolate_sequence_start(seq)
        })
        .sum();


    println!("Part 2: {}", res);

}


fn extrapolate_sequence_end(s: Vec<i64>) -> i64 {
    let mut last_vals = Vec::<i64>::new();
    last_vals.push(*s.last().unwrap());

    let mut diffs = s;
    while diffs.iter().any(|&x| x != 0) {

        diffs = diffs
            .into_iter()
            .fold((Vec::<i64>::new(), None), |mut a,c| {
                if let Some(x) = a.1 {
                    a.0.push(c-x);
                }
                a.1 = Some(c);
                a
            }).0;
        last_vals.push(*diffs.last().unwrap())
    }

    last_vals.into_iter().sum()
}

fn extrapolate_sequence_start(s: Vec<i64>) -> i64 {
    let mut first_vals = Vec::<i64>::new();
    first_vals.push(*s.first().unwrap());

    let mut diffs = s;
    while diffs.iter().any(|&x| x != 0) {

        diffs = diffs
            .into_iter()
            .fold((Vec::<i64>::new(), None), |mut a,c| {
                if let Some(x) = a.1 {
                    a.0.push(c-x);
                }
                a.1 = Some(c);
                a
            }).0;
        first_vals.push(*diffs.first().unwrap())
    }

    first_vals.into_iter().rfold(0, |a, i| {
        i - a
    })
}
