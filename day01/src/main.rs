use std::env;
use std::fs;

// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
// struct Elf {
//     calories: i32,
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();

    let mut elves: Vec<i32> = Vec::new();
    let mut sum = 0;

    for line in lines {
        if line == "" {
            elves.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    elves.sort();

    println!("most calories: {}", elves.last().unwrap());

    let three_largest_sum: i32 = elves.iter().rev().take(3).sum();

    println!("sum of three largest: {}", three_largest_sum);
}
