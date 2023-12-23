

fn main() {

    let grid = include_bytes!("../input.txt")
        .split(|x| *x == b'\n')
        .map(|x| x.to_vec())
        .filter(|x| x.len() > 0)
        .collect::<Vec<Vec<u8>>>();

    let start_col =grid[0].iter().position(|x| *x == b'.').unwrap();
    let start =vec![ (0,start_col), (1,start_col)];
    let ways = walk(&grid, start, true).unwrap();

    println!("{:?}", ways);
    let res = ways.iter().fold(0, |a,&i| {
        if a > i {
            return a;
        }
        i
    });
    println!("Part 1: {res}.");

    let start =vec![ (0,start_col), (1,start_col)];
    let ways = walk(&grid, start, false).unwrap();
    let res = ways.iter().fold(0, |a,&i| {
        if a > i {
            return a;
        }
        i
    });
    println!("{:?}", ways);
    println!("Part 2: {res}");
}


fn walk(grd: &Vec<Vec<u8>>, mut p_history: Vec<(usize,usize)>, part1: bool) -> Option<Vec<usize>> {

    let (mut r, mut c) = p_history.last().unwrap();
    loop {
        // println!("{r},{c}");
        let next : Vec<(usize, usize)>= [(1_i32,0_i32), (-1,0), (0,1), (0,-1)]
            .into_iter()
            .filter(|(dr,dc)| {
                let rn = (r as i32 + *dr) as usize;
                let cn = (c as i32 + *dc) as usize;
                if p_history.contains(&(rn,cn)) {
                    return false
                }
                let pos = grd[rn][cn];
                match (pos, *dr, *dc, part1) {
                    (b'#',_,_,_) => false,
                    (b'.',_,_,_) => true,
                    (b'>',_,1, true) => true,
                    (b'<',_,-1, true) => true,
                    (b'^',-1,_, true) => true,
                    (b'v',1,_, true) => true,
                    (_,_,_, false) => true,
                    _ => false,
                }
            })
            .map(|(dr,dc)| {
                ((r as i32 + dr) as usize,
                (c as i32 + dc) as usize)
            })
            .collect();

        if next.is_empty() {
            return None;
        }
        if next.len() == 1 {
            // println!("{r},{c}");
            // maybe we have to add 1 to the result here?
            (r,c) = (next[0].0,next[0].1);
            if r == grd.len() - 1 {
                return Some(vec![p_history.len()])
            }
            p_history.push((r,c));
            continue;
        }
        return Some(
            next
                .into_iter()
                .filter_map(|next| {
                    let mut p_hist = p_history.clone();
                    p_hist.push(next);
                    walk(grd, p_hist, part1)
                })
                .fold(vec![], |mut a,mut i| {
                    a.append(&mut i);
                    a
                })
        )
    }
}

