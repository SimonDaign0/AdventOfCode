fn read_input() -> &'static str {
    include_str!("./input.txt")
}

fn main() {
    let fc = read_input();

    let total: u32 = fc
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| solve_machine(line))
        .sum();
    println!("total presses: {total}");
}

fn solve_machine(line: &str) -> u32 {
    let mut tokens = line.split_ascii_whitespace();
    let target_str = tokens
        .next()
        .unwrap()
        .trim_matches(|c| c == '[' || c == ']');
    let target_mask =
        target_str.bytes().enumerate().fold(
            0_u16,
            |mask, (i, b)| if b == b'#' { mask | (1 << i) } else { mask },
        );
    let btn_masks: Vec<u16> = tokens
        .take_while(|token| token.starts_with('('))
        .map(|token| {
            token
                .trim_matches(|c| c == '(' || c == ')')
                .split(',')
                .map(|num| num.parse::<u16>().unwrap())
                .fold(0_u16, |mask, idx| mask | (1 << idx))
        })
        .collect();

    let mut min_count = u32::MAX;
    let n = btn_masks.len();
    for mask in 0..(1 << n) {
        let mut current_state = 0_u16;
        for (i, btn_mask) in btn_masks.iter().enumerate() {
            if mask & (1 << i) != 0 {
                current_state ^= btn_mask;
            }
        }
        if current_state == target_mask {
            let activation_count = u16::count_ones(mask);
            if activation_count < min_count {
                min_count = activation_count
            }
        }
    }
    min_count
}
