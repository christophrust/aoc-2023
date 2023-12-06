fn main() {

    let times = vec![35,93,73,66];
    let record = vec![212,2060,1201,1044];
    // let times = vec![7,15,30];
    // let record = vec![9, 40, 200];

    let res: i32 = times
        .into_iter()
        .zip(record)
        .map(|(t, d)| {
            // println!("{:?}", (t,d));
            let mut s = t/2;
            // println!("{}",s);
            if s * (t - s) <= d {
                return 0
            }
            while s * (t - s) > d {
                s -=1;
            }
            //println!("{}",s);
            t + 1 - 2 * (s + 1)
        })
        .product();
    println!("{}", res);

    let t = 35937366_u64;
    let d = 212206012011044_u64;
    // let t = 71530_u64;
    // let d = 940200_u64;

    let mut s = t/2;
    while s * (t - s) > d {
        s /=2;
    }
    println!("{}",s);
    while s * (t - s) <= d {
        s += 1;
    }
    println!("{}", t + 1 - 2 * s)
}
