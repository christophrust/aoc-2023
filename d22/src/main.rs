use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::{HashMap,HashSet};

fn main() {

    let re = Regex::new(r"([0-9]*),([0-9]*),([0-9]*)~([0-9]*),([0-9]*),([0-9]*)")
        .unwrap();

    let mut index = 0_usize;
    let mut space = Space3D{bricks: HashMap::new(), allocated: HashSet::new()};

    let file = File::open("input1.txt").unwrap();
    io::BufReader::new(&file)
        .lines()
        .for_each(|l| {
            let l = l.unwrap();
            let (_, [sx,sy,sz,ex,ey,ez]) = re.captures(&l).unwrap().extract();

            let cubes = match (sx==ex, sy == ey, sz == ez) {
                (_,_,false) | (true,true,true) => {
                    let e = ez.parse::<usize>().unwrap();
                    let s = sz.parse::<usize>().unwrap();
                    let (v0, v1);
                    if e < s {
                        v0 = e;
                        v1 = s;
                    } else {
                        v0 = s;
                        v1 = e;
                    }
                    ((v0..=v1).into_iter().map(|v| (
                                sx.parse::<usize>().unwrap() ,
                                sy.parse::<usize>().unwrap(),
                                v
                            )
                    ).collect(), true)
                },
                (_,false,_) => {
                    let e = ey.parse::<usize>().unwrap();
                    let s = sy.parse::<usize>().unwrap();
                    let (v0, v1);
                    if e < s {
                        v0 = e;
                        v1 = s;
                    } else {
                        v0 = s;
                        v1 = e;
                    }
                   ( (v0..=v1).into_iter().map(|v| (
                                sx.parse::<usize>().unwrap() ,
                                v,
                                sz.parse::<usize>().unwrap(),
                            )
                    ).collect(), false)
                },
                (false,_,_) => {
                    let e = ex.parse::<usize>().unwrap();
                    let s = sx.parse::<usize>().unwrap();
                    let (v0, v1);
                    if e < s {
                        v0 = e;
                        v1 = s;
                    } else {
                        v0 = s;
                        v1 = e;
                    }
                    ((v0..=v1).into_iter().map(|v| (
                                v,
                                sy.parse::<usize>().unwrap(),
                                sz.parse::<usize>().unwrap() ,
                            )
                    ).collect(), false)
                },
            };
            space.bricks.insert(index, cubes);
            index += 1;
        });

    space.allocate();
    space.settle(false);

    let res = (0..index)
                .into_iter()
                .filter(|idx| {
                    space.take_out_is_safe(*idx)
                })
                .count();


    println!("Part1: {}", res);
}


#[derive(Debug,PartialEq, Eq, Clone)]
struct Space3D {
    bricks: HashMap<usize, (Vec<(usize,usize,usize)>, bool),>,
    allocated: HashSet<(usize,usize,usize)>,
}


impl Space3D {
    fn allocate(&mut self) {
        self.allocated = self.bricks.iter().map(|(_,p)| p.0.clone()).flatten().collect();
    }

    fn settle(&mut self, stop_early: bool) -> bool {

        let mut res = false;
        let mut bv : Vec<(usize,usize)>= self
            .bricks
            .iter()
            .map(|(&k,v)| {
                let minz = v.0[0].2.min(v.0[v.0.len()-1].2);
                (k, minz)
            })
            .collect();

        bv.sort_unstable_by(|a,b| {
            a.1.cmp(&b.1)
        });



        for (idx, minz) in bv {
            let mut dz = 0;
            let (cubes, oriented_down) = self.bricks.get_mut(&idx).unwrap();
            for zd in 1..minz {
                if *oriented_down {
                    let (px,py) = (cubes[0].0,cubes[0].1);
                    if self.allocated.get(&(px,py,minz-zd)).is_none() {
                        dz = zd;
                        if stop_early {
                            return true;
                        }
                        res = true;
                    } else {
                        break;
                    }
                } else {
                    if (0..cubes.len()).into_iter().all(|i| {
                        let (px,py) = (cubes[i].0,cubes[i].1);
                        self.allocated.get(&(px,py,minz-zd)).is_none()
                    }) {
                        dz = zd;
                        if stop_early {
                            return true;
                        }
                        res = true;
                    } else {
                        break;
                    }
                }
            }
            if dz > 0 {
                for cube in cubes {
                    self.allocated.remove(&cube);
                    cube.2 -= dz;
                    self.allocated.remove(cube);
                }
            }
        }
        res
    }
    fn take_out_is_safe(&self, index: usize) -> bool {
        let mut space_copy = self.clone();
        let take_out_brick = space_copy.bricks.remove(&index).unwrap();
        for cube in take_out_brick.0 {
            space_copy.allocated.remove(&cube);
        }
        !space_copy.settle(true)
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_name() {

    }
}
