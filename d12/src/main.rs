use rayon::prelude::{IntoParallelIterator, ParallelIterator};
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
        .map(Result::unwrap)
        .collect::<Vec<String>>()
        .into_par_iter()
        .fold(|| 0_i64, |a, x| {
            println!("{}", x);
            let d: Vec<i64> = re.find_iter(&x).map(|x| x.as_str().parse().unwrap()).collect();
            let s = x.split(' ').next().unwrap().to_string();
            let d = (0..5).map(|_| d.clone()).flatten().collect();
            let s = (0..5).map(|_| s.clone())
                          .fold(String::new(), |mut a,i| {
                              if &a != "" {
                                  a.push_str("?");
                              }
                              a.push_str(&i);
                              a
                          });
            a + permute(&s, &d)
        })
        .sum::<i64>();

    println!("Part 2: {}", res);
}

fn factorial(x: i64) -> i64 {
    if x == 0 || x == 1 {
        return 1;
    }
    (1..=x).into_iter().product()
}

fn binom(n: i64, k: i64) -> i64 {
    if k > n {
        return 0;
    }
    factorial(n) / (factorial(k) * factorial(n-k))
}

fn permute(s: &str, r: &Vec<i64>) -> i64 {

    // println!("called with {} and {:?}", s, r);
    let s = s.trim_matches('.');



    if s.len() < (r.iter().sum::<i64>() as usize + r.len() - 1) || s.len() == 0 {
        return 0;
    }

    if s.chars().all(|x| x == '?') {
        if r.len() == 1 {
            return s.len() as i64 - r[0] + 1;
        }

        let a = r.iter().map(|x| x+1).sum::<i64>();
        let b = s.len() as i64 + 1;
        let rr = binom(r.len() as i64 + b - a, r.len() as i64);
       //  print!(" rr-> {}\n", rr);
        return rr;
    }

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

        assert_eq!(permute("?????", &vec![2,1]), 3);
        assert_eq!(permute("?????", &vec![2,1]), 3);
        assert_eq!(permute("?????", &vec![2]), 4);

        assert_eq!(permute(".??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.",
                             &vec![1,1,3,1,1,3,1,1,3,1,1,3,1,1,3]), 16384);
        assert_eq!(
            permute("???.###????.###????.###????.###????.###",
            &vec![1,1,3,1,1,3,1,1,3,1,1,3,1,1,3]), 1
        );
        assert_eq!(
            permute("?###??????????###??????????###??????????###??????????###????????",
            &vec![3,2,1,3,2,1,3,2,1,3,2,1,3,2,1]), 506250
        );

    }

    #[test]
    fn test_binom() {
        assert_eq!(binom(3,2), 3);
        assert_eq!(binom(4,2), 6);
    }
}
