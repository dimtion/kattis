use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let nums: Vec<i16> = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();;
    println!("{}", nums[1] * 2 - nums[0]);
}
