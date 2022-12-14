use std::env;
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

    max_depth += 2;

    println!("{rocks:?}, {x_range:?}, {max_depth}");

    let mut scan = vec![vec![Point::Air; x_range.1 - x_range.0 + 1]; max_depth + 1];
    let mut offset = x_range.0;

    pretty_print_scan(&scan);

    for rock in rocks {
        for (p1, p2) in rock.iter().zip(rock.iter().skip(1)) {
            match (p1.0 == p2.0, p1.1 == p2.1) {
                (true, false) => {
                    let mut range = vec![p1.1, p2.1];
                    range.sort();

                    for i in range[0]..=range[1] {
                        scan[i][p1.0 - offset] = Point::Rock;
                    }
                }
                (false, true) => {
                    let mut range = vec![p1.0, p2.0];
                    range.sort();

                    for i in range[0]..=range[1] {
                        scan[p1.1][i - offset] = Point::Rock;
                    }
                }
                _ => unimplemented!(),
            }
        }
    }

    let last_row_index = scan.len() - 1;
    for i in 0..scan[0].len() {
        scan[last_row_index][i] = Point::Rock;
    }

    pretty_print_scan(&scan);

    let mut origin = 500 - offset;
    let mut counter = 0;
    loop {
        let mut position = (origin as i32, 0);

        if let Some(Point::Rock) | Some(Point::Sand) = get_point(&scan, position) {
            println!("The cave is filled");
            break;
        }

        loop {
            match get_point(&scan, (position.0, position.1 + 1)) {
                // next position is air -> continue
                Some(Point::Air) => {
                    position = (position.0, position.1 + 1);
                    continue;
                }
                // next position is sand or rock
                Some(Point::Sand) | Some(Point::Rock) => {
                    // check the position to the left, if air -> continue
                    match get_point(&scan, (position.0 - 1, position.1 + 1)) {
                        Some(Point::Air) => {
                            position = (position.0 - 1, position.1 + 1);
                            continue;
                        }
                        None => {
                            // out of bounds -> expand
                            expand(&mut scan, position.0 == 0);
                            if position.0 == 0 {
                                position = (position.0 + 1, position.1);
                                offset += 1;
                                origin += 1;
                            }
                            continue;
                        }
                        _ => (),
                    }

                    // check the position to the right, if air -> continue
                    match get_point(&scan, (position.0 + 1, position.1 + 1)) {
                        Some(Point::Air) => {
                            position = (position.0 + 1, position.1 + 1);
                            continue;
                        }
                        None => {
                            // out of bounds -> expand
                            expand(&mut scan, position.0 == 0);
                            if position.0 == 0 {
                                position = (position.0 + 1, position.1);
                                offset += 1;
                                origin += 1;
                            }

                            continue;
                        }
                        _ => (),
                    }

                    // else place sand
                    scan[position.1][position.0 as usize] = Point::Sand;
                    break;
                }
                None => unimplemented!("Ran out of bounds."),
            }
        }

        counter += 1;
    }

    pretty_print_scan(&scan);

    println!("result: {counter}");
}

fn get_point(scan: &Vec<Vec<Point>>, (x, y): (i32, usize)) -> Option<&Point> {
    if x < 0 || (x as usize) >= scan[0].len() {
        return None;
    }
    scan.get(y)?.get(x as usize)
}

fn expand(scan: &mut Vec<Vec<Point>>, start: bool) -> () {
    let length = scan.len();

    if start {
        for (i, line) in scan.iter_mut().enumerate() {
            line.insert(
                0,
                if i == length - 1 {
                    Point::Rock
                } else {
                    Point::Air
                },
            );
        }
        return;
    }

    for (i, line) in scan.iter_mut().enumerate() {
        line.push(if i == length - 1 {
            Point::Rock
        } else {
            Point::Air
        });
    }
}
