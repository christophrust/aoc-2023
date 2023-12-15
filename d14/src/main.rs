use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

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
    let res = p.cycle(1000000000);
    println!("Part 2: {}", res);
}


#[derive(Debug, Clone, Hash)]
struct Platform {
    cubes: Vec<(usize,usize)>,
    rocks: Vec<(usize,usize)>,
    height: usize,
    width: usize,
}


impl PartialEq for Platform {
    fn eq(&self, other: &Self) -> bool {
        let mut s_cubes = self.cubes.clone();
        let mut o_cubes = other.cubes.clone();
        let mut s_rocks = self.rocks.clone();
        let mut o_rocks = other.rocks.clone();

        s_cubes.sort_unstable();
        o_cubes.sort_unstable();
        for i in 0..s_cubes.len() {
            if s_cubes[i] != o_cubes[i] {
                return false;
            }
        }
        s_rocks.sort_unstable();
        o_rocks.sort_unstable();
        for i in 0..s_rocks.len() {
            if s_rocks[i] != o_rocks[i] {
                return false;
            }
        }
        true
    }
}
impl Eq for Platform {}

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
                'W' => (*r,*c),
                'E' => (*r,self.width - *c),
                _ => unreachable!(),
            }
        });
        self.rocks.sort_unstable_by_key(|(r,c)| {
            match direction {
                'N' => (*c, *r),
                'S' => (*c, self.height - *r),
                'W' => (*r, *c),
                'E' => (*r, self.width -*c),
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
                self.cubes[i].0 = self.height - pos -1;
            }
            'E' => {
                self.cubes[i].1 = self.width - pos - 1;
            },
            'W' => {
                self.cubes[i].1 = pos;
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
            'S' => self.height - self.rocks[i].0 -1,
            'E' => self.width - self.rocks[i].1 -1,
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


            let (s1, s2, r1, r2) =match direction {
                'N' => (self.cubes[i].0, self.cubes[i].1, self.rocks[j].0, self.rocks[j].1),
                'S' => (self.height - self.cubes[i].0, self.cubes[i].1, self.height - self.rocks[j].0, self.rocks[j].1),
                'W' => (self.cubes[i].1, self.cubes[i].0, self.rocks[j].1, self.rocks[j].0),
                'E' => (self.width - self.cubes[i].1, self.cubes[i].0, self.width - self.rocks[j].1, self.rocks[j].0),
                _ => unreachable!(),
            };
            let x = (s1.cmp(&r1), s2.cmp(&r2));
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
                (O::Less, _) | (O::Equal, O::Less) => {
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
            }
            if i == self.cubes.len() {
                break;
            }
        }
    }

    fn get_load(&self) -> usize {
        self.cubes.iter().map(|(r,_)| self.height - r).sum()
    }

    fn cycle(&mut self, times: usize) -> usize {

        let mut hist = HashMap::<Self, usize>::new();
        let mut loads = Vec::<usize>::new();
        let mut start_cycle = 0_usize;
        let mut cycle_len = 0_usize;
        for i in 0..times {
            self.slide('N');
            self.slide('W');
            self.slide('S');
            self.slide('E');

            loads.push(self.get_load());

            if let Some(s) = hist.get(&self) {
                println!("cycle len: {}", i + 1 - *s);
                start_cycle = *s;
                cycle_len = i + 1 - *s;
                break;
            } else {
                hist.insert(self.clone(), i + 1);
            }
        }

        let idx = (times - start_cycle) % cycle_len + start_cycle - 1;
        return loads[idx];
    }

    #[allow(dead_code)]
    fn print(&self){
        println!("---------------------------------");
        for i in 0..self.width {
            for j in 0..self.height {
                if self.cubes.contains(&(i,j)) {
                    print!("O");
                } else if self.rocks.contains(&(i,j)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
        println!("---------------------------------");
    }
}
