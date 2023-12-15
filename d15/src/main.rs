use std::io::Read;



fn main() {

    let res = include_bytes!("../input.txt")
        .split(|&x| x == b',')
        .map(|x| x.into_iter().filter(|&&x| x != 10).fold(0, |a, i| {
            ((a + *i as u32) * 17) % 256
        }))
        .sum::<u32>();
    println!("Part 1: {res}");
}
