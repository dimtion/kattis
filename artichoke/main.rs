use std::io::{self, BufRead};

type Float = f32;

#[derive(Debug, Clone, Copy)]
struct MaxMin {
    max: Float,
    min: Float,
}

impl MaxMin {
    #[inline]
    fn score(&self) -> Float {
        self.max - self.min
    }
}

fn main() {
    let stdin = io::stdin();
    let nums: Vec<Float> = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let p = nums[0];
    let a = nums[1];
    let b = nums[2];
    let c = nums[3];
    let d = nums[4];
    let n = nums[5] as usize;
    let mut points = Vec::new();
    for i in 1..(n+1) {
        let k = i as Float;
        // The price is simplified, because we do only need relative differences
        let price = p * ((a * k + b).sin() + (c * k + d).cos());
        points.push(price);
    }

    let mut best = MaxMin {
        max: points[0],
        min: points[0],
    };
    let mut current = best;
    for i in 0..n {
        if points[i] > current.max {
            if best.score() < current.score() {
                best = current;
            }
            current = MaxMin {
                max: points[i],
                min: points[i],
            };
        } else if points[i] < current.min {
            current.min = points[i];
        }
    }
    if best.score() < current.score() {
        best = current;
    }
    println!("{:.6}", best.score());
}
