use regex::Regex;
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
            permute(s, &d)
        })
        .sum();

    println!("Part 1: {}", res);

    let file = File::open("input.txt").unwrap();
    let res: i64 = io::BufReader::new(&file)
        .lines()
        .map(|x| {
            let x = x.unwrap();
            println!("{}", x);
            let d: Vec<i64> = re.find_iter(&x).map(|x| x.as_str().parse().unwrap()).collect();
            let s = x.split(' ').next().unwrap().to_string();
            let d = (0..5).map(|_| d.clone()).flatten().collect();
            let s = (0..5).map(|_| s.clone())
                          .fold(String::new(), |mut a,i| {
                              a.push_str(&i);
                              a
                          });
            permute(&s, &d)
        })
        .sum();

    println!("Part 2: {}", res);
}



fn permute(s: &str, r: &Vec<i64>) -> i64 {

    // println!("called with {} and {:?}", s, r);
    let s = s.trim_matches('.');


    if s.len() < r.iter().sum::<i64>() as usize || s.len() == 0 {
        // print!(" -> {}\n", 0);
        return 0;
    }

    // if r.len() == 1 {
    //     if s.len() as i64 == r[0] {
    //         if s.chars().all(|x| x == '?' || x == '#') {
    //             print!(" -> {}\n", 1);
    //             return 1;
    //         } else {
    //             print!(" -> {}\n", 0);
    //             return 0;
    //         }
    //     }

    //     // print!(" -> {}\n", (s.len() - r[0] as usize).min(min_ps_len(s) + 1));
    //     // return (s.len() - r[0] as usize).min(min_ps_len(s) + 1) as i64;

    // }


    let mut cnt = 0;

    let (p, remainder) = s.split_at(r[0] as usize);

    if p.chars().all(|x| x == '?' || x == '#') {

        if r.len() == 1 && remainder.chars().all(|x| x != '#') {
            cnt += 1;
        } else if r.len() > 1 {
            let (p, rem) = remainder.split_at(1);
            if p != "#" {
                cnt += permute(rem, &r[1..].to_vec());
            }
        }
    }

    match s.split_at(1) {
        ("#", _) => {},
        (_, rem) => {
            cnt += permute(rem, r);
        }
    }

    // print!(" --> {}\n", cnt);
    cnt
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        assert_eq!(permute(".??..??...?##.", &vec![1,1,3]), 4);
        assert_eq!(permute("????.######..#####.", &vec![1,6,5]), 4);
        assert_eq!(permute("???#?##???", &vec![4,4]), 1);
        assert_eq!(permute("????.######..#####.", &vec![1,6,5]), 4);
    }
}
