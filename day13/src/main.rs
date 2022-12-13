use std::env;
use std::fs;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
enum Value {
    Number(usize),
    Vector(Vec<Value>),
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match compare_signals(self, other) {
            Some(true) => std::cmp::Ordering::Less,
            None => std::cmp::Ordering::Equal,
            Some(false) => std::cmp::Ordering::Greater,
        }
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

fn char_to_usize(c: &char) -> usize {
    match c {
        '0'..='9' => (*c as u8 - b'0') as usize,
        _ => unimplemented!("Expected number, got {c}."),
    }
}

fn parse_signal(chars: &mut Peekable<Chars>) -> Option<Value> {
    match chars.next() {
        Some('[') => {
            let mut values: Vec<Value> = vec![];
            loop {
                match parse_signal(chars) {
                    Some(value) => values.push(value),
                    None => break,
                }

                match chars.next() {
                    Some(',') => (),
                    Some(']') => break,
                    c => unimplemented!("Expected '[' or ',', got {c:?}."),
                }
            }
            Some(Value::Vector(values))
        }
        Some(']') => None,
        Some(c) => {
            let mut number = char_to_usize(&c);
            loop {
                if let Some(',') | Some(']') = chars.peek() {
                    break;
                }

                let c = chars.next().unwrap();
                number *= 10;
                number += char_to_usize(&c);
            }
            Some(Value::Number(number))
        }
        None => unimplemented!(),
    }
}

fn parse_and_compare_signals(left_str: &str, right_str: &str) -> Option<bool> {
    let (left, right) = match (
        parse_signal(&mut left_str.chars().peekable()),
        parse_signal(&mut right_str.chars().peekable()),
    ) {
        (Some(left_value), Some(right_value)) => (left_value, right_value),
        _ => unimplemented!("Could not parse signals!"),
    };

    compare_signals(&left, &right)
}

fn compare_signals(left: &Value, right: &Value) -> Option<bool> {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => {
            if l == r {
                return None;
            }
            Some(l < r)
        }
        (Value::Vector(l), Value::Vector(r)) => {
            for (left_zip, right_zip) in l.iter().zip(r.iter()) {
                match compare_signals(left_zip, right_zip) {
                    Some(result) => return Some(result),
                    None => {}
                };
            }

            match compare_signals(&Value::Number(l.len()), &Value::Number(r.len())) {
                Some(result) => Some(result),
                None => None,
            }
        }
        (Value::Number(number), Value::Vector(_)) => {
            compare_signals(&Value::Vector(vec![Value::Number(*number)]), right)
        }
        (Value::Vector(_), Value::Number(number)) => {
            compare_signals(left, &Value::Vector(vec![Value::Number(*number)]))
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_path = args
        .get(1)
        .expect("Pass in input file path as the first argument!");

    let content = fs::read_to_string(file_path).expect("Could not read the input file!");

    let mut result = 0;
    for (i, block) in content.split("\n\n").enumerate() {
        match block.split("\n").collect::<Vec<&str>>()[..] {
            [left, right] => {
                if let Some(true) = parse_and_compare_signals(left, right) {
                    result += i + 1;
                }
            }
            _ => unimplemented!(),
        }
    }

    println!("part 1: {result}");

    let mut signals = vec![];

    for line in content.lines().filter(|&e| e != "") {
        signals.push(
            parse_signal(&mut line.chars().peekable())
                .expect("Could not parse a signal on line {line}."),
        );
    }

    signals.push(Value::Vector(vec![Value::Vector(vec![Value::Number(2)])]));
    signals.push(Value::Vector(vec![Value::Vector(vec![Value::Number(6)])]));

    signals.sort();

    let d1 = signals
        .iter()
        .position(|e| *e == Value::Vector(vec![Value::Vector(vec![Value::Number(2)])]))
        .expect("First part of the divider was not found!");
    let d2 = signals
        .iter()
        .position(|e| *e == Value::Vector(vec![Value::Vector(vec![Value::Number(6)])]))
        .expect("Second part of the divider was not found!");

    println!("part 2: {} * {} = {}", d1 + 1, d2 + 1, (d1 + 1) * (d2 + 1));
}
