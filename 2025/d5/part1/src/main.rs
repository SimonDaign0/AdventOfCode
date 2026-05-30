use std::time::Instant;

fn read_input() -> &'static str {
    include_str!("./input.txt")
}

fn main() {
    let fc = read_input().trim_end();
    let mut fresh_count = 0;
    let (rngs, ids) = fc.split_once("\n\n").unwrap();
    let rngs: Vec<(u64, u64)> = rngs
        .split("\n")
        .map(|s| s.split_once("-").unwrap())
        .map(|(min, max)| (min.parse::<u64>().unwrap(), max.parse::<u64>().unwrap()))
        .collect();
    let ids: Vec<u64> = ids
        .split("\n")
        .map(|id| id.parse::<u64>().unwrap())
        .collect();
    let start = Instant::now();
    //Brute force
    for id in ids {
        let mut is_fresh = false;
        for rng in &rngs {
            if id >= rng.0 && id <= rng.1 {
                is_fresh = true;
                break;
            }
        }
        if is_fresh {
            fresh_count += 1;
        }
    }
    let end = start.elapsed();
    println!("time: {:?}", end);
    println!("fresh count: {}", fresh_count);
}
