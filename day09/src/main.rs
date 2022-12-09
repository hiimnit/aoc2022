use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_path = args
        .get(1)
        .expect("Pass in input file path as the first argument!");

    let content = fs::read_to_string(file_path).expect("Could not read the input file!");

    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut set: HashSet<(i32, i32)> = HashSet::from([tail]);

    for line in content.lines() {
        match line.split_whitespace().collect::<Vec<&str>>()[..] {
            [direction, steps_str] => {
                let movement = dir_to_movement(direction);
                let steps = steps_str.parse::<usize>().unwrap();

                for _ in 0..steps {
                    head = (head.0 + movement.0, head.1 + movement.1);

                    let tail_movement = calc_tail_movement(&head, &tail);
                    tail = (tail.0 + tail_movement.0, tail.1 + tail_movement.1);

                    set.insert(tail);
                }
            }
            _ => unimplemented!("Could not match line {line}."),
        }
    }

    println!("part 1: {}", set.len());

    let mut head = (0, 0);
    let mut tails = vec![(0, 0); 9];
    let mut set: HashSet<(i32, i32)> = HashSet::from([head]);

    for line in content.lines() {
        match line.split_whitespace().collect::<Vec<&str>>()[..] {
            [direction, steps_str] => {
                let movement = dir_to_movement(direction);
                let steps = steps_str.parse::<usize>().unwrap();

                for _ in 0..steps {
                    head = (head.0 + movement.0, head.1 + movement.1);

                    let mut parent = &head;
                    for i in 0..tails.len() {
                        let tail_movement = calc_tail_movement(parent, &tails[i]);
                        tails[i] = (tails[i].0 + tail_movement.0, tails[i].1 + tail_movement.1);
                        parent = &tails[i];
                    }

                    set.insert(*tails.last().unwrap());
                }
            }
            _ => unimplemented!("Could not match line {line}."),
        }
    }

    println!("part 2: {}", set.len());

    println!("{}", -2 % 2);
}

fn calc_tail_movement(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    let vector = (head.0 - tail.0, head.1 - tail.1);

    // if both parts are 1 or smaller, do nothing
    if vector.0.abs() < 2 && vector.1.abs() < 2 {
        return (0, 0);
    }

    // else move tail - difference of 2 is clamped to 1
    (vector.0.clamp(-1, 1), vector.1.clamp(-1, 1))
}

fn dir_to_movement(direction: &str) -> (i32, i32) {
    match direction {
        "U" => (0, 1),
        "D" => (0, -1),
        "L" => (-1, 0),
        "R" => (1, 0),
        _ => unimplemented!("Unknown direction {direction}!"),
    }
}
