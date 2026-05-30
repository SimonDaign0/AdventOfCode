fn read_input() -> &'static str {
    include_str!("./input.txt")
}

fn main() {
    let fc = read_input();
    let mut nbs_lines: Vec<&str> = fc.lines().filter(|l| !l.is_empty()).collect();
    let ops_line: &str = nbs_lines.pop().expect("corrupted input file");

    let op_lens: Vec<usize> = op_lengths(ops_line);
    let ops: Vec<char> = ops_line
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect();

    let nbs_grid: Vec<Vec<&str>> = nbs_lines
        .iter()
        .map(|line| as_chunks(line, &op_lens))
        .collect();

    let mut total: u64 = 0;
    for col in 0..nbs_grid[0].len() {
        let mut col_nbs: Vec<&str> = vec![];
        for row in 0..nbs_grid.len() {
            col_nbs.push(nbs_grid[row][col]);
        }
        total += solve_column(ops[col], op_lens[col], &col_nbs);
    }
    println!("total: {total}");
}

fn as_chunks<'a>(line: &'a str, chunk_sizes: &[usize]) -> Vec<&'a str> {
    let mut vec = vec![];
    let mut start = 0;
    for size in chunk_sizes {
        let chunk = &line[start..start + *size];
        vec.push(chunk);
        start += size;
    }
    vec
}
// Ascii only
fn solve_column(op: char, op_len: usize, col: &[&str]) -> u64 {
    let mut total = { if op == '+' { 0 } else { 1 } };
    for i in 0..op_len {
        //chars as bytes
        let mut char_digits: Vec<u8> = Vec::with_capacity(op_len);
        for &chars in col {
            if let Some(&byte) = chars.as_bytes().get(i).filter(|&&b| b != b' ') {
                char_digits.push(byte);
            }
        }
        let mut accumulator = 0;
        for byte in char_digits {
            accumulator *= 10;
            accumulator += (byte - b'0') as u64;
        }

        if accumulator != 0 {
            match op {
                '+' => total += accumulator,
                _mul => total *= accumulator,
            }
        }
    }
    total
}

fn op_lengths(ops_line: &str) -> Vec<usize> {
    let mut vec: Vec<usize> = vec![];
    let mut start = 0;
    //basicly .enumerate but skips idx 0
    for (i, c) in ops_line.char_indices().skip(1) {
        if !c.is_ascii_whitespace() {
            vec.push(i - start);
            start = i;
        }
    }
    vec.push(ops_line.len() - start);
    vec
}
