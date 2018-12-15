use std::io::{self, BufRead};
use std::collections::BinaryHeap;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let _num_items: i32 = lines.next().unwrap()
        .split_whitespace().next().unwrap()
        .parse().unwrap();

    let items: Vec<i32> = lines.map(|line| line.parse().unwrap()).collect();

    let mut i = 0;
    let mut done = false;

    while !done {
        i += 1;
        let mut modulated_heap: BinaryHeap<i32> = items.iter().map(|el| el % i).collect();
        
        let mut old_value = i;
        let mut eq = false;
        while !modulated_heap.is_empty() {
            let el = modulated_heap.pop().unwrap();
            // eprintln!("i: {}, o: {}, n: {}", i, old_value, el);
            if el == old_value {
                eq = true;
                break;
            }
            old_value = el;
        }
        done = !eq;
    }
    println!("{}", i);
}
