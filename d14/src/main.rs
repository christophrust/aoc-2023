use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let file = File::open("input.txt").unwrap();

    let mut r = 100_usize;

    let res = io::BufReader::new(&file)
        .lines()
        .fold((0, [100;100]), |mut a, l| {
            for ci in l.unwrap().char_indices() {
                match ci.1 {
                    'O' => {
                        a.0 += a.1[ci.0];
                        a.1[ci.0] -= 1;
                    },
                    '#' => {
                        a.1[ci.0] = r - 1;
                    },
                    _ => {}
                }
            }
            r -= 1;
            a
        }).0;

    println!("Part 1: {}", res);

    let mut p = Platform::from_file("input.txt");
    p.slide('N');

    let res = p.get_load();

    println!("Part 2: {}", res);
}

#[derive(Debug)]
struct Platform {
    cubes: Vec<(usize,usize)>,
    rocks: Vec<(usize,usize)>,
    height: usize,
    width: usize,
}

impl Platform {

    fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(path).unwrap();
        let mut r = 0;
        let mut cubes = Vec::<(usize,usize)>::new();
        let mut rocks = Vec::<(usize,usize)>::new();
        let mut w = None;
        io::BufReader::new(&file)
            .lines()
            .for_each(|l| {
                let l = l.unwrap();
                if w.is_none() {w = Some(l.len())}
                for (i,c) in l.char_indices() {
                    match c {
                        'O' => {
                            cubes.push((r,i));
                        },
                        '#' => {
                            rocks.push((r,i));
                        },
                        _ => {}
                    }
                }
                r += 1;
            });
        Self { cubes, rocks, height: r, width: w.unwrap()}
    }

    fn arrange(&mut self, direction: char) {
        self.cubes.sort_unstable_by_key(|(r,c)| {
            match direction {
                'N' => (*c,*r),
                'S' => (*c,self.height - *r),
                'E' => (*r,*c),
                'W' => (*r,self.width - *c),
                _ => unreachable!(),
            }
        });
        self.rocks.sort_unstable_by_key(|(r,c)| {
            match direction {
                'N' => (*c,*r),
                'S' => (*c,self.height - *r),
                'E' => (*r,*c),
                'W' => (self.height - *r,*c),
                _ => unreachable!(),
            }
        });
    }

    fn set_stone_vpos(&mut self, i: usize, pos: usize, direction: char) {
        match direction {
            'N' => {
                self.cubes[i].0 = pos;
            }
            'S' => {
                self.cubes[i].0 = self.height - pos;
            }
            'E' => {
                self.cubes[i].1 = self.width - pos;
            },
            'W' => {
                self.cubes[i].1= pos;
            },
            _ => unreachable!(),
        };
    }


    fn get_stone_hpos(&self, i: usize, direction: char) -> usize {
        match direction {
            'N' | 'S' => self.cubes[i].1,
            'E' | 'W' => self.cubes[i].0,
            _ => unreachable!(),
        }
    }


    fn get_rock_vpos(&self, i: usize, direction: char) -> usize {
        match direction {
            'N' => self.rocks[i].0,
            'S' => self.height -self.rocks[i].0,
            'E' => self.width - self.rocks[i].1,
            'W' =>self.rocks[i].1,
            _ => unreachable!(),
        }
    }

    fn slide(&mut self, direction: char) {
        type O = std::cmp::Ordering;
        self.arrange(direction);

        let (mut c, mut r, mut i, mut j) = (self.get_stone_hpos(0, direction),0,0,0);
        loop {


            if c != self.get_stone_hpos(i, direction) {
                r = 0;
                c = self.get_stone_hpos(i, direction);
            }


            let x =match direction {
                'N' => (self.cubes[i].0.cmp(&self.rocks[j].0), self.cubes[i].1.cmp(&self.rocks[j].1)),
                'S' => (self.rocks[j].0.cmp(&self.cubes[i].0), self.cubes[i].1.cmp(&self.rocks[j].1)),
                'E' => (self.cubes[i].1.cmp(&self.rocks[j].1), self.cubes[i].1.cmp(&self.rocks[j].0)),
                'W' => (self.rocks[j].1.cmp(&self.cubes[i].1), self.rocks[j].1.cmp(&self.cubes[i].0)), // TODO
                _ => unreachable!(),
            };

            match x {
                (_, O::Greater) => {
                    if j + 1 < self.rocks.len() {
                        j+=1;
                    } else {
                        self.set_stone_vpos(i, r, direction);
                        r+=1;
                        i+=1;
                    }
                },
                (O::Less, _) => {
                    self.set_stone_vpos(i, r, direction);
                    r+=1;
                    i+=1;
                },
                (_, O::Equal) => {
                    // println!("--{r}");
                    if j + 1 < self.rocks.len() {
                        r = self.get_rock_vpos(j, direction) + 1;
                        j+=1;
                    } else if j + 1 == self.rocks.len() && r <= self.get_rock_vpos(j, direction) {
                        r = self.get_rock_vpos(j, direction) + 1;
                    } else {
                        self.set_stone_vpos(i, r, direction);
                        r+=1;
                        i+=1;
                    }
                },
                (O::Greater, O::Less) => {
                    // println!("---{:?}",x);
                    self.set_stone_vpos(i, r, direction);
                    r+=1;
                    i+=1;
                },
                _ => {unreachable!()}
            }
            if i == self.cubes.len() {
                break;
            }
        }
    }

    fn get_load(&self) -> usize {
        self.cubes.iter().map(|(r,_)| self.height - r).sum()
    }
}
