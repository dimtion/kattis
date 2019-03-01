use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<f32> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        if nums.len() == 5 {
            let x1 = nums[0];
            let y1 = nums[1];
            let x2 = nums[2];
            let y2 = nums[3];
            let p = nums[4];
            println!("{}", ((x1 - x2).abs().powf(p) + (y1 - y2).abs().powf(p)).powf(1./p))
        }
    }
}
