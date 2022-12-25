use std::collections::HashSet;
use std::env;
use std::fs;

fn get_neighbors((x, y, z): (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    vec![
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = args
        .get(1)
        .expect("Input file path expected as the first argument!");

    let content = fs::read_to_string(input_file_path).expect("Could not read the input file!");

    let mut result = 0;
    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();
    for line in content.lines() {
        let (x, y, z) = match line.split(',').collect::<Vec<&str>>()[..] {
            [x, y, z] => (
                x.parse::<i32>().unwrap(),
                y.parse::<i32>().unwrap(),
                z.parse::<i32>().unwrap(),
            ),
            _ => unimplemented!("Unexpected input {line}"),
        };

        result += 6;
        result -= get_neighbors((x, y, z))
            .iter()
            .filter(|e| cubes.contains(e))
            .count()
            * 2;
        cubes.insert((x, y, z));
    }

    println!("part 1: {result}");

    let mut space = Space::new(&cubes);

    space.flood_fill();

    let air_touched_rocks = space.count_air_touched_rocks();

    println!(
        "part 2: {result} - {air_touched_rocks} = {}",
        result - air_touched_rocks
    );
}

#[derive(Debug, Clone, PartialEq)]
enum Point {
    Air,
    Rock,
    Lava,
}

#[derive(Debug)]
struct Space {
    points: Vec<Vec<Vec<Point>>>,
    offset: (i32, i32, i32),
}

impl Space {
    pub fn new(rocks: &HashSet<(i32, i32, i32)>) -> Self {
        let x_min = rocks.iter().map(|e| e.0).min().unwrap() - 1;
        let x_max = rocks.iter().map(|e| e.0).max().unwrap() + 1;

        let y_min = rocks.iter().map(|e| e.1).min().unwrap() - 1;
        let y_max = rocks.iter().map(|e| e.1).max().unwrap() + 1;

        let z_min = rocks.iter().map(|e| e.2).min().unwrap() - 1;
        let z_max = rocks.iter().map(|e| e.2).max().unwrap() + 1;

        let size = (
            (x_max - x_min + 1) as usize,
            (y_max - y_min + 1) as usize,
            (z_max - z_min + 1) as usize,
        );

        let mut space = Self {
            points: vec![vec![vec![Point::Air; size.2]; size.1]; size.0],
            offset: (x_min, y_min, z_min),
        };

        for rock in rocks {
            space.add_rock(rock);
        }

        return space;
    }

    pub fn add_rock(self: &mut Self, (x, y, z): &(i32, i32, i32)) {
        self.points[(x - self.offset.0) as usize][(y - self.offset.1) as usize]
            [(z - self.offset.2) as usize] = Point::Rock;
    }

    pub fn flood_fill(self: &mut Self) {
        // starting at (0, 0, 0), fill the remaining reachable points with lava
        // (the space is setup so that it is one space larger on all sides, so these points will always be `Air`)

        let mut queue = vec![(0, 0, 0)];
        let mut queued: HashSet<(i32, i32, i32)> = HashSet::from([(0, 0, 0)]);

        while let Some((x, y, z)) = queue.pop() {
            if !self.is_valid_point((x, y, z)) {
                continue;
            }
            if self.points[x as usize][y as usize][z as usize] != Point::Air {
                continue;
            }

            self.points[x as usize][y as usize][z as usize] = Point::Lava;

            for neighbor in get_neighbors((x, y, z)) {
                if !queued.contains(&neighbor) {
                    queue.push(neighbor);
                    queued.insert(neighbor);
                }
            }
        }
    }

    pub fn is_valid_point(self: &Self, (x, y, z): (i32, i32, i32)) -> bool {
        if x < 0 || x as usize >= self.points.len() {
            return false;
        }

        if y < 0 || y as usize >= self.points[x as usize].len() {
            return false;
        }

        if z < 0 || z as usize >= self.points[x as usize][y as usize].len() {
            return false;
        }

        true
    }

    pub fn count_air_touched_rocks(self: &Self) -> usize {
        let mut result = 0;
        for (x, plane) in self.points.iter().enumerate() {
            for (y, row) in plane.iter().enumerate() {
                for (z, point) in row.iter().enumerate() {
                    if *point != Point::Air {
                        continue;
                    }

                    result += get_neighbors((x as i32, y as i32, z as i32))
                        .iter()
                        .filter(|e| {
                            self.points[e.0 as usize][e.1 as usize][e.2 as usize] == Point::Rock
                        })
                        .count();
                }
            }
        }

        result
    }
}
