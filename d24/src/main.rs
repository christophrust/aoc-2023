use std::fs::File;
use std::io::{self,BufRead};
use regex::Regex;
//
// use f128::f128;
 type f128 = f64;


fn main() {
    let re = Regex::new(r"([-0-9]*), ([-0-9]*), ([-0-9]*) @ ([-0-9]*), ([-0-9]*), ([-0-9]*)").unwrap();

    let file = File::open("input.txt").unwrap();
    // let test_area = [[7.0,27.0]; 3];

    // let file = File::open("input.txt").unwrap();
    let mut test_area = [[f128::from(200000000000000.0),f128::from(400000000000000.0)]; 3];

    let mut particles: Vec<Particle<f128>> = io::BufReader::new(&file)
        .lines()
        .map(|x| {
            let x = x.unwrap();
            let (_, vals): (_,[&str; 6]) = re.captures(&x).expect(&format!("{:?}",x)).extract();
            let [x,y,z,vx,vy,vz]: [f128;6] = vals
                .into_iter()
                .map(|x| f128::from(x.parse::<f64>().unwrap()))
                .collect::<Vec<f128>>()
                .try_into()
                .unwrap();
            Particle{x,y,z,vx,vy,vz}
        })
        .collect();


    // let (mx, my, mz) = particles.iter().fold((particles[0].x, particles[0].y, particles[0].z), |a,i| {
    //     (a.0.min(i.x), a.1.min(i.y), a.2.min(i.z))
    // });
    // for p in particles.iter_mut() {
    //     p.x -= mx;
    //     p.y -= my;
    //     p.z -= mz;
    // }
    // test_area[0][0] -= mx;
    // test_area[0][1] -= mx;
    // test_area[1][0] -= my;
    // test_area[1][1] -= my;
    // test_area[2][0] -= mz;
    // test_area[2][1] -= mz;

    let mut res = 0;
    for i in 0..particles.len() {
        for j in (i+1)..particles.len() {
            if particles[i].intersetcts(
                &particles[j],
                Some(test_area)) {
                res += 1;
            }
        }
    };
    println!("Part 1: {}", res);

    let mut res = 0;
    for i in 0..particles.len() {
        for j in (i+1)..particles.len() {
            if particles[i].parallel_and_disj(
                &particles[j]) {
                res += 1;
                println!("{i}, {j}")
            }
        }
    };

    println!("Nparallel 1: {}", res);
    if let Some((p, t0, t1)) = find_rock_line(&particles) {
        let (x,y,z, vx, vy, vz, t0, t1) = if t0.abs() < t1.abs() {
            println!("check");
            (p.x, p.y, p.z, p.vx, p.vy, p.vz, t0, t1)
        } else {
            (p.x + p.vx, p.y + p.vy, p.z + p.vz, -p.vx, -p.vy, -p.vz, t1, t0)
        };
        println!("normalized: {t0}, {},, {:?}", t1-t0, (x,y,z,vx,vy,vz));
        let f = t0/(t1 - t0);

        let px0: f64 = (x - f * vx).into();
        let py0: f64 = (y - f * vy).into();
        let pz0: f64 = (z - f * vz).into();
        let res: f64 = (px0 + py0 + pz0).into();
        println!("Part2: {},{},{}, {}", px0, py0, pz0, res)
    };

    let pp = particles.iter().map(|p| [
        p.x as i64,
        p.y as i64,
        p.z as i64,
        p.vx as i64,
        p.vy as i64,
        p.vz as i64,
    ])
        .collect::<Vec<[i64;6]>>();

    let res = part2(&pp);
    println!("Part2: {res}");
}

#[derive(Copy, Clone, Debug)]
struct Particle<T> {
    x: T,
    y: T,
    z: T,
    vx: T,
    vy: T,
    vz: T,
}

impl Default for Particle<f128> {
    fn default() -> Self {
        Self { x: f128::from(0), y: f128::from(0), z: f128::from(0), vx: f128::from(0), vy: f128::from(0), vz: f128::from(0)}
    }
}

impl Particle<f128> {
    fn intersetcts(&self, other: &Self, area: Option<[[f128; 2];3]>) -> bool {
        let mu = ((other.y - self.y) *self.vx /self.vy - other.x + self.x)/( other.vx - other.vy * self.vx / self.vy );
        let lam = ((self.y - other.y) *other.vx /other.vy - self.x + other.x)/( self.vx - self.vy * other.vx / other.vy );

        // println!("mu: {}", mu);
        if mu < f128::from(0) || lam < f128::from(0){
            return false
        }
        let cx = other.x + mu * other.vx;
        // println!("X: {}",cx);
        if let Some(area) = area {
            if cx < area[0][0] || cx > area[0][1] {
                return false;
            }
        }
        let cy = other.y + mu * other.vy;
        // println!("Y: {}",cy);
        if let Some(area) = area {
            if cy < area[1][0] || cy > area[1][1] {
                return false;
            }
        }
        // let cz = other.z + mu * other.vz;
        // if cz != self.z + lam * self.vz {
        //     return false;
        // }
        true
    }


    fn parallel_and_disj(&self, other: &Self) -> bool {
        if self.vx/other.vx != self.vy/other.vy || self.vx/other.vz != self.vy/other.vz {
            return false;
        }
        let f = (other.x - self.x)/self.vx;
        if self.y + f* self.vy == other.y || self.z + f * self.vz == other.z {
            return false;
        }
        true
    }

    fn find_intersection_point(&self, ep: (f128,f128,f128), d1: (f128,f128,f128), d2: (f128,f128,f128)) -> ((f128,f128,f128), f128) {

        let ((a,d,g),(b,e,h), (c,f,i)) = (d1,d2, (-self.vx, -self.vy, -self.vz));
        let x = self.x - ep.0;
        let y = self.y - ep.1;
        let z = self.z - ep.2;

        let det =
            a * e * i +
            b * f * g +
            c * d * h -
            c * e * g -
            b * d * i -
            a * f * h;

        let (mu, lam, nu) = (
            (
                (e * i - f * h) * x +
                    (c * h - b * i) * y +
                    (b * f - c * e) * z
            )/ det,
            (
                (f * g - d * i) * x +
                    (a * i - c * g) * y +
                    (c * d - a * f) * z
            )/det,
            (
                (d * h - e * g) * x +
                    (b * g - a * h) * y +
                    (a * e - b * d) * z
            )/ det,
        );
        let p = (self.x + nu * self.vx,
                 self.y + nu * self.vy,
                 self.z + nu * self.vz);
        let p1 = (ep.0 + mu * d1.0 + lam * d2.0, ep.1 + mu * d1.1 + lam * d2.1, ep.2 + mu * d1.2 + lam * d2.2);
        println!("pts: {:?}\n{:?}", p, p1);
        (p, nu)
    }
}


fn find_rock_line(particles: &Vec<Particle<f128>>) -> Option<(Particle<f128>, f128, f128)>{

    for i in 0..particles.len() {
        if let Some(j) = ((i+1)..particles.len())
            .into_iter()
            .find(|x| particles[i].parallel_and_disj(
                &particles[*x])) {
                let ep = (particles[i].x,particles[i].y,particles[i].z);
                let d1 = (particles[i].vx,particles[i].vy,particles[i].vz);

                // let dx = particles[j].x - particles[i].x;
                // let d2 = (
                //     f128::from(50),
                //     (particles[j].y - particles[i].y)/dx * f128::from(50),
                //     (particles[j].z - particles[i].z)/dx * f128::from(50),
                // );
                let d2 = (
                    particles[j].x - particles[i].x,

                    (particles[j].y - particles[i].y),
                    (particles[j].z - particles[i].z),
                );
                let mut cnt = 0;
                let mut iii = 0;
                let mut p = Particle::<f128>::default();
                let (mut t0, mut _t1) = (f128::from(0),f128::from(0));
                for ii in 0..particles.len() {
                    if ii== i || ii == j {
                        continue;
                    }
                    if cnt == 0 {
                        let (pnt, t) = particles[ii].find_intersection_point(ep, d1, d2);
                        t0 = t;
                        p.x = pnt.0;
                        p.y = pnt.1;
                        p.z = pnt.2;
                        cnt += 1;
                        iii = ii;
                        continue;
                    } else {
                        let (pnt, t) = particles[ii].find_intersection_point(ep, d1, d2);
                        _t1 = t;
                        p.vx = pnt.0 - p.x;
                        p.vy = pnt.1 - p.y;
                        p.vz = pnt.2 - p.z;
                        println!("times: {t0}, {_t1}, {iii}, {ii}");
                        return Some((p, t0,_t1));

                    }

                }
                break;
            }
    }
    return None;

}


pub fn part2(input: &[[i64; 6]]) -> i128 {
    // Calculations need the range of `i128`.
    let widen = |i: usize| input[i].map(|n| n as i128);
    let [a, b, c, d, e, f] = widen(0);
    let [g, h, i, j, k, l] = widen(1);
    let [m, n, o, p, q, r] = widen(2);

    // Coefficients for the 6 simulataneous linear equations.
    // Columns are px, py, pz, vx, vy, vz of the rock equal to a constant.
    let mut matrix = [
        [0, l - f, e - k, 0, c - i, h - b, e * c - b * f + h * l - k * i],
        [0, r - f, e - q, 0, c - o, n - b, e * c - b * f + n * r - q * o],
        [f - l, 0, j - d, i - c, 0, a - g, a * f - d * c + j * i - g * l],
        [f - r, 0, p - d, o - c, 0, a - m, a * f - d * c + p * o - m * r],
        [k - e, d - j, 0, b - h, g - a, 0, d * b - a * e + g * k - j * h],
        [q - e, d - p, 0, b - n, m - a, 0, d * b - a * e + m * q - p * n],
    ];

    // Use Gaussian elimination to solve for the 6 unknowns.
    // Forward elimination, processing columns from left to right.
    // This will leave a matrix in row echelon form.
    for pivot in 0..6 {
        // Make leading coefficient of each row positive to make subsequent calculations easier.
        for row in &mut matrix[pivot..] {
            if row[pivot] < 0 {
                // Flip signs of each coefficient.
                row.iter_mut().for_each(|n| *n = -*n);
            }
        }

        loop {
            // Reduce by GCD each time otherwise coefficients will overflow even a `i128`.
            for row in &mut matrix[pivot..] {
                let mut factor = 0;

                for &next in row.iter() {
                    if next != 0 {
                        if factor == 0 {
                            factor = next.abs();
                        } else {
                            factor = gcd(factor, next.abs());
                        }
                    }
                }

                row.iter_mut().for_each(|c| *c /= factor);
            }

            let column = matrix.map(|row| row[pivot]);

            // If only one non-zero coefficient remaining in the column then we're done.
            if column[pivot..].iter().filter(|&&c| c > 0).count() == 1 {
                // Move this row into the pivot location
                let index = column.iter().rposition(|&c| c > 0).unwrap();
                matrix.swap(pivot, index);
                break;
            }

            // Find the row with the lowest non-zero leading coefficient.
            let min = *column[pivot..].iter().filter(|&&c| c > 0).min().unwrap();
            let index = column.iter().rposition(|&c| c == min).unwrap();

            // Subtract as many multiples of this minimum row from each other row as possible
            // to shrink the coefficients of our column towards zero.
            for row in pivot..6 {
                if row != index && column[row] != 0 {
                    let factor = column[row] / min;

                    for col in 0..7 {
                        matrix[row][col] -= factor * matrix[index][col];
                    }
                }
            }
        }
    }

    // Back substitution, processing columns from right to left.
    // This will leave the matrix in reduced row echelon form.
    // The solved unknowns are then in the 7th column.
    for pivot in (0..6).rev() {
        // We're explicitly told that the results are integers so integer division is safe
        // and will not mangle result.
        matrix[pivot][6] /= matrix[pivot][pivot];

        for row in 0..pivot {
            matrix[row][6] -= matrix[pivot][6] * matrix[row][pivot];
        }
    }

    // x + y + z
    matrix[0][6] + matrix[1][6] + matrix[2][6]
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
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

    // #[test]
    // fn test_find_intersection_point() {
    //     let p = Particle{x: 0.0, y: 0.0, z: 5.0, vx: 0.0, vy: 0.0, vz: 1.0};
    //     let ip = p.find_intersection_point((1.0,10.0,0.0), (0.0,1.0,0.0), (1.0,0.0,0.0));
    //     println!("{:?}", ip);

    //     let p = Particle{x: 0.1, y: 1.5, z: 5.0, vx: 0.0, vy: 0.0, vz: 23222393439.0};
    //     let ip = p.find_intersection_point((1.0,10.0,0.0), (0.0,1.0,0.0), (1.0,0.0,0.0));
    //     println!("{:?}", ip);

    //     assert!(false);
    // }
}

// mu  = ((y2 - y1) *vx1 /vy1 -x2 + x1)/( vx2 - vy2 * vx1 /vy1 )
/*
 * find x s.t. x_i
 */
