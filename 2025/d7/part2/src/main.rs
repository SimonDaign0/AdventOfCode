//07
use std::collections::HashMap;
fn read_input() -> &'static str {
    include_str!("./input.txt")
}

fn main() {
    let fc = read_input().trim_end();
    let rows: Vec<&str> = fc.split('\n').collect();
    let start = rows[0].find('S').unwrap();
    let init = (2_usize, start);
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();
    let timelines = scan(init, &rows, &mut map);
    println!("timeline count: {}", timelines);
}

fn scan(
    timeline: (usize, usize),
    rows: &Vec<&str>,
    map: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let result = map.get(&timeline);
    if let Some(count) = result {
        return *count;
    } else {
        let mut split_count = |start_pos: (usize, usize)| {
            let mut curr_pos = start_pos;
            while curr_pos.0 < rows.len() - 1 && rows[curr_pos.0].as_bytes()[curr_pos.1] != b'^' {
                curr_pos.0 += 1;
            }
            let result = if rows[curr_pos.0].as_bytes()[curr_pos.1] == b'^' {
                scan(curr_pos, rows, map)
            } else {
                1
            };
            map.insert(curr_pos, result);
            result
        };
        let left = split_count((timeline.0 + 1, timeline.1 - 1));
        let right = split_count((timeline.0 + 1, timeline.1 + 1));
        return left + right;
    }
}
