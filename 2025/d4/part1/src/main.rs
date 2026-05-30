fn read_input() -> &'static str {
    include_str!("./input.txt")
}

fn main() {
    let fc = read_input();
    let grid: Vec<Vec<char>> = fc.lines().map(|l| l.chars().collect()).collect();
    //helper closure to check if option char idx at option row idx == '@'
    let is_paper_at = |r: Option<usize>, c: Option<usize>| {
        r.and_then(|r| c.and_then(|c| grid.get(r)?.get(c)))
            .is_some_and(|&ch| ch == '@')
    };
    let mut accessed = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '.' {
                continue;
            }
            let above = row.checked_sub(1);
            let below = row.checked_add(1);
            //checks all Some 8 around
            let adj_count = [
                (above, col.checked_sub(1)),
                (above, Some(col)),
                (above, col.checked_add(1)),
                (Some(row), col.checked_sub(1)),
                (Some(row), col.checked_add(1)),
                (below, col.checked_sub(1)),
                (below, Some(col)),
                (below, col.checked_add(1)),
            ]
            .iter()
            .filter(|&&(r, c)| is_paper_at(r, c))
            .count();
            if adj_count < 4 {
                accessed += 1;
            }
        }
    }
    println!("amt accessed, {}", accessed);
}
