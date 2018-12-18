use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().map(|l| l.unwrap()).next();
    let mut final_line = String::new();
    for l in line.unwrap().chars() {
        match l {
            '<' => {
                final_line.pop();
            }
            _ => {
                final_line.push(l);
            }
        }
    }
    println!("{}", final_line);
}
