use std::collections::HashMap;
use std::io::{self, BufRead};
use std::iter;

fn main() {
    let kb_map: HashMap<char, char> = [
        "``1234567890-=",
        "QQWERTYUIOP[]\\",
        "AASDFGHJKL;'",
        "ZZXCVBNM,./",
    ]
        .iter()
        .map(|&l| {
            l.chars()
                .collect::<Vec<_>>()
                .windows(2)
                .map(|e| (e[1], e[0]))
                .collect::<Vec<_>>()
        }).flat_map(|x| x)
        .chain(iter::once((' ', ' ')))
        .collect();

    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        println!("{}", line.chars().map(|c| kb_map[&c]).collect::<String>());
    }
}
