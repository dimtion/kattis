use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let mut stack: HashMap<String, i32> = HashMap::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        
        if tokens[0] == "define" && tokens.len() == 3 {
            let t = tokens[2].to_string();
            let v = tokens[1];
            stack.insert(t, v.clone().parse().unwrap());
        } else if tokens[0] == "eval" && tokens.len() == 4 {
            let val1 = if stack.contains_key(tokens[1]) {
                stack[tokens[1]]
            } else {
                match tokens[1].parse() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("undefined");
                        continue;
                    },
                }
            };
            let val2 = if stack.contains_key(tokens[3]) {
                stack[tokens[3]]
            } else {
                match tokens[3].parse() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("undefined");
                        continue;
                    },
                }
            };
            match tokens[2] {
                ">" => println!("{}", val1 > val2),
                "<" => println!("{}", val1 < val2),
                "=" => println!("{}", val1 == val2),
                _   => println!("undefined"),
            }
        } else {
            println!("undefined");
        }
    }
}


