use std::io::{self, BufRead};

// TODO (idea): implement per the lesson description.

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() {
            continue;
        }
        println!("TODO");
    }
}

/*
 * m =
 *
 */
struct BloomFilter {
    m: u32,
    k: u32,
    n: u32,
    c: u32,
    fp: f64,
}

impl BloomFilter {
    fn new(n: u32, fp: f64) -> Self {
        // m = ceil( -n * ln(p) / (ln(2) ^ 2))
        let m = 0;
        // k = round( (m/n) * ln(2))
        let k = 0;
        let c = 0;
        Self { m, k, n, c, fp }
    }
}
