use std::fmt::Write;

fn read_input() -> &'static str {
    include_str!("./input.txt")
}

fn handle_range(range: &str) -> u64 {
    let mut accumulator: u64 = 0;
    let (left, right) = range.split_once("-").unwrap();
    let min: u64 = left.parse().unwrap();
    let max: u64 = right.parse().unwrap();

    let mut buf = String::new();
    for nb in min..max + 1 {
        buf.clear();
        write!(buf, "{}", nb).unwrap();
        if has_double_patern(&buf) {
            accumulator += nb;
        }
    }
    accumulator
}

fn has_double_patern(s: &str) -> bool {
    if s.len() % 2 > 0 {
        return false;
    }
    let half = s.len() / 2;
    s[..half] == s[half..]
}

fn main() {
    let fc = read_input();
    let ranges: Vec<&str> = fc.trim().split(",").collect();
    let mut accumulator = 0;
    for range in ranges {
        accumulator += handle_range(range);
    }
    println!("inv ID sum: {}", accumulator);
}
