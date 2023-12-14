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
}

impl Platform {

    fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(path).unwrap();
        let mut r = 0;
        let mut cubes = Vec::<(usize,usize)>::new();
        let mut rocks = Vec::<(usize,usize)>::new();
        io::BufReader::new(&file)
            .lines()
            .for_each(|l| {
                for (i,c) in l.unwrap().char_indices() {
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
        Self { cubes, rocks, height: r}
    }

    fn slide(&mut self, d: char) {
        type O = std::cmp::Ordering;
        self.cubes.sort_unstable_by_key(|(r,c)| {
            match d {
                'N' => (*c,*r),
                'S' => (*c,self.height - *r),
                'E' => (*r,*c),
                'W' => (self.height - *r,*c),
                _ => unreachable!(),
            }
        });
        self.rocks.sort_unstable_by_key(|(r,c)| {
            match d {
                'N' => (*c,*r),
                'S' => (self.height - *c,self.height - *r),
                'E' => (*r,*c),
                'W' => (self.height - *r,self.height - *c),
                _ => unreachable!(),
            }
        });
        // println!("{:?}", self);

        let (mut c, mut r, mut i, mut j) = (self.cubes[0].1,0,0,0);
        loop {

            if d == 'N' || d == 'S' {
                if c != self.cubes[i].1 {
                    r = 0;
                    c = self.cubes[i].1;
                }
            } else {
                if c != self.cubes[i].0 {
                    r = 0;
                    c = self.cubes[i].0;
                }
            }

            let x =match d {
                'N' => (self.cubes[i].0.cmp(&self.rocks[j].0), self.cubes[i].1.cmp(&self.rocks[j].1)),
                'S' => (self.rocks[j].0.cmp(&self.cubes[i].0), self.rocks[j].1.cmp(&self.cubes[i].1)),
                'E' => (self.cubes[i].1.cmp(&self.rocks[j].1), self.cubes[i].1.cmp(&self.rocks[j].0)),
                'W' => (self.rocks[j].1.cmp(&self.cubes[i].1), self.rocks[j].1.cmp(&self.cubes[i].0)),
                _ => unreachable!(),
            };

            match x {
                (_, O::Greater) => {
                    if j + 1 < self.rocks.len() {
                        j+=1;
                    } else {
                        self.cubes[i].0 = r;
                        r+=1;
                        i+=1;
                    }
                },
                (O::Less, _) => {
                    self.cubes[i].0 = r;
                    r+=1;
                    i+=1;
                },
                (_, O::Equal) => {
                    // println!("--{r}");
                    if j + 1 < self.rocks.len() {
                        r = self.rocks[j].0 + 1;
                        j+=1;
                    } else if j + 1 == self.rocks.len() && r <= self.rocks[j].0 {
                        r = self.rocks[j].0 + 1;
                    } else {
                        self.cubes[i].0 = r;
                        r+=1;
                        i+=1;
                    }
                },
                (O::Greater, O::Less) => {
                    // println!("---{:?}",x);
                    self.cubes[i].0 = r;
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
