use std::fs::File;
use std::io::{self, BufRead};


fn main() {
    let file = File::open("input.txt").unwrap();

    let mut r = 100_usize;
    let res = io::BufReader::new(&file)
        .lines()
        .fold((0, [100;100]), |mut a, l| {
            for ci in l.unwrap().char_indices() {
                match ci.1 {
                    'O' => {
                        a.0 += a.1[ci.0];
                        a.1[ci.0] -= 1;
                    },
                    '#' => {
                        a.1[ci.0] = r - 1;
                    },
                    _ => {}
                }
            }
            r -= 1;
            a
        }).0;

    println!("Part 1: {}", res);


    println!("Part 2: {}", res);
}
