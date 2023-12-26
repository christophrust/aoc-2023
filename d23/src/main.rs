use std::collections::HashMap;



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

    let nodes1 = grd_to_node(&grid, true);
    for node in nodes1.iter() {
        println!("{:?}", node);
    }
    println!("{:?}", nodes1.len());
    let ways = walk2(&nodes1, vec![(0,start_col)], grid.len() - 1).unwrap();


    println!("{:?}", ways);
    let res = ways.iter().fold(0, |a,&i| {
        if a > i {
            return a;
        }
        i
    });
    println!("Part 1: {res}.");


    let nodes2 = grd_to_node(&grid, false);
    for node in nodes2.iter() {
        println!("{:?}", node);
    }
    println!("{:?}", nodes2.len());
    let ways = walk2(&nodes2, vec![(0,start_col)], grid.len() - 1).unwrap();
    let res = ways.iter().fold(0, |a,&i| {
        if a > i {
            return a;
        }
        i
    });

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


fn walk2(nodes: &HashMap<(usize,usize), Vec<((usize,usize), usize)>>, p_history: Vec<(usize,usize)>, lrow: usize) -> Option<Vec<usize>> {
    let (r, c) = p_history.last().unwrap();
    if *r == lrow {
        return Some(vec![0]);
    }
    match nodes.get(&(*r,*c)) {
        Some(childs) => {
            Some(
                childs
                    .into_iter()
                    .filter_map(|x| {
                        if p_history.contains(&x.0) {
                            return None
                        }
                        let mut p_history = p_history.clone();
                        p_history.push(x.0);
                        let mut routes = walk2(&nodes, p_history, lrow)?;
                        for r in routes.iter_mut() {
                            *r += x.1;
                        };
                        Some(routes)
                    })
                    .flatten()
                    .collect()
            )
        },
        None => {
            return None;
        }
    }
}

fn grd_to_node(grd: &Vec<Vec<u8>>, part1: bool) -> HashMap<(usize,usize), Vec<((usize,usize), usize)>> {

    let mut res = HashMap::<(usize,usize), Vec<((usize,usize), usize)>>::new();
    let start_col =grd[0].iter().position(|x| *x == b'.').unwrap();
    res.insert((0, start_col), vec![]);
    let mut queue = Vec::<(usize,usize, usize, usize)>::new();

    let mut lnp = Vec::<(usize,usize)>::new();
    lnp.push((0, start_col));
    let (mut lnr, mut lnc) = (0, start_col);
    let (mut r, mut c) : (usize,usize);
    let (mut lr, mut lc) : (usize,usize);
    let mut plen: usize;
    let mut cnt = 0;

    queue.push((1, start_col, lnr, lnc));

    while !queue.is_empty() {
        (r,c, lnr, lnc) = queue.pop().unwrap();
        (lr, lc) = (lnr, lnc);
        plen = 1;
        let mut dir = false;
        println!("o: {r},{c}");
        println!("queue: {:?}", queue);
        loop {
            // println!("i: {r},{c}");
            // println!("{r},{c}");
            let next : Vec<(usize, usize)>= [(1_i32,0_i32), (-1,0), (0,1), (0,-1)]
                .into_iter()
                .filter(|(dr,dc)| {
                    let rn = (r as i32 + *dr) as usize;
                    let cn = (c as i32 + *dc) as usize;
                    if (rn, cn) == (lr, lc) || lnp.contains(&(rn,cn)) {
                        return false;
                    }
                    let pos = grd[rn][cn];
                    match (pos, *dr, *dc, part1) {
                        (b'#',_,_,_) => false,
                        (b'.',_,_,_) => true,
                        (b'>',_,1, true) => {dir = true;true},
                        (b'<',_,-1, true) => {dir = true;true},
                        (b'^',-1,_, true) => {dir = true;true},
                        (b'v',1,_, true) => {dir = true;true},
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
                //println!("Empty: {r},{c}");
                break;
            }

            if next.len() == 1 {
                plen +=1;
                (lr,lc) = (r,c);
                (r,c) = (next[0].0,next[0].1);
                if r == grd.len() - 1 {
                    if let Some(v) =  res.get_mut(&(lnr,lnc)) {
                        v.push(((r,c), plen));
                    } else {
                        res.insert((lnr,lnc), vec![((r,c), plen)]);
                    }
                    break;
                }
                if res.get(&(r,c)).is_some() {
                    if let Some(v) = res.get_mut(&(r,c)) {
                        v.push(((lnr, lnc), plen));
                    }
                    if let Some(v) = res.get_mut(&(lnr,lnc)) {
                        v.push(((lr,lc),plen));
                    }
                    break;
                }
                continue;
            }

            lnp.push((lr,lc));
            if let Some(v) = res.get_mut(&(lnr, lnc)) {
                v.push(((r,c), plen));
            } else {
                res.insert((lnr,lnc), vec![((r,c), plen)]);
            }
            if !dir {
                res.insert((r,c), vec![((lnr,lnc), plen)]);
            }
            (lnr, lnc) = (r,c);
            for p in next {
                queue.push((p.0, p.1, lnr, lnc));
            }
            cnt += 1;
            if !part1 && cnt == 100{
                return res;
            }
            break;
        }
    }
    res
}
