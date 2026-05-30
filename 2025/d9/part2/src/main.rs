use std::time::Instant;

fn read_input() -> &'static str {
    include_str!("./input.txt")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Tile {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy, Debug)]
struct Rect {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Rect {
    fn from(t1: Tile, t2: Tile) -> Self {
        Self {
            min_x: t1.x.min(t2.x),
            max_x: t1.x.max(t2.x),
            min_y: t1.y.min(t2.y),
            max_y: t1.y.max(t2.y),
        }
    }

    fn surf(&self) -> u64 {
        ((self.max_x - self.min_x) as u64 + 1) * ((self.max_y - self.min_y) as u64 + 1)
    }
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    p1: Tile,
    p2: Tile,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Edge {
    fn from(p1: Tile, p2: Tile) -> Self {
        Self {
            p1,
            p2,
            min_x: p1.x.min(p2.x),
            max_x: p1.x.max(p2.x),
            min_y: p1.y.min(p2.y),
            max_y: p1.y.max(p2.y),
        }
    }

    fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }
}

// Raycasts using a fractional y-coordinate (y + 0.5) to cleanly avoid corner vertex collisions
fn is_point_inside(x: f64, y: f64, edges: &[Edge]) -> bool {
    let mut inside = false;
    for edge in edges {
        if edge.is_vertical() {
            let edge_min_y = edge.min_y as f64;
            let edge_max_y = edge.max_y as f64;
            let edge_x = edge.p1.x as f64;

            if edge_min_y <= y && edge_max_y > y {
                if x > edge_x {
                    inside = !inside;
                }
            }
        }
    }
    inside
}

fn main() {
    let start_time = Instant::now();
    let fc = read_input();
    let tiles: Vec<Tile> = fc
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let s = line.split_once(',').unwrap();
            let x = s.0.trim().parse::<i64>().unwrap();
            let y = s.1.trim().parse::<i64>().unwrap();
            Tile { x, y }
        })
        .collect();

    let n = tiles.len();
    let mut edges = Vec::with_capacity(n);
    for i in 0..n {
        edges.push(Edge::from(tiles[i], tiles[(i + 1) % n]));
    }

    let mut max_surf = 0;

    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            let t1 = tiles[i];
            let t2 = tiles[j];

            if t1.x == t2.x || t1.y == t2.y {
                continue;
            }

            let rect = Rect::from(t1, t2);
            let current_surf = rect.surf();
            if current_surf <= max_surf {
                continue;
            }

            // Test the interior center of the first tile row/column to see if it starts inside the polygon.
            let test_x = rect.min_x as f64 + 0.5;
            let test_y = rect.min_y as f64 + 0.5;

            if !is_point_inside(test_x, test_y, &edges) {
                continue;
            }

            // Fast Structural Validation
            let mut valid = true;
            for edge in &edges {
                // If the edge does not cross into the internal bounding box space, it's fine
                if edge.max_x <= rect.min_x
                    || edge.min_x >= rect.max_x
                    || edge.min_y <= rect.min_y
                    || edge.max_y >= rect.max_y
                {
                    continue;
                }

                if edge.is_vertical() {
                    // If a vertical wall sits completely inside our x boundaries, it splits our rectangle
                    if edge.p1.x > rect.min_x && edge.p1.x < rect.max_x {
                        valid = false;
                        break;
                    }
                } else {
                    // If a horizontal wall sits completely inside our y boundaries, it splits our rectangle
                    if edge.p1.y > rect.min_y && edge.p1.y < rect.max_y {
                        valid = false;
                        break;
                    }
                }
            }

            if valid {
                max_surf = current_surf;
            }
        }
    }

    println!("Max Surf Area: {}", max_surf);
    println!("Completed in: {:?}", start_time.elapsed());
}
