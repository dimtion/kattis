use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let nums: Vec<i16> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let r = nums[0];
    let s = nums[1];
    let k = nums[2];
    let fly_matrix: Vec<Vec<bool>> = lines
        .map(|l| {
            let mut v = Vec::new();
            for c in l.unwrap().chars() {
                v.push(match c {
                    '.' => false,
                    '*' => true,
                    _ => false,
                });
            }
            v
        }).collect();
    let mut best_flies = 0;
    let mut best_x = 0;
    let mut best_y = 0;
    for y in 0..(r - k + 1) {
        for x in 0..(s - k + 1) {
            let mut flies = 0;
            for i in 1..(k - 1) {
                for j in 1..(k - 1) {
                    if fly_matrix[(y + j) as usize][(x + i) as usize] {
                        flies += 1;
                    }
                }
            }
            if flies > best_flies {
                best_flies = flies;
                best_x = x as usize;
                best_y = y as usize;
            }
        }
    }
    println!("{}", best_flies);
    for y in 0..r as usize {
        let k = k as usize;
        let mut line = String::new();
        for x in 0..s as usize {
            line.push(match fly_matrix[y][x] {
                _ if x == best_x && y == best_y => '+',
                _ if x == best_x + k - 1 && y == best_y + k - 1 => '+',
                _ if x == best_x && y == best_y + k - 1 => '+',
                _ if x == best_x + k - 1 && y == best_y => '+',
                _ if x == best_x && y > best_y && y < best_y + k - 1 => '|',
                _ if x == best_x + k - 1 && y > best_y && y < best_y + k - 1 => '|',
                _ if x > best_x && x < best_x + k - 1 && y == best_y => '-',
                _ if x > best_x && x < best_x + k - 1 && y == best_y + k - 1 => '-',
                true => '*',
                false => '.',
            });
        }
        println!("{}", line);
    }
}
