use std::fs::File;
use std::io::{self, BufRead};


fn main() {

    let file = File::open("input.txt").unwrap();

    let mut pos = (0,0);
    let mut cnt = 0_usize;
    let map = io::BufReader::new(&file)
        .lines()
        .map(|x| {
            let x = x.unwrap();
            if let Some(p) = x.find('S') {
                pos = (p, cnt)
            }
            cnt += 1;
            x.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();



    // In which direction do we start?
    let mut d = if vec!['|', 'J', 'L'].contains(&map[pos.1 + 1][pos.0]) {
        D::D
    } else if vec!['|', '7', 'F'].contains(&map[pos.1 - 1][pos.0]){
        D::U
    } else {
        D::R
    };

    let mut res = 0_usize;
    let mut path = vec![(pos.0 as i64, pos.1 as i64)];

    loop {

        match d {
            D::U => pos.1 -= 1,
            D::D => pos.1 += 1,
            D::L => pos.0 -= 1,
            D::R => pos.0 += 1,
        }
        path.push((pos.0 as i64, pos.1 as i64));

        res += 1;
        match map[pos.1][pos.0] {
            '|' => {},
            '-' => {},
            'L' => {
                if d == D::D {
                    d = D::R;
                } else {
                    d = D::U;
                }
            },
            'J' => {
                if d == D::D {
                    d = D::L;
                } else {
                    d = D::U;
                }
            },
            '7' => {
                if d == D::R {
                    d = D::D;
                } else {
                    d = D::L;
                }
            },
            'F' => {
                if d == D::U {
                    d = D::R;
                } else {
                    d = D::D;
                }
            },
            'S' => {
                break
            },
            _ => {unreachable!()},
        }
    }
    println!("Part 1: {}", res/2);

    let mut cnt = 0;
    for j in 0..map[0].len() {
        for i in 0..map.len() {
            if !path.contains(&(i as i64,j as i64)) && pt_in_poly((i as i64,j as i64), &path) {
                cnt += 1;
            }
        }
    }
    println!("Part 2: {}", cnt);
}


#[derive(Debug, PartialEq, Eq)]
enum D {
    U,
    D,
    R,
    L,
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
