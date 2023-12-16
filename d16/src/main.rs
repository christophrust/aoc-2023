use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {

    let file = File::open("input.txt").unwrap();
    let mut r = 1_usize;
    let mut c = 1_usize;
    let mut hm = HashMap::<(usize,usize), (char, Vec<char>)>::new();
    io::BufReader::new(&file)
        .lines()
        .for_each(|x| {
            let x = x.unwrap();
            c = 1_usize;
            x.chars().for_each(|cc|{
                hm.insert(
                    (r,c),
                    (cc, vec![])
                );
                c += 1;
            });
            r += 1;
        });

    let res = get_energized_fields(&hm, (1,1,'r'));

    println!("Part 1: {}", res);

    let mut res = res;
    (0..r).for_each(|rrr| {
        let m = get_energized_fields(&hm, (rrr + 1, 1,'r'));
        if m > res{
            res = m;
        }
        let m = get_energized_fields(&hm, (rrr + 1, c,'l'));
        if m > res{
            res = m;
        }
    });

    (0..c).for_each(|cc| {
        let m = get_energized_fields(&hm, (1, cc + 1,'d'));
        if m > res{
            res = m;
        }
        let m = get_energized_fields(&hm, (r, cc + 1,'u'));
        if m > res{
            res = m;
        }
    });

    println!("Part 2: {}", res);
}



fn get_energized_fields(
    hm: &HashMap<(usize,usize), (char, Vec<char>)>,
    start: (usize,usize, char)
) -> usize {

    let mut curr = start;
    let mut start: Vec<(usize,usize,char)> = vec![];
    let mut hm = hm.clone();
    loop {
        match hm.get_mut(&(curr.0,curr.1)) {
            Some((c,v)) => {
                if v.contains(&curr.2) {
                    if start.is_empty() {
                        break;
                    }
                    curr = start.pop().unwrap();
                    continue;
                }
                v.push(curr.2);
                match (curr.2, c) {
                    ('r','|') | ('l','|') => {
                        start.push((curr.0,curr.1,'u'));
                        curr = (curr.0 + 1, curr.1, 'd');
                    },
                    ('u', '-') | ('d', '-') => {
                        start.push((curr.0,curr.1,'r'));
                        curr = (curr.0, curr.1 - 1, 'l');
                    },
                    ('r','/') | ('l','\\') => {
                        curr = (curr.0 - 1, curr.1, 'u');
                    },
                    ('r','\\') | ('l','/') => {
                        curr = (curr.0 + 1, curr.1, 'd');
                    },
                    ('u','/') | ('d','\\')  => {
                        curr = (curr.0, curr.1 + 1, 'r');
                    },
                    ('u','\\') | ('d','/')  => {
                        curr = (curr.0, curr.1 - 1, 'l');
                    },
                    ('u',_) => {
                        curr.0 -=1;
                    },
                    ('d',_) => {
                        curr.0 +=1;
                    },
                    ('r',_) => {
                        curr.1 +=1;
                    },
                    ('l',_) => {
                        curr.1 -=1;
                    },
                    x => {
                        unreachable!("{:?}", x);
                    }
                }
            },
            None => {
                if start.is_empty() {
                    break;
                }
                curr = start.pop().unwrap();
                continue;
            }
        }
    }


    hm.iter().filter(|(_,(_,v))| {
        !v.is_empty()
    }).count()
}
