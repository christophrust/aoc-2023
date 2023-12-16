use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use cached::proc_macro::cached;

fn main() {
    let re = Regex::new(r"[0-9]+").unwrap();
    let file = File::open("input.txt").unwrap();


    let res: i64 = io::BufReader::new(&file)
        .lines()
        .map(|x| {
            let x = x.unwrap();
            let d = re.find_iter(&x).map(|x| x.as_str().parse().unwrap()).collect();
            let s = x.split(' ').next().unwrap();
            permute(s.to_string(), d)
        })
        .sum();

    println!("Part 1: {}", res);

    let file = File::open("input.txt").unwrap();
    let res: i64 = io::BufReader::new(&file)
        .lines()
        .map(|x|{
            let x = x.unwrap();
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
            // a + permute(&s, &d)
            // a + dc_permute(&s, &d)
            permute(s, d)
        })
        .sum::<i64>();

    println!("Part 2: {}", res);
}


#[cached]
fn permute(s: String, r: Vec<i64>) -> i64 {

    println!("called with {} and {:?}", s, r);
    // let s = s.trim_matches('.');


    if r.len() == 0 {
        return match s.find('#') {
            None => 1,
            Some(_) => 0,
        }
    }
    if s.len() == 0 && r.len() > 0 {
        return 0;
    }

    let mut cnt = 0;

    let (cur, rem) = (r[0], r[1..].to_vec());
    let up = s.len() - rem.iter().sum::<i64>() as usize - rem.len() - cur as usize +1;
    println!("{up}");
    for i in 0..up {
        println!("-- {i}");
        if s.chars().take(i).find(|&x| x == '#').is_some() {
            break;
        }
        let nxt = i + cur as usize;
        if nxt <= s.len() &&
            s[i..nxt].find('.').is_none() && s.chars().nth(nxt) != Some('#') {
                cnt += permute(s[nxt+1..].to_string(), rem.clone())
        }
    }

    cnt
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        assert_eq!(permute(".??..??...?##.".to_string(), vec![1,1,3]), 4);
        assert_eq!(permute("????.######..#####.".to_string(), vec![1,6,5]), 4);
        assert_eq!(permute("???#?##???".to_string(), vec![4,4]), 1);
        assert_eq!(permute("????.######..#####.".to_string(), vec![1,6,5]), 4);

        assert_eq!(permute("??".to_string(), vec![1]), 2);
        assert_eq!(permute("?????".to_string(), vec![2,1]), 3);
        assert_eq!(permute("?????".to_string(), vec![2,1]), 3);
        assert_eq!(permute("?????".to_string(), vec![2]), 4);

        assert_eq!(permute(".??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.".to_string(),
                             vec![1,1,3,1,1,3,1,1,3,1,1,3,1,1,3]), 16384);

        assert_eq!(
            permute("???.###????.###????.###????.###????.###".to_string(),
            vec![1,1,3,1,1,3,1,1,3,1,1,3,1,1,3]), 1
        );
        assert_eq!(
            permute("?###??????????###??????????###??????????###??????????###????????".to_string(),
            vec![3,2,1,3,2,1,3,2,1,3,2,1,3,2,1]), 506250
        );

    }


}
