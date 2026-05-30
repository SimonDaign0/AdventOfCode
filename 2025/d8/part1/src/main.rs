use std::collections::HashMap;
const GRP_RNG: usize = 1000;

fn read_input() -> &'static str {
    include_str!("./input.txt")
}
struct Link {
    p1: usize,
    p2: usize,
    dist_sq: u64,
}
impl Link {
    fn new(pts: &[(u64, u64, u64)], p1: usize, p2: usize) -> Self {
        let dist_sq = {
            let dx = pts[p2].0.abs_diff(pts[p1].0);
            let dy = pts[p2].1.abs_diff(pts[p1].1);
            let dz = pts[p2].2.abs_diff(pts[p1].2);
            dx.pow(2) + dy.pow(2) + dz.pow(2)
        };
        Link { p1, p2, dist_sq }
    }
}

fn main() {
    let fc = read_input();
    let pts: Vec<(u64, u64, u64)> = fc
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut parts = l.split(',');
            let x = parts.next().unwrap().trim().parse::<u64>().unwrap();
            let y = parts.next().unwrap().trim().parse::<u64>().unwrap();
            let z = parts.next().unwrap().trim().parse::<u64>().unwrap();
            (x, y, z)
        })
        .collect();

    //Would be better to use BinaryHeap if taking too long
    let mut links: Vec<Link> = vec![];
    for i in 0..pts.len() {
        for j in i + 1..pts.len() {
            links.push(Link::new(&pts, i, j));
        }
    }
    links.sort_by_key(|l| l.dist_sq);

    let mut parents: Vec<usize> = (0..pts.len()).collect();
    // will panic if GRP_RNG > links.len()
    for i in 0..GRP_RNG {
        merge(&mut parents, links[i].p1, links[i].p2);
    }

    let mut map: HashMap<usize, usize> = HashMap::new();
    for i in 0..parents.len() {
        let root = root(&mut parents, i);
        *map.entry(root).or_insert(0) += 1;
    }
    let mut vals: Vec<usize> = map.values().copied().collect();
    let prod: usize = if vals.len() >= 3 {
        vals.select_nth_unstable_by(2, |a, b| b.cmp(a));
        vals[0..3].iter().product()
    } else {
        vals.iter().product() // Multiply whatever groups exist if vals len < 3
    };

    println!("prod: {:?}", prod);
}

fn root(parents: &mut Vec<usize>, pt: usize) -> usize {
    if parents[pt] == pt {
        return pt;
    } else {
        parents[pt] = root(parents, parents[pt]);
        parents[pt]
    }
}

fn merge(parents: &mut Vec<usize>, pt1: usize, pt2: usize) {
    let parent1 = root(parents, pt1);
    let parent2 = root(parents, pt2);
    parents[parent1] = parent2;
}
