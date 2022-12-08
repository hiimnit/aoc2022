use std::env;
use std::fs;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_path = args
        .get(1)
        .expect("Pass in input file path as the first argument!");

    let content = fs::read_to_string(file_path).expect("Could not read the input file!");

    let trees = content
        .lines()
        .into_iter()
        .map(|e| e.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut grid: Vec<Vec<usize>> = vec![];

    for i in 0..trees.len() {
        if grid.len() == i {
            grid.push(vec![]);
        }

        let mut max = (b'0' - 1) as char;
        for j in 0..trees[i].len() {
            if grid[i].len() == j {
                grid[i].push(0);
            }
            if trees[i][j] > max {
                max = trees[i][j];
                grid[i][j] += 1;
            }
        }

        let mut max = (b'0' - 1) as char;
        for j in (0..trees[i].len()).rev() {
            if trees[i][j] > max {
                max = trees[i][j];
                grid[i][j] += 1;
            }
        }
    }

    for j in 0..trees[0].len() {
        let mut max = (b'0' - 1) as char;
        for i in 0..trees.len() {
            if trees[i][j] > max {
                max = trees[i][j];
                grid[i][j] += 1;
            }
        }

        let mut max = (b'0' - 1) as char;
        for i in (0..trees.len()).rev() {
            if trees[i][j] > max {
                max = trees[i][j];
                grid[i][j] += 1;
            }
        }
    }

    let result = grid.iter().fold(0, |acc, e| {
        acc + (e.len() - e.iter().filter(|e| **e == 0).count())
    });
    println!("part 1: {}", result);

    let mut max = 0;
    for (i, line) in trees.iter().enumerate() {
        for (j, tree) in line.iter().enumerate() {
            max = std::cmp::max(max, score_tree(i, j, tree, &trees));
        }
    }

    println!("part 2: {}", max);
}

fn score_tree(row: usize, col: usize, tree: &char, trees: &Vec<Vec<char>>) -> i32 {
    vec![
        score_range((0..col).rev(), |i| trees[row][i] >= *tree),
        score_range((col + 1)..trees[row].len(), |i| trees[row][i] >= *tree),
        score_range((0..row).rev(), |i| trees[i][col] >= *tree),
        score_range((row + 1)..trees.len(), |i| trees[i][col] >= *tree),
    ]
    .iter()
    .product()
}

fn score_range<F>(range: impl Iterator<Item = usize>, cmp: F) -> i32
where
    F: Fn(usize) -> bool,
{
    let mut result = 0;
    for i in range {
        result += 1;

        if cmp(i) {
            break;
        }
    }
    result
}
