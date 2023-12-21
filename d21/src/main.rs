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


    for _i in 0..146 {
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


    // for r in 0..nr {
    //     let ln = (0..nc)
    //         .into_iter()
    //         .map(|c| {
    //             match pos.get(&(r,c)) {
    //                 Some(_) => 'O',
    //                 None => '.',
    //             }
    //         })
    //         .collect::<String>();
    //     println!("{ln}");
    // }
    let mut pos = HashSet::<(isize,isize)>::new();
    // pos.insert((0,129));
    pos.insert(spos);

    for _i in 0..(65 + 3 * 131) {
        pos = pos
            .into_iter()
            .fold(HashSet::<(isize,isize)>::new(), |mut a,(r,c)| {
                for (dr,dc) in [(1,0),(-1,0),(0,1), (0,-1)] {
                    let np = (r + dr, c + dc);
                    if stones.get(&(np.0 % 131, np.1 % 131)).is_none() {
                        a.insert(np);
                    }
                }
                a
            });
    }



    let res = pos.len();
    println!("Part2: {}",res);

    // println!("nc: {}",nc);
    // println!("nr: {}",nr);
    // println!("spos: {:?}",spos);
}
