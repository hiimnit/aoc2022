use std::env;
use std::fs;

// https://doc.rust-lang.org/std/collections/binary_heap/index.html
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct Map {
    start: usize,
    end: usize,
    rows: usize,
    cols: usize,
    map: Vec<Node>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    height: u8,
    best_cost: usize,
    edges: Vec<usize>,
}

impl Node {
    pub fn new(height: u8) -> Self {
        Self {
            height,
            best_cost: usize::MAX,
            edges: vec![],
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.best_cost.cmp(&self.best_cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_byte(map: &Vec<Vec<Node>>, byte: u8) -> Option<(usize, usize)> {
    map.iter().enumerate().find_map(|(x, row)| {
        let position = row.iter().position(|e| e.height == byte);

        if let Some(y) = position {
            return Some((x, y));
        }
        None
    })
}

impl Map {
    pub fn new(input: &String) -> Self {
        let mut map = input
            .lines()
            .map(|e| {
                e.chars()
                    .into_iter()
                    .map(|e| Node::new(e as u8))
                    .collect::<Vec<Node>>()
            })
            .collect::<Vec<Vec<Node>>>();

        let start = find_byte(&map, b'S').unwrap();
        let end = find_byte(&map, b'E').unwrap();

        map[start.0][start.1].height = b'a' as u8;
        map[end.0][end.1].height = b'z' as u8;

        let rows = map.len();
        let cols = map[0].len();

        for row in 0..rows {
            for col in 0..cols {
                let candidates: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
                for candidate in candidates {
                    let (n1, n2) = ((row as i32 + candidate.0), (col as i32 + candidate.1));

                    if n1 >= 0
                        && n1 < rows as i32
                        && n2 >= 0
                        && n2 < cols as i32
                        && map[row][col].height + 1 >= map[n1 as usize][n2 as usize].height
                    {
                        map[row][col].edges.push(n1 as usize * cols + n2 as usize);
                    }
                }
            }
        }

        Self {
            map: map.into_iter().flatten().collect::<Vec<Node>>(),
            rows,
            cols,
            start: start.0 * cols + start.1,
            end: end.0 * cols + end.1,
        }
    }

    pub fn reset(self: &mut Self) {
        for position in self.map.iter_mut() {
            position.best_cost = usize::MAX;
        }
    }

    pub fn solve(self: &mut Self) -> Option<usize> {
        println!("solving for start {}", self.start);

        self.map[self.start].best_cost = 0;

        let mut heap = BinaryHeap::new();
        heap.push(self.start);

        while let Some(node) = heap.pop() {
            if node == self.end {
                return Some(self.map[node].best_cost);
            }

            for i in self.map[node].edges.clone() {
                let new_cost = self.map[node].best_cost + 1;

                if new_cost < self.map[i].best_cost {
                    self.map[i].best_cost = new_cost;
                    heap.push(i);
                }
            }
        }

        None
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_path = args
        .get(1)
        .expect("Pass in input file path as the first argument!");

    let content = fs::read_to_string(file_path).expect("Could not read the input file!");
    let mut map = Map::new(&content);

    println!("part 1: {}", map.solve().unwrap());

    // input only has few 'b' heights - all in the second column => we only need to test 'a' heights in the first column

    let indices = (0..map.rows).map(|e| e * map.cols).collect::<Vec<usize>>();
    let mut results: Vec<usize> = indices
        .iter()
        .map(|e| {
            map.reset();
            map.start = *e;
            map.solve().unwrap()
        })
        .collect();
    results.sort();

    println!("part 2: {}", results.first().unwrap());
}
