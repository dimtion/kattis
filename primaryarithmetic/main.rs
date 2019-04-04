use std::io::{self, BufRead};

type Int = i64;

fn calc_carries(a: Int, b: Int) -> Int {
    let mut a1 = a;
    let mut b1 = b;
    let mut n_carr = 0;
    let mut carr = 0;
    while a1 > 0 || b1 > 0 {
        carr = ((a1 % 10) + (b1 % 10) + carr) / 10;
        if carr > 0 {
            n_carr += 1
        }
        a1 = a1 / 10;
        b1 = b1 / 10;
    }
    n_carr
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<Int> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let a = nums[0];
        let b = nums[1];
        if a == 0 && b == 0 {
            break;
        }
        match calc_carries(a, b) {
            0 => println!("No carry operation."),
            1 => println!("1 carry operation."),
            x => println!("{} carry operations.", x),
        }
    }
}
