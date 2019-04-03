use std::io::{self, BufRead};
use std::string::String;

type Int = i64;

fn gcde(a: Int, b: Int) -> (Int, Int, Int) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (gcd, x, y) = gcde(b % a, a);
    (gcd, y - (b / a) * x, x)
}

fn inverse(a: Int, n: Int) -> Int {
    let (gcd, x, _y) = gcde(a, n);
    if gcd != 1 {
        return -1;
    }
    (x % n + n) % n
}

fn mod_add(a: Int, b: Int, n: Int) -> Int {
    (a + b) % n
}

fn mod_sub(a: Int, b: Int, n: Int) -> Int {
    (((a - b) % n) + n) % n
}

fn mod_mul(a: Int, b: Int, n: Int) -> Int {
    (a * b) % n
}

fn mod_div(a: Int, b: Int, n: Int) -> Int {
    let i = inverse(b, n);
    if i == -1 {
        return -1;
    }
    (a * inverse(b, n)) % n
}

fn get_n_t(line: &String) -> (Int, usize) {
    let nums: Vec<Int> = line
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    (nums[0], nums[1] as usize)
}

fn get_a_o_p(line: &String) -> (Int, char, Int) {
    let tokens: Vec<String> = line
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    (
        tokens[0].parse().unwrap(),
        tokens[1].chars().next().unwrap(),
        tokens[2].parse().unwrap(),
    )
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let (mut n, mut t) = get_n_t(&lines.next().unwrap());
    while n != 0 && t != 0 {
        let (a, op, b) = get_a_o_p(&lines.next().unwrap());
        let res = match op {
            '+' => mod_add(a, b, n),
            '-' => mod_sub(a, b, n),
            '*' => mod_mul(a, b, n),
            '/' => mod_div(a, b, n),
            _ => panic!(),
        };
        println!("{}", res);
        t -= 1;
        if t <= 0 {
            let (n1, t1) = get_n_t(&lines.next().unwrap());
            n = n1;
            t = t1;
        }
    }
}
