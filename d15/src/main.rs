use std::collections::HashMap;


fn main() {

    let res = include_bytes!("../input.txt")
        .split(|&x| x == b',')
        .map(|x| hash(x) as u32)
        .sum::<u32>();
    println!("Part 1: {res}");

    let mut boxes = Vec::<LenseBox>::with_capacity(256);
    for _ in 0..256 {
        boxes.push(LenseBox::new());
    }


    include_bytes!("../input.txt")
        .split(|&x| x == b',')
        .for_each(|x| {
            let mut d = x.split(|c| *c == b'=' || *c == b'-');
            let l = d.next().unwrap();
            let label = l.to_vec();
            let b = &mut boxes.get_mut(hash(l) as usize).unwrap().slots;

            match d.next() {
                Some([10]) => {},
                Some([foc]) => {
                    if let Some(lens) = b.get_mut(&label) {
                        lens.0 = *foc - 48;
                    } else {
                        b.insert(label, (*foc - 48, b.len()));
                    }
                },
                Some([]) => {
                    let pos = match b.get(&label) {
                        Some((_,p)) => Some(*p),
                        _ => None,
                    };
                    if let Some(pos) = pos {
                        b.remove(&label);
                        b.iter_mut().for_each(|(_,(_,p))| {
                            if *p > pos {
                                *p -= 1;
                            }
                        })
                    }
                },
                None => {
                    unreachable!("{}, {:?}", std::str::from_utf8(x).unwrap(), x);
                },
                Some(x) => {
                    println!("{}, {:?}", std::str::from_utf8(x).unwrap(), x);
                    unreachable!()
                },
            }
        });

    let mut cnt = 0;
    let res = boxes.into_iter().map(|b| {
        cnt += 1;
        if b.slots.len() > 0 {
            cnt * b.slots.iter().fold(0, |a,(_, (f,p))| {
                a + *f as u32 * (*p as u32 + 1)
            })
        } else {
            0
        }
    })
        .sum::<u32>();

    println!("Part 2: {res}");
}

#[allow(arithmetic_overflow)]
fn hash(x: &[u8]) -> u8 {
    x.into_iter().filter(|&&x| x != 10).fold(0, |a, i| {
        let s = a.wrapping_add(*i);
        (s << 4).wrapping_add(s)

    })
}


#[derive(Clone, Debug)]
struct LenseBox {
    slots: HashMap<Vec<u8>, (u8,usize)>
}

impl LenseBox {
    fn new() -> Self {
        Self { slots: HashMap::<Vec<u8>, (u8,usize)>::new() }
    }
}


// impl Copy for LensBox {}
