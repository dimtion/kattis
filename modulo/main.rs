use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let divisor = 42;
    let mut distincts = HashSet::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let num: i16 = line.parse().unwrap();
        distincts.insert(num % divisor);
    }
    println!("{}", distincts.len());
}
