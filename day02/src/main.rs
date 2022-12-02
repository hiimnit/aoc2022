use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = args
        .get(1)
        .expect("Should pass a file path as the second parameter.");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = contents.lines();

    let mut result = 0;
    for line in lines {
        let words = line.split_whitespace();

        result += match words.collect::<Vec<&str>>().as_slice() {
            ["A", "X"] => 3 + 1,
            ["B", "X"] => 0 + 1,
            ["C", "X"] => 6 + 1,
            ["A", "Y"] => 6 + 2,
            ["B", "Y"] => 3 + 2,
            ["C", "Y"] => 0 + 2,
            ["A", "Z"] => 0 + 3,
            ["B", "Z"] => 6 + 3,
            ["C", "Z"] => 3 + 3,
            _ => unimplemented!(),
        }
    }

    println!("result: {}", result);

    let lines = contents.lines();

    let mut result = 0;
    for line in lines {
        let words = line.split_whitespace();

        result += match words.collect::<Vec<&str>>().as_slice() {
            ["A", "X"] => 0 + 3,
            ["B", "X"] => 0 + 1,
            ["C", "X"] => 0 + 2,
            ["A", "Y"] => 3 + 1,
            ["B", "Y"] => 3 + 2,
            ["C", "Y"] => 3 + 3,
            ["A", "Z"] => 6 + 2,
            ["B", "Z"] => 6 + 3,
            ["C", "Z"] => 6 + 1,
            _ => unimplemented!(),
        }
    }

    println!("result: {}", result);
}
