use std::fs::File;
use std::io::{self,BufRead};
use regex::Regex;

fn main() {
    let re = Regex::new(r"([-0-9]*), ([-0-9]*), ([-0-9]*) @ ([-0-9]*), ([-0-9]*), ([-0-9]*)").unwrap();

    // let file = File::open("input1.txt").unwrap();
    // let test_area = [[7.0,27.0]; 3];

    let file = File::open("input.txt").unwrap();
    let test_area = [[200000000000000.0,400000000000000.0]; 3];

    let particles: Vec<Particle<f64>> = io::BufReader::new(&file)
        .lines()
        .map(|x| {
            let x = x.unwrap();
            let (_, vals): (_,[&str; 6]) = re.captures(&x).expect(&format!("{:?}",x)).extract();
            let [x,y,z,vx,vy,vz]: [f64;6] = vals
                .into_iter()
                .map(|x| x.parse::<f64>().unwrap())
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap();
            Particle{x,y,z,vx,vy,vz}
        })
        .collect();

    let mut res = 0;
    for i in 0..particles.len() {
        for j in (i+1)..particles.len() {
            if particles[i].intersetcts(
                &particles[j],
                test_area,
                false) {
                res += 1;
            }
        }
    };
    println!("Part 1: {}", res);

}

struct Particle<T> {
    x: T,
    y: T,
    z: T,
    vx: T,
    vy: T,
    vz: T,
}

impl Particle<f64> {
    fn intersetcts(&self, other: &Self, area: [[f64; 2];3], usez: bool) -> bool {
        let mu = ((other.y - self.y) *self.vx /self.vy - other.x + self.x)/( other.vx - other.vy * self.vx / self.vy );
        let lam = ((self.y - other.y) *other.vx /other.vy - self.x + other.x)/( self.vx - self.vy * other.vx / other.vy );

        // println!("mu: {}", mu);
        if mu < 0.0 || lam < 0.0{
            return false
        }
        let cx = other.x + mu * other.vx;
        // println!("X: {}",cx);
        if cx < area[0][0] || cx > area[0][1] {
            return false;
        }
        let cy = other.y + mu * other.vy;
        // println!("Y: {}",cy);
        if cy < area[1][0] || cy > area[1][1] {
            return false;
        }
        if !usez {
            return true;
        }
        // let cz = self.z + f * self.vz;
        // if cz != other.z + f* other.vz  || cz < area[2][0] || cy > area[2][1] {
        //     return false;
        // }
        true
    }
}


#[cfg(test)]
mod tests {
    use regex::Regex;
    #[test]
    fn test_name() {
        let re = Regex::new(r"([-0-9]*), ([-0-9]*), ([-0-9]*) @ ([-0-9]*), ([-0-9]*), ([-0-9]*)").unwrap();
        // let hs = "19, 13, 30 @ -2,  1, -2";
        let hs = "19, 13, 30 @ -2, 1, -10";
        assert!(re.find(&hs).is_some());

    }
}

// mu  = ((y2 - y1) *vx1 /vy1 -x2 + x1)/( vx2 - vy2 * vx1 /vy1 )
