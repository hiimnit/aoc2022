use std::env;
use std::fs;

fn are_unique(values: &[char], count: usize) -> bool {
    if count < 2 {
        return true;
    }

    if values.len() != count {
        panic!("unexpected length {}, expected {}", values.len(), count);
    }

    if values[1..].contains(&values[0]) {
        return false;
    }

    return are_unique(&values[1..], count - 1);
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_path = args
        .get(1)
        .expect("Pass in input file path as the first argument!");

    let sequence_length = args
        .get(2)
        .expect("Pass in the desired unique sequence length as the second parameter!")
        .parse::<usize>()
        .expect("Sequence length must be a valid usize!");

    let content = fs::read_to_string(file_path).expect("Could not read the input file!");

    let mut values: Vec<char> = content
        .chars()
        .into_iter()
        .take(sequence_length - 1)
        .collect();

    for (i, char) in content
        .chars()
        .into_iter()
        .skip(sequence_length - 1)
        .enumerate()
    {
        values.push(char);

        if are_unique(&values, sequence_length) {
            println!("result: {}", i + sequence_length);
            break;
        }

        values = values.into_iter().skip(1).collect();
    }
}
