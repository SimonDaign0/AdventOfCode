//28 - 2
use std::collections::HashSet;
fn read_input() -> &'static str {
    include_str!("./input.txt")
}

fn main() {
    let fc = read_input().trim_end();
    let rows: Vec<&str> = fc.split('\n').collect();
    let start = rows[0].find('S').unwrap();
    let mut set: HashSet<usize> = HashSet::new();
    set.insert(start);
    let mut splits = 0;
    for r in rows {
        let mut split_beams: Vec<usize> = vec![];
        for beam in set.iter() {
            if r.chars().nth(*beam) == Some('^') {
                split_beams.push(beam - 1);
                split_beams.push(beam + 1);
                splits += 1;
            } else {
                split_beams.push(*beam);
            }
        }
        set.clear();
        for b in &split_beams {
            set.insert(*b);
        }
    }
    println!("split count: {}", splits);
}
