use rand::{thread_rng, Rng};

fn main() {
    let N = 100;
    let I = 35;

    let mut rng = thread_rng();

    let mut generated: Vec<i32>;
    for _ in 0..N {
        let mut rnd: i32 = rng.gen_range(0, I);
        if !generated.contains(&rnd) {
            generated.push(rnd)
        }
    }

    println!("{}", generated.len());
    for e in generated.iter() {
        println!("{}",  e * I);
    }
}
