use std::fs::File;
use std::io::{self, BufRead, Read};
use std::collections::HashMap;
mod dikstra;

fn main() {

    let grid = include_bytes!("../input.txt")
        .split(|b| *b == b'\n')
        .filter(|x| x.len() > 1)
        .collect::<Vec<_>>();

    let res = dikstra::dijkstra(&grid, 1, 3);
    println!("Part 1: {}", res);
    let res = dikstra::dijkstra(&grid, 4, 10);
    println!("Part 2: {}", res);
}
