use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {

    let re = Regex::new(r"[0-9]+").unwrap();

    let file = File::open("input.txt").unwrap();
    let mut res = io::BufReader::new(&file)
        .lines()
        .map(Result::unwrap)
        .fold((0,None,None), |mut windw, l| {
            let l1 = windw.1;
            windw.1 = windw.2;
            windw.2 = Some(l);
            //println!("{}", part1(&re, &l1, &accum.1,&accum.2));
            windw.0 += part1(&re, &l1, &windw.1,&windw.2);

            windw
        });
    res.0 += part1(&re, &None, &res.2, &res.1);
    println!("Part 1: {}", res.0);


    // part 2
    let file = File::open("input.txt").unwrap();

    let mut res = io::BufReader::new(&file)
        .lines()
        .map(Result::unwrap)
        .fold((0,None,None), |mut windw, l| {
            let l1 = windw.1;
            windw.1 = windw.2;
            windw.2 = Some(l);
            windw.0 += part2(&re, &l1, &windw.1,&windw.2);

            windw
        });
    res.0 += part2(&re, &None, &res.2, &res.1);
    println!("Part 2: {}", res.0);
}


fn part1(re:&Regex, l1: &Option<String>, l2: &Option<String>, l3: &Option<String>) -> u32 {
    match (l1,l2,l3) {
        (Some(l1), Some(l2), Some(l3)) => {
            re
                .find_iter(l2.as_str())
                .map(|x| {
                    let li = if x.start() > 0 {x.start() - 1} else {x.start()};
                    let ui = if x.end() < l2.len() {x.end() + 1 } else {x.end()};

                    let c1 = l1[li..ui].find(|x: char| {x != '.' && !x.is_ascii_digit()}).is_some();
                    let c2 = l2[li..ui].find(|x: char| {x != '.' && !x.is_ascii_digit()}).is_some();
                    let c3 = l3[li..ui].find(|x: char| {x != '.' && !x.is_ascii_digit()}).is_some();
                    if c1 || c2 || c3 {
                        return x.as_str().parse::<u32>().unwrap()
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        },
        (None, Some(l1), Some(l2)) => {
            re
                .find_iter(l1.as_str())
                .map(|x| {
                    let li = if x.start() > 0 {x.start() - 1} else {x.start()};
                    let ui = if x.end() < l1.len() {x.end() + 1 } else {x.end()};
                    let c1 = l1[li..ui].find(|x: char| {x != '.' && !x.is_ascii_digit()}).is_some();
                    let c2 = l2[li..ui].find(|x: char| {x != '.' && !x.is_ascii_digit()}).is_some();
                    if c1 || c2 {
                        return x.as_str().parse::<u32>().unwrap()
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        },
        _ => 0
    }
}



fn part2(re:&Regex, l1: &Option<String>, l2: &Option<String>, l3: &Option<String>) -> u32 {
    let mut m = Vec::<(usize,usize, u32)>::new();
    if let Some(l) = l1 {
        m.append(&mut re
            .find_iter(l.as_str())
            .map(|x| (x.start(), x.end(), x.as_str().parse::<u32>().unwrap()))
            .collect());
    }
    if let Some(l) = l2 {
        m.append(&mut re
            .find_iter(l.as_str())
            .map(|x| (x.start(), x.end(), x.as_str().parse::<u32>().unwrap()))
            .collect());
    }
    if let Some(l) = l3 {
        m.append(&mut re
            .find_iter(l.as_str())
            .map(|x| (x.start(), x.end(), x.as_str().parse::<u32>().unwrap()))
            .collect());
    }
    let mut line = &String::new();
    match (l1,l2,l3) {
        (Some(_), Some(l2), Some(_)) => {
            line = l2;
        },
        (None, Some(l1), Some(_)) => {
            line = l1;
        },
        _ => {
        }
    }
    line
        .match_indices('*')
        .map(|(i,_)|{
            let a = m
                .iter()
                .fold(Vec::<u32>::new(), |mut acc, item| {
                    if item.0 <= i + 1 && item.1 >= i {
                        // println!("{:?}, {}", item, i);
                        acc.push(item.2);
                    }
                    acc
                });
            if a.len() == 2 {
                a.iter().product()
            } else {
                0
            }
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;


    #[test]
    fn test_part1() {
        let re = Regex::new(r"[0-9]+").unwrap();

        let m = re.find("....1232....").unwrap();
        let s = "....1232....".to_string();
        println!("{:?}", &s[m.start()..m.end()]);

        let res = part1(
            &re,
            &Some("...*......".to_string()),
            &Some("..35..633.".to_string()),
            &Some("......#...".to_string())
        );
        assert_eq!(res, 35+633);

        let res = part1(
            &re,
            &Some("..35..633.".to_string()),
            &Some("...*......".to_string()),
            &None
        );
        assert_eq!(res, 35);

        let res = part1(
            &re,
            &Some("467..114..".to_string()),
            &Some("...*......".to_string()),
            &None
        );
        assert_eq!(res, 467);
    }

    #[test]
    fn test_part2() {
        let re = Regex::new(r"[0-9]+").unwrap();

        let res = part2(
            &re,
            &Some("..35.633..".to_string()),
            &Some("....*.....".to_string()),
            &Some("......#...".to_string())
        );
        assert_eq!(res, 35*633);

        let res = part2(
            &re,
            &Some("..35..633.".to_string()),
            &Some("...*......".to_string()),
            &None
        );
        assert_eq!(res, 0);

        let res = part2(
            &re,
            &Some("467*114...".to_string()),
            &Some("...*......".to_string()),
            &None
        );
        assert_eq!(res, 467*114);

        let res = part2(
            &re,
            &Some("467*114...".to_string()),
            &None,
            &None
        );
        assert_eq!(res, 467*114);

        let res = part2(
            &re,
            &Some("..35.633..".to_string()),
            &Some("....*.....".to_string()),
            &Some("....2.#...".to_string())
        );
        assert_eq!(res, 0);

        let res = part2(
            &re,
            &Some("...22...".to_string()),
            &Some(".....*..".to_string()),
            &Some("......22..".to_string())
        );
        assert_eq!(res, 0);
    }
}
