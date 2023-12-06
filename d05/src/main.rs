use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    let re = Regex::new(r"[0-9]+").unwrap();
    let re2 = Regex::new(r"([0-9]+) ([0-9]+) ([0-9]+)").unwrap();

    let file = File::open("input.txt").unwrap();
    let mut bf = io::BufReader::new(&file).lines();

    let seeds: Vec<i64> = re
        .find_iter(
            &bf.next().unwrap().unwrap()
        )
        .filter_map(|x| x.as_str().parse::<i64>().ok())
        .collect();

    let mut maps = Vec::<Vec<(i64,i64,i64)>>::new();
    while let Some(Ok(l)) = bf.next() {
        if l == "" {
            maps.push(Vec::<(i64,i64,i64)>::new())
        }
        if let Some((_,[a,b,c])) = re2.captures(&l).map(|x| x.extract()) {
            let l = maps.last_mut().unwrap();

            l.push((
                a.parse::<i64>().unwrap(),
                b.parse::<i64>().unwrap(),
                c.parse::<i64>().unwrap()
            ));
        };
    }

    // part 1
    let res = seeds
        .iter()
        .fold(-1, |mut accum, &x| {
            let mut res = x;
            for m in maps.iter() {
                for (d, s, r) in m {
                    if res >= *s && res <= *s + *r {
                        res = d + res - s;
                        break;
                    }
                }
            }
            if res < accum || accum == -1 {
                accum = res;
            }
            accum
        });
    println!("Part 1: {:?}", res);

    // part 2
    let res = seeds
        .chunks(2)
        .fold(-1, |mut accum,x| {

            let res = map_min_pos(x[0], x[1], &maps, 0);
            // print!("->{}\n", res);
            if res < accum || accum == -1 {
                accum = res;
            }

            accum
        });


    println!("Part 2: {:?}", res);

}


fn map_min_pos(source: i64, range: i64, mi: &Vec<Vec<(i64,i64,i64)>>, depth: usize) -> i64 {
    // println!("Called with {}, {}, {}", source, range, depth);
    let mut res = -1;
    let mut temp_res;
    let mut c = Some((source,range));

    while let Some((current_s, current_r)) = c {
        // println!("while loop");

        let mut max_s = 0;
        temp_res = -1;
        for (d, s, r) in mi[depth].iter() {

            if *s > max_s {max_s = *s;}
            let range_oversize = (current_s + current_r) - (*s + *r);

            match
                (
                    current_s >= *s && current_s < *s + *r,
                    current_s + current_r <= *s + *r,
                    depth+1 < mi.len()
                ) {
                    (true, true, true) => {
                        temp_res = map_min_pos(*d + current_s - *s, current_r, mi, depth + 1);
                        c = None;
                        break;
                    },
                    (true,true,false) => {
                        temp_res = *d + current_s - *s;
                        c = None;
                        break;
                    },
                    (true, false, true) => {
                        temp_res = map_min_pos(*d + current_s - *s, current_r - range_oversize , mi, depth + 1);
                        c = Some((*s + *r, range_oversize));
                        break;
                    },
                    (true, false, false) => {
                        // println!("blub: {}, {}, {}, {}, {}", s, r, range_oversize, current_s, current_r);
                        temp_res = *d;
                        c = Some((*s + *r, range_oversize));
                        break;
                    },
                    _ => {},
                }
        }
        if temp_res == -1 {
            if let Some((next_entry, next_range)) = find_next_entry(current_s, current_r, &mi[depth]) {
                if depth + 1 < mi.len() {
                    temp_res = map_min_pos(current_s, next_entry - current_s + 1, mi, depth + 1);
                } else {
                    temp_res = current_s;
                }
                c = Some((next_entry, next_range));
            } else {
                if depth + 1 < mi.len() {
                    temp_res = map_min_pos(current_s, current_r, mi, depth + 1);
                } else {
                    temp_res = current_s;
                }
                c = None;
            }
        }
        if temp_res < res || res == -1 {
            res = temp_res;
        }
    }
    res
}

fn find_next_entry(source: i64, range: i64, maps: &Vec<(i64,i64,i64)>) -> Option<(i64, i64)> {
    let next = maps.iter().fold(0, |mut val, (_,x,_)| {
        if val == 0 && *x >= source && *x < source + range {
            val = *x;
        } else if *x < val && *x >= source && *x < source + range {
            val = *x
        }
        val
    });
    match next {
        0 => None,
        x => Some((x, source + range - x)),
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let map : Vec<Vec<(i64,i64,i64)>> = vec![
            vec![
                (88,18,7),
                (18,25,70)
            ],
            vec![
                (45, 77, 23),
                (81,45,19),
                (68,64,13)
            ],
            vec![
                (0,69,1),
                (1,0,69)
            ],
            vec![
                (60,56,37),
                (56,93,4)
            ]
        ];
        let res = map_min_pos(57, 0, &map, 2);
        assert_eq!(res, 62);

        let map1 = vec![
            vec![
                (88,18,7),
                (18,25,70)
            ]];
        let res = map_min_pos(88, 0, &map1, 0);
        assert_eq!(res, 81);

        let res = map_min_pos(88, 0, &map, 0);
        assert_eq!(res, 50);

        let res = map_min_pos(56, 40, &map, 3);
        assert_eq!(res, 56);
    }

    #[test]
    fn test_find_next_entry() {
        let m = vec![(88,18,7),(18,25,70)];
        assert_eq!(find_next_entry(10, 10, &m), Some((18,2)));
    }
}
