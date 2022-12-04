use std::env;
use std::fs;
use std::ops::RangeInclusive;

fn parse_range(range: &str) -> RangeInclusive<i32> {
    if let &[from, to] = range
        .split("-")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&e| e.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .as_slice()
    {
        return from..=to;
    }

    unimplemented!("range parsing error");
}

fn compare_ranges(first: &str, second: &str) -> (bool, bool) {
    let first_range = parse_range(first);
    let second_range = parse_range(second);

    (
        is_range_fully_contained(&first_range, &second_range)
            || is_range_fully_contained(&second_range, &first_range),
        is_range_partially_overlapping(&first_range, &second_range)
            || is_range_partially_overlapping(&second_range, &first_range),
    )
}

fn is_range_fully_contained(parent: &RangeInclusive<i32>, child: &RangeInclusive<i32>) -> bool {
    parent.contains(child.start()) && parent.contains(child.end())
}

fn is_range_partially_overlapping(
    parent: &RangeInclusive<i32>,
    child: &RangeInclusive<i32>,
) -> bool {
    parent.contains(child.start()) || parent.contains(child.end())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = args
        .get(1)
        .expect("Should pass a file path as the second parameter.");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut full_overlap: u32 = 0;
    let mut partial_overlap: u32 = 0;

    for line in lines {
        if let &[first, second] = line.split(",").collect::<Vec<&str>>().as_slice() {
            let (full, partial) = compare_ranges(first, second);
            if full {
                full_overlap += 1;
            }
            if partial {
                partial_overlap += 1;
            }
        } else {
            unimplemented!("section parsing error");
        }
    }

    println!("full_overlap: {}", full_overlap);
    println!("partial_overlap: {}", partial_overlap);
}
