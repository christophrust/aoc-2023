use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};



fn main() {

    let mut stones = HashSet::<(isize,isize)>::new();

    let mut r = 0_isize;
    let mut nc = 0_isize;
    let mut spos = (0,0);
    let file = File::open("input.txt").unwrap();
    io::BufReader::new(&file)
        .lines()
        .for_each(|x| {
            let x = x.unwrap();
            x.char_indices().for_each(|(c,cc)| {
                nc = (c as isize +1).max(nc);
                match cc {
                    '#' => {stones.insert((r,c as isize));},
                    'S' => {spos = (r,c as isize);},
                    _ => {}
                }
            });
            r += 1;
        });
    let nr = r;




    let mut pos = HashSet::<(isize,isize)>::new();
    // pos.insert((0,129));
    pos.insert(spos);


    for _i in 0..64 {
        pos = pos
            .into_iter()
            .fold(HashSet::<(isize,isize)>::new(), |mut a,(r,c)| {
                for (dr,dc) in [(1,0),(-1,0),(0,1), (0,-1)] {
                    let np = (r + dr, c + dc);
                    if np.0 >= nr || np.1 >= nc || np.0 < 0 || np.1 < 0  {
                        continue;
                    }
                    if stones.get(&np).is_none() {
                        a.insert(np);
                    }
                }
                a
            });
    }



    let res = pos.len();
    println!("Part1: {}",res);


    let pat: Vec<usize>=  (0..3).into_iter().map(|j| {
        let mut pos = HashSet::<(isize,isize)>::new();
        pos.insert(spos);

        for _i in 0..(65 + j * 131) {
            pos = pos
                .into_iter()
                .fold(HashSet::<(isize,isize)>::new(), |mut a,(r,c)| {
                    for (dr,dc) in [(1,0),(-1,0),(0,1), (0,-1)] {
                        let np = (r + dr, c + dc);
                        let (mut pr, mut pc) = (np.0 % 131, np.1 % 131);
                        if pr < 0 {
                            pr += 131;
                        }
                        if pc < 0 {
                            pc += 131;
                        }
                        if stones.get(&(pr, pc)).is_none() {
                            a.insert(np);
                        }
                    }
                    a
                });
        }
        pos.len()
    }).collect();

    let fd : Vec<usize>= (1..pat.len())
        .into_iter()
        .map(|i| pat[i] - pat[i-1])
        .collect();

    let sd: Vec<usize> = (1..fd.len())
        .into_iter()
        .map(|i| fd[i] - fd[i-1])
        .collect();

    let d2 = sd[0] as u64;
    let d1 = fd[0] as u64- d2;
    let d0 = pat[0] as u64;


    let i = 202300_u64; //  = (26501365 - 65) % 131
    println!("Part 2: {}", i * d1 + i * (i+1) / 2 * d2 + d0)
}



#[cfg(test)]
mod tests {

    #[test]
    fn test_name() {

        println!("{:?}", (0..4).into_iter().map(|i| i * 30572).collect::<Vec<usize>>());
        assert_eq!(-3%2, 1);
    }
}



// d2_i = c2
// d1_1 = c2 * i + c1
// d = sum d1_i + c = i * c1 + sum i * c2 =
