use std::io::{self, BufRead};

// TODO (idea): implement per the lesson description.

fn parse(command: &str) -> Result<Command, String> {
    let parts: Vec<&str> = command.split(' ').collect();
    match parts[0] {
        "INIT" => {
            let n = parts[1].parse::<u32>().unwrap_or(0);
            let fp = parts[2].parse::<f64>().unwrap_or(0.0);

            Ok(Command::INIT { n, fp })
        }
        _ => Err(String::from("Error")),
    }
}
fn main() {
    let mut bf: BloomFilter;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() {
            continue;
        }

        let command = parse(&l);

        match command {
            Ok(c) => match c {
                Command::INIT { n, fp } => {
                    bf = BloomFilter::new(n, fp);
                    println!("OK m={} k={}", bf.m, bf.k);
                }
                Command::UKNOWN => {
                    println!("UKNOWN");
                }
            },
            _ => {
                println!("TODO");
            }
        }
    }
}

enum Command {
    INIT { n: u32, fp: f64 },
    UKNOWN,
}
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
        let en = -1 * n as i32;

        let m = ((en as f64 * fp.ln()) / (2_f64.ln().powi(2))).ceil() as u32;
        // k = round( (m/n) * ln(2))
        //  let k = ((m/n as f64) * 2.0_f64.ln()).round();
        let k = ((m / n) as f64 * 2.0_f64.ln()).round() as u32;
        let c = 0;
        Self { m, k, n, c, fp }
    }
}
