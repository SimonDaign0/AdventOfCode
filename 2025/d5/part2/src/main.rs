use std::time::Instant;

fn read_input() -> &'static str {
    include_str!("./input.txt")
}

fn main() {
    let fc = read_input().trim_end();
    let (rngs, _) = fc.split_once("\n\n").unwrap();
    let rngs: Vec<(u64, u64)> = rngs
        .split("\n")
        .map(|s| s.split_once("-").unwrap())
        .map(|(min, max)| (min.parse::<u64>().unwrap(), max.parse::<u64>().unwrap()))
        .collect();

    let start = Instant::now();
    let mut merges: Vec<(u64, u64)> = vec![];
    for rng in &rngs {
        if let Some(overlapping) = merges.iter_mut().find(|r| is_overlapping(r, rng)) {
            merge(overlapping, rng);
        } else {
            merges.push(*rng);
        }
    }
    merges.sort_by_key(|&(min, _)| min);
    let mut i = 0;
    while i + 1 < merges.len() {
        if is_overlapping(&merges[i], &merges[i + 1]) {
            let other = merges.remove(i + 1);
            merge(&mut merges[i], &other);
        } else {
            i += 1;
        }
    }
    let fresh_count: u64 = merges.iter().map(|(min, max)| max - min + 1).sum();
    let end = start.elapsed();
    println!("time: {:?}", end);
    println!("Fresh count: {}", fresh_count);
}

fn is_overlapping(i: &(u64, u64), j: &(u64, u64)) -> bool {
    j.0 <= i.1 && j.1 >= i.0
}

fn merge(taken: &mut (u64, u64), other: &(u64, u64)) {
    let merged_min = taken.0.min(other.0);
    let merged_max = taken.1.max(other.1);
    *taken = (merged_min, merged_max);
}
