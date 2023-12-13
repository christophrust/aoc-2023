use std::fs::File;
use std::io::{self, BufRead};


fn main() {
    let file = File::open("input.txt").unwrap();

    let mut cnt = 0_usize;
    let res = io::BufReader::new(&file)
        .lines()
        .fold(vec![Pattern::new()], |mut a, x| {
            let x = x.unwrap();
            if &x == "" {
                a.push(Pattern::new());
            } else {
                if let Some(pat) = a.last_mut() {
                    pat.add_row(x);
                }
            }
            a
         })
        .into_iter()
        .fold(0, |a, mut x| {
            x.finalize();
            cnt += 1;
            a + x.reflection_sig()
        });

    println!("Part 1: {}", res);


    let file = File::open("input.txt").unwrap();

    let mut cnt = 0_usize;
    let res = io::BufReader::new(&file)
        .lines()
        .fold(vec![Pattern::new()], |mut a, x| {
            let x = x.unwrap();
            if &x == "" {
                a.push(Pattern::new());
            } else {
                if let Some(pat) = a.last_mut() {
                    pat.add_row(x);
                }
            }
            a
         })
        .into_iter()
        .fold(0, |a, mut x| {
            x.finalize();
            cnt += 1;
            a + x.find_smug()
        });

    println!("Part 2: {}", res);
}


#[derive(Clone)]
struct Pattern {
    r: Vec<Vec<u8>>,
    c: Vec<Vec<u8>>,
    nr: Option<usize>,
    nc: Option<usize>,
}


impl Pattern {

    fn new() -> Self {
        Self { r: vec![], c: vec![], nr: None, nc: None }
    }

    fn add_row(&mut self, r: String) {
        self.r.push(r.chars().map(|x| x as u8).collect());
    }

    fn finalize(&mut self) {
        self.nr = Some(self.r.len());
        self.nc = Some(self.r[0].len());
        for i in 0..self.nc.unwrap() {
            let c: Vec<u8> = (0..self.r.len())
                .map(|j| {
                    self.r[j][i]
                })
                .collect();
            self.c.push(c);
        }
    }

    fn find_mirror_lines(&self) -> Vec<usize> {
        let mut m = self.find_rmirror();
        m.append(&mut self.find_cmirror());
        m
    }

    fn find_rmirror(&self) -> Vec<usize> {
        let mut rm = Vec::<usize>::new();
        for i in 1..self.nr.unwrap() {
            let (l,r) = self.r.split_at(i);
            if (0..r.len().min(l.len())).all(|j| {
                r[j] == l[l.len() - j - 1]
            }) {
                rm.push(100 * i);
            }
        }
        rm
    }

    fn find_cmirror(&self) -> Vec<usize> {
        let mut rc = Vec::<usize>::new();
        for i in 1..self.nc.unwrap() {
            let (l,r) = self.c.split_at(i);
            if (0..r.len().min(l.len())).all(|j| {
                r[j] == l[l.len() - j - 1]
            }) {
                rc.push(i);
            }
        }
        rc
    }

    fn reflection_sig(&self) -> usize {

        if self.find_rmirror().len() > 0 {
            return self.find_rmirror()[0];
        }
        self.find_cmirror()[0]
    }

    fn find_smug(&self) -> usize {
        let sig = self.find_mirror_lines();
        let mut fixed = self.clone();

        for i in 0..self.nr.unwrap() {
            for j in 0..self.nc.unwrap() {
                match self.c[j][i] {
                    b'#' => {
                        fixed.c[j][i] = b'.';
                        fixed.r[i][j] = b'.';
                        let fs = fixed.find_mirror_lines();
                        if fs.len() != 0 && fs!= sig {
                            return fs.into_iter().filter(|&x| x != sig[0]).next().unwrap();
                        }
                        fixed.c[j][i] = b'#';
                        fixed.r[i][j] = b'#';
                    },
                    _ =>  {
                        fixed.c[j][i] = b'#';
                        fixed.r[i][j] = b'#';
                        let fs = fixed.find_mirror_lines();
                        if fs.len() != 0 && fs!= sig {
                            return fs.into_iter().filter(|&x| x != sig[0]).next().unwrap();
                        }
                        fixed.c[j][i] = b'.';
                        fixed.r[i][j] = b'.';
                    }
                }
            }
        }
        unreachable!()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let mut p = Pattern::new();

        p.add_row("#.##..##.".to_string());
        p.add_row("..#.##.#.".to_string());
        p.add_row("##......#".to_string());
        p.add_row("##......#".to_string());
        p.add_row("..#.##.#.".to_string());
        p.add_row("..##..##.".to_string());
        p.add_row("#.#.##.#.".to_string());

        p.finalize();
        assert_eq!(p.find_cmirror()[0], 5);
        assert_eq!(p.reflection_sig(), 5);

        assert_eq!(p.find_smug(), 300);


        let mut p = Pattern::new();
        p.add_row("#...##..#".to_string());
        p.add_row("#....#..#".to_string());
        p.add_row("..##..###".to_string());
        p.add_row("#####.##.".to_string());
        p.add_row("#####.##.".to_string());
        p.add_row("..##..###".to_string());
        p.add_row("#....#..#".to_string());

        p.finalize();
        assert_eq!(p.find_rmirror()[0], 400);
        assert_eq!(p.reflection_sig(), 400);
        assert_eq!(p.find_smug(), 100);
    }
}
