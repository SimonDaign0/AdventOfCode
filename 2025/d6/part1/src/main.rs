use std::iter::{Product, Sum};

fn read_input() -> &'static str {
    include_str!("./input.txt")
}

fn main() {
    let fc = read_input().trim_end();
    let split_idx = fc.find('*').unwrap();
    let (nb_section, ops_section) = (&fc[..split_idx], &fc[split_idx..]);

    let nbs_grid: Vec<Vec<u64>> = nb_section
        .split_terminator('\n')
        .map(|s| {
            s.split_whitespace()
                .map(|nb| nb.parse::<u64>().unwrap())
                .collect()
        })
        .collect();
    let ops: Vec<char> = ops_section
        .split_whitespace()
        .map(|s| s.parse::<char>().unwrap())
        .collect();

    let mut total: u64 = 0;
    for col in 0..nbs_grid[0].len() {
        let operator = ops[col];
        let mut col_nbs: Vec<u64> = vec![];

        for r in &nbs_grid {
            col_nbs.push(r[col]);
        }

        total += operate_on(col_nbs, operator);
    }
    println!("total : {}", total)
}

fn operate_on(col: Vec<u64>, op: char) -> u64 {
    match op {
        '+' => return Sum::sum(col.iter()),
        '*' => {
            return Product::product(col.iter());
        }
        other => {
            println!("underfined op: {}", other);
            panic!()
        }
    }
}
