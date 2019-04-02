use std::io::{self, BufRead};

struct SortedVec(Vec<f32>);

impl SortedVec {
    #[inline]
    pub fn push(&mut self, value: f32) {
        let mut min = 0;
        let mut max = self.len();
        while min < max {
            let i = (max + min) / 2;
            if value < self[i] {
                max = i;
            } else {
                min = i + 1;
            }
        }
        self.insert(min, value);
    }

    pub fn len(&mut self) -> usize {
        self.0.len()
    }
    pub fn insert(&mut self, index: usize, value: f32) {
        self.0.insert(index, value);
    }

}

impl std::ops::Index<usize> for SortedVec {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        &self.0[i]
    }
}


fn main() {
    let stdin = io::stdin();
    let mut distances = SortedVec(Vec::new());
    let mut lines = stdin.lock().lines();
    let nums: Vec<f32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let xc = nums[0];
    let yc = nums[1];

    for line in lines.map(|l| l.unwrap()) {
        let nums: Vec<f32> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let x = nums[0];
        let y = nums[1];
        let z = nums[2];
        let d2 = ((x - xc) * (x - xc) + (y - yc) * (y - yc)).sqrt();
        distances.push((d2 - z).max(0.0));
    }
    let mut max_pow = 0.;
    let mut max_n = 0;

    const M: usize = 2;

    while max_n < distances.len() {
        let mut j = max_n;
        while j < distances.len() && distances[j] == distances[max_n] {
            j += 1;
        }
        if j > M {
            max_pow = distances[max_n];
            break;
        }
        max_n = j;
    }

    println!("{}", max_pow.floor());
}
