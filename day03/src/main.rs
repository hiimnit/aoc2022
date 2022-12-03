use std::env;
use std::fs;

fn score(char: &u8) -> u8 {
    return match char {
        b'a'..=b'z' => char - b'a' + 1,
        b'A'..=b'Z' => char - b'A' + 27,
        _ => unimplemented!(),
    };
}

fn score_common_char_2(left: &str, right: &str) -> u8 {
    for char in left.as_bytes() {
        // TODO
        if right.as_bytes().contains(char) {
            return score(char);
        }
    }
    unimplemented!();
}

fn score_common_char_3(first: &str, second: &str, third: &str) -> u8 {
    for char in first.as_bytes() {
        if second.as_bytes().contains(char) && third.as_bytes().contains(char) {
            return score(char);
        }
    }
    unimplemented!();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = args
        .get(1)
        .expect("Should pass a file path as the second parameter.");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut result: u32 = 0;

    for line in lines {
        let (left, right) = line.split_at(line.len() / 2);

        result += score_common_char_2(left, right) as u32;
    }

    println!("part 1: {}", result);

    let mut lines = contents.lines().peekable();
    let mut result: u32 = 0;

    while lines.peek().is_some() {
        result += score_common_char_3(
            lines.next().unwrap(),
            lines.next().unwrap(),
            lines.next().unwrap(),
        ) as u32;
    }

    println!("part 2: {}", result);
}
