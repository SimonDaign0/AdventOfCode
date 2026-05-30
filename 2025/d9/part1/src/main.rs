fn read_input() -> &'static str {
    include_str!("./input.txt")
}

fn main() {
    let fc = read_input();

    let tiles: Vec<(u64, u64)> = fc
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let s = l.split_once(',').unwrap();
            (s.0.parse::<u64>().unwrap(), s.1.parse::<u64>().unwrap())
        })
        .collect();

    let mut max_surface = 0;
    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            let current_surface = surf(tiles[i], tiles[j]);
            max_surface = max_surface.max(current_surface);
        }
    }
    println!("Largest surface: {}", max_surface);
}

fn surf(t1: (u64, u64), t2: (u64, u64)) -> u64 {
    let dx = t1.1.abs_diff(t2.1) + 1;
    let dy = t1.0.abs_diff(t2.0) + 1;
    dx * dy
}
