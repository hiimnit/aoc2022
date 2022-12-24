use std::env;
use std::fs;

// part 1: cargo run input
// part 2: cargo run input 811589153 10

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = args
        .get(1)
        .expect("Input file path expected as the first argument!");

    let content = fs::read_to_string(input_file_path).expect("Could not read the input file!");

    let decryption_key = args
        .get(2)
        .or(Some(&"1".to_string()))
        .unwrap()
        .parse::<i64>()
        .expect("Decryption key must be a valid i64!");

    let mut encrypted = content
        .lines()
        .map(|e| e.parse::<i64>().unwrap() * decryption_key)
        .collect::<Vec<i64>>();
    let mut positions = encrypted
        .iter()
        .enumerate()
        .map(|e| e.0)
        .collect::<Vec<usize>>();

    let mixing_iterations = args
        .get(3)
        .or(Some(&"1".to_string()))
        .unwrap()
        .parse::<usize>()
        .expect("Decryption key must be a valid usize!");

    let length = positions.len();
    for _ in 0..mixing_iterations {
        for i in 0..length {
            let position = positions.iter().position(|&e| e == i).unwrap();
            let value = encrypted[position];
            let mut next_position = position as i64 + value;

            if next_position < 0 || next_position >= length as i64 {
                next_position %= length as i64 - 1;
                if next_position < 0 {
                    next_position += length as i64 - 1;
                }
            }

            move_element(&mut encrypted, position, next_position as usize);
            move_element(&mut positions, position, next_position as usize);
        }
    }

    let null_position = encrypted.iter().position(|&e| e == 0).unwrap();
    println!(
        "result: {} + {} + {} = {}",
        encrypted[(null_position + 1000) % length],
        encrypted[(null_position + 2000) % length],
        encrypted[(null_position + 3000) % length],
        encrypted[(null_position + 1000) % length]
            + encrypted[(null_position + 2000) % length]
            + encrypted[(null_position + 3000) % length],
    );
}

fn move_element<T>(array: &mut Vec<T>, from: usize, to: usize) {
    if from == to {
        return;
    }
    let element = array.remove(from);
    array.insert(to, element);
}
