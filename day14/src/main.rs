use std::env;
use std::fmt;
use std::fs;

#[derive(Debug, Clone)]
enum Point {
    Air,
    Rock,
    Sand,
}

impl Point {
    fn to_string(&self) -> &str {
        match self {
            Point::Air => " ",
            Point::Rock => "#",
            Point::Sand => "o",
        }
    }
}

fn pretty_print_scan(scan: &Vec<Vec<Point>>) -> () {
    for line in scan {
        println!(
            "{}",
            line.iter()
                .map(|e| e.to_string())
                .collect::<Vec<&str>>()
                .join("")
        );
    }
    println!("{}", vec!["="; scan[0].len()].join(""));
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_path = args
        .get(1)
        .expect("Pass in input file path as the first argument!");

    let content = fs::read_to_string(file_path).expect("Could not read the input file!");

    println!("Hello, world!");

    let mut rocks: Vec<Vec<(usize, usize)>> = vec![];
    let mut x_range: (usize, usize) = (usize::MAX, usize::MIN);
    let mut max_depth: usize = usize::MIN;

    for line in content.lines() {
        let mut rock: Vec<(usize, usize)> = vec![];

        for point in line
            .split(" -> ")
            .map(|e| e.split(",").collect::<Vec<&str>>())
        {
            let (x, y) = match point[..] {
                [x, y] => (
                    x.parse::<usize>().expect("Expected valid usize, got {x}"),
                    y.parse::<usize>().expect("Expected valid usize, got {y}"),
                ),
                _ => unimplemented!("Expected point, got {point:?}"),
            };

            if x < x_range.0 {
                x_range = (x, x_range.1);
            }
            if x > x_range.1 {
                x_range = (x_range.0, x);
            }
            if y > max_depth {
                max_depth = y;
            }

            rock.push((x, y));
        }
        rocks.push(rock);
    }

    println!("{rocks:?}, {x_range:?}, {max_depth}");

    let mut scan = vec![vec![Point::Air; x_range.1 - x_range.0 + 1]; max_depth + 1];

    // pretty_print_scan(&scan);
    for rock in rocks {
        for (p1, p2) in rock.iter().zip(rock.iter().skip(1)) {
            match (p1.0 == p2.0, p1.1 == p2.1) {
                (true, false) => {
                    let mut range = vec![p1.1, p2.1];
                    range.sort();

                    for i in range[0]..=range[1] {
                        scan[i][p1.0 - x_range.0] = Point::Rock;
                    }
                }
                (false, true) => {
                    let mut range = vec![p1.0, p2.0];
                    range.sort();

                    for i in range[0]..=range[1] {
                        scan[p1.1][i - x_range.0] = Point::Rock;
                    }
                }
                _ => unimplemented!(),
            }
        }
    }

    // pretty_print_scan(&scan); // TODO loop - add sand

    let origin = 500 - x_range.0;
    let mut counter = 0;
    'outer: loop {
        let mut position = (origin, 0);

        loop {
            match get_point(&scan, (position.0, position.1 + 1)) {
                Some(Point::Air) => {
                    position = (position.0, position.1 + 1);
                    continue;
                }
                Some(Point::Sand) | Some(Point::Rock) => {
                    if let Some(Point::Air) = get_point(&scan, (position.0 - 1, position.1 + 1)) {
                        position = (position.0 - 1, position.1 + 1);
                        continue;
                    }

                    if let Some(Point::Air) = get_point(&scan, (position.0 + 1, position.1 + 1)) {
                        position = (position.0 + 1, position.1 + 1);
                        continue;
                    }

                    scan[position.1][position.0] = Point::Sand;
                    break;
                }
                None => {
                    break 'outer;
                }
            }
        }

        // if counter % 100 == 0 {
        //     pretty_print_scan(&scan);
        // }

        counter += 1;
    }

    pretty_print_scan(&scan);

    println!("part 1: {counter}");
}

fn get_point(scan: &Vec<Vec<Point>>, (x, y): (usize, usize)) -> Option<&Point> {
    scan.get(y)?.get(x)
}
