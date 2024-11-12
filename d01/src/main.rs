use std::fs::File;
use std::io::{self, BufRead};


fn main() {

    // part 1
    let file = File::open("input.txt")
        .unwrap();

    let s : u32 = io::BufReader::new(&file)
        .lines()
        .into_iter()
        .map(|x| {
            let x = x.unwrap();
            first_and_last_digit_part1(x)
        })
        .sum();

    println!("Result of part 1: {}", s);

    // part 2
    let file = File::open("input.txt")
         .unwrap();
    let s : u32 = io::BufReader::new(&file)
        .lines()
        .into_iter()
        .map(|x| {
            let x = x.unwrap();
            first_and_last_digit(x.clone(), true)
        })
        .sum();
    println!("Result of part 2: {}", s);
}


fn first_and_last_digit_part1(s: String) -> u32 {
    let mut res = 0_u32;
    let mut c = s.chars();

    while let Some(l) =  c.next() {
        if l.is_ascii_digit() {
            res += l.to_digit(10).unwrap() * 10;
            break;
        };
    }
    let mut c = s.chars();
    while let Some(r) = c.next_back() {
        if r.is_ascii_digit() {
            res += r.to_digit(10).unwrap();
            break;
        }
    }
    res
}


fn first_and_last_digit(s: String, tt: bool) -> u32 {

    let digits = vec![
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];
    let (mut li, mut ri, mut l, mut r, mut c) = (s.len(), 0, 0, 0, 0);
    if tt {
        for d in digits {
            if let Some(i) = s.find(d) {
                if i <= li {
                    li = i;
                    l = c + 1;
                }
            }
            if let Some(i) = s.rfind(d) {
                if i >= ri {
                    ri = i;
                    r = c as u32 + 1;
                }
            }
            c+=1;
        }

    }
    if let Some(i) = s.find(&['1', '2', '3', '4','5','6','7','8','9']) {
        if i <= li {
            l = s.chars().nth(i).unwrap().to_digit(10).unwrap();
        }
    }

    if let Some(i) = s.rfind(&['1', '2', '3', '4','5','6','7','8','9']) {
        if i >= ri {
            r = s.chars().nth(i).unwrap().to_digit(10).unwrap();
        }
    }
    l * 10 + r
}
