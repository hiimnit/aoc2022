use std::env;
use std::fs;

fn snafu_to_base_10(snafu: &str) -> i64 {
    let mut result = 0;
    let mut multiplier = 1;
    for ch in snafu.chars().rev() {
        let number = match ch {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => unimplemented!("Unknown digit {ch}!"),
        };

        result += number * multiplier;
        multiplier *= 5;
    }

    result
}

fn base_10_to_snafu(base10: i64) -> String {
    if base10 == -2 {
        return "=".to_owned();
    }

    let mut base10 = base10;
    let mut result: Vec<char> = vec![];

    while base10 != 0 {
        let modulo = base10 % 5;

        let (digit, carry) = match modulo {
            4 => ('-', 1),
            3 => ('=', 2),
            2 => ('2', 0),
            1 => ('1', 0),
            0 => ('0', 0),
            _ => unimplemented!("Unknown state {modulo}!"),
        };

        result.push(digit);
        base10 += carry;
        base10 /= 5;
    }

    result.iter().rev().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = args
        .get(1)
        .expect("Input file path expected as the first argument!");

    let content = fs::read_to_string(input_file_path).expect("Could not read the input file!");

    let mut numbers = vec![];
    for line in content.lines() {
        numbers.push(snafu_to_base_10(line));
    }

    println!("part 1: {}", base_10_to_snafu(numbers.iter().sum()));
}
