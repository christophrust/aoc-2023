

fn main() {

    let grid = include_bytes!("../input1.txt")
        .split(|x| *x == b'\n')
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<u8>>>();
    let start_col =grid[0].iter().position(|x| *x == b'.').unwrap();
    let start =vec![ (0,start_col), (1,start_col)];
    let ways = walk(&grid, start).unwrap();

    let res = ways.into_iter().fold(0, |a,i| {
        if a > i {
            return a;
        }
        i
    });
    println!("Part 1: {res}.");
}


fn walk(grd: &Vec<Vec<u8>>, mut p_history: Vec<(usize,usize)>) -> Option<Vec<usize>> {

    let (mut r, mut c) = p_history.last().unwrap();
    loop {
        let next : Vec<(usize, usize)>= [(1_i32,0_i32), (-1,0), (0,1), (0,-1)]
            .into_iter()
            .filter(|(dr,dc)| {
                let rn = (r as i32 + *dr) as usize;
                let cn = (c as i32 + *dc) as usize;
                if p_history.contains(&(rn,cn)) {
                    return false
                }
                let pos = grd[rn][cn];
                match (pos, *dr, *dc) {
                    (b'#',_,_) => false,
                    (b'.',_,_) => true,
                    (b'>',_,1) => true,
                    (b'<',_,-1) => true,
                    (b'^',-1,_) => true,
                    (b'v',1,_) => true,
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
            // maybe we have to add 1 to the result here?
            if r == grd.len() - 1 {
                return Some(vec![p_history.len()])
            }
            (r,c) = (next[0].0,next[0].1);
            p_history.push((r,c));
            continue;
        }
        return Some(
            next
                .into_iter()
                .filter_map(|next| {
                    let mut p_hist = p_history.clone();
                    p_hist.push(next);
                    walk(grd, p_hist)
                })
                .fold(vec![], |mut a,mut i| {
                    a.append(&mut i);
                    a
                })
        )
    }
}
