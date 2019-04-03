use std::io::{self, BufRead};
use std::cmp::max;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    lines.next();
    for line in lines {
        let full_num = line;
        let mut curr_num = String::from("");
        let mut max_bits = 0;
        for l in full_num.chars() {
            curr_num.push(l);
            let mut n: i32 = curr_num.parse().unwrap();
            let mut bits = 0;
            while n > 0 {
                bits += n % 2;
                n >>= 1;
            }
            max_bits = max(bits, max_bits);
        }
        println!("{}", max_bits);
    }
}
