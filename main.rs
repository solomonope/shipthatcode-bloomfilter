use std::io::{self, BufRead};

// TODO (idea): implement per the lesson description.

fn parse(command: &str) -> Result<Command, String> {
    let parts: Vec<&str> = command.split(' ').collect();
    match parts[0] {
        "ADD" => Ok(Command::ADD {
            key: String::from(parts[1]),
        }),
        "CHECK" => Ok(Command::CHECK {
            key: String::from(parts[1]),
        }),
        "INIT" => {
            let n = parts[1].parse::<u32>().unwrap_or(0);
            let fp = parts[2].parse::<f64>().unwrap_or(0.0);

            Ok(Command::INIT { n, fp })
        }
        "STATS" => Ok(Command::STATS),
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
                _ => {
                    println!("UNHandled")
                }
            },
            Err(_) => {
                println!("TODO");
            }
        }
    }
}

enum Command {
    ADD { key: String },
    CHECK { key: String },
    INIT { n: u32, fp: f64 },
    STATS,
}
struct BloomFilter {
    v: Vec<u8>,
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

        let k = ((m as f64 / n as f64) * 2.0_f64.ln()).round() as u32;
        let c = 0;
        Self {
            v: vec![0; m as usize],
            m,
            k,
            n,
            c,
            fp,
        }
    }
}
