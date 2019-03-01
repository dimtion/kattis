use std::io::{self, BufRead};
use std::cmp::max;

fn main() {
    let stdin = io::stdin();
    let mut a = 0;
    let mut b = 1;
    let mut c = 2;
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<i64> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        a = nums[0];
        b = nums[1];
        c = nums[2];
    }
    let mut n = 0;
    while max(b-a, c-b) > 1 {
        if b-a > c-b {
            c = b;
            b = b - 1;
        } else {
            a = b;
            b = c - 1;
        }
        // println!("{} {} {}", a, b, c);
        n += 1;
    }

    println!("{}", n);
}
