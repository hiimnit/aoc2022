use std::env;
use std::fs;
use std::ops::RangeInclusive;

struct Pair {
    sensor: (i32, i32),
    beacon: (i32, i32),
    radius: i32,
}

impl Pair {
    pub fn new(sensor: (i32, i32), beacon: (i32, i32)) -> Self {
        Self {
            sensor,
            beacon,
            radius: distance(sensor, beacon),
        }
    }

    pub fn get_intersection_range(self: &Self, y: i32) -> Option<RangeInclusive<i32>> {
        let x = self.radius - (y - self.sensor.1).abs();
        if x < 0 {
            return None;
        }

        Some((self.sensor.0 - x)..=(self.sensor.0 + x))
    }
}

struct Map {
    pairs: Vec<Pair>,
}

impl Map {
    pub fn new() -> Self {
        Self { pairs: vec![] }
    }

    pub fn add_pair(self: &mut Self, sensor: (i32, i32), beacon: (i32, i32)) -> () {
        self.pairs.push(Pair::new(sensor, beacon))
    }

    fn merge_at_line(self: &Self, y: i32) -> Option<Vec<RangeInclusive<i32>>> {
        let mut intersections = vec![];

        for pair in &self.pairs {
            if let Some(intersection) = pair.get_intersection_range(y) {
                intersections.push(intersection);
            }
        }

        if intersections.is_empty() {
            return None;
        }

        let mut merged_intersections = vec![];
        let mut current_intersection = intersections.pop().unwrap();
        'outer: loop {
            for (i, intersection) in merged_intersections.iter().enumerate() {
                match merge_ranges(&current_intersection, intersection) {
                    Some(merge) => {
                        current_intersection = merge;
                        merged_intersections.remove(i);
                        continue 'outer;
                    }
                    None => {}
                }
            }

            merged_intersections.push(current_intersection.clone());
            match intersections.pop() {
                Some(intersection) => current_intersection = intersection,
                None => break,
            }
        }

        Some(merged_intersections)
    }

    pub fn scan_line(self: &Self, y: i32) -> Option<i32> {
        if let Some(merged_intersections) = self.merge_at_line(y) {
            return Some(
                merged_intersections
                    .iter()
                    .map(|e| e.end() - e.start())
                    .sum(),
            );
        }

        None
    }

    // not pretty/optimal, but its getting too late
    pub fn check_range_at_line(self: &Self, y: i32, range: &RangeInclusive<i32>) -> Option<i32> {
        let merged_intersections = self.merge_at_line(y)?;

        let mut remaining_ranges = vec![range.clone()];
        for intersection in merged_intersections {
            'inner: loop {
                for (i, range) in remaining_ranges.iter().enumerate() {
                    match subract_ranges(range, &intersection) {
                        RangeDifference::Full => {}
                        RangeDifference::OneRange(difference) => {
                            remaining_ranges[i] = difference;
                            continue 'inner;
                        }
                        RangeDifference::TwoRanges(difference1, difference2) => {
                            remaining_ranges[i] = difference1;
                            remaining_ranges.push(difference2);
                            continue 'inner;
                        }
                        RangeDifference::Nothing => {
                            remaining_ranges.remove(i);
                            continue 'inner;
                        }
                    }
                }
                if remaining_ranges.is_empty() {
                    return None;
                }

                break 'inner;
            }
        }

        match remaining_ranges.len() {
            1 => {
                let result = remaining_ranges.first().unwrap();
                assert!(result.start() == result.end());
                return Some(*result.start());
            }
            _ => unimplemented!(),
        }
    }
}

fn distance(sensor: (i32, i32), beacon: (i32, i32)) -> i32 {
    (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs()
}

fn merge_ranges(
    input: &RangeInclusive<i32>,
    other: &RangeInclusive<i32>,
) -> Option<RangeInclusive<i32>> {
    match (other.contains(&input.start()), other.contains(&input.end())) {
        (true, true) => return Some(other.clone()),
        (true, false) => return Some(*other.start()..=*input.end()),
        (false, true) => return Some(*input.start()..=*other.end()),
        (false, false) => {}
    }

    if input.contains(&other.start()) && input.contains(&other.end()) {
        return Some(input.clone());
    }

    if input.end() + 1 == *other.start() {
        return Some(*input.start()..=*other.end());
    }

    if other.end() + 1 == *input.start() {
        return Some(*other.start()..=*input.end());
    }

    None
}

#[derive(Debug)]
enum RangeDifference {
    Full,
    OneRange(RangeInclusive<i32>),
    TwoRanges(RangeInclusive<i32>, RangeInclusive<i32>),
    Nothing,
}

fn subract_ranges(
    minuend: &RangeInclusive<i32>,
    subtrahend: &RangeInclusive<i32>,
) -> RangeDifference {
    match (
        subtrahend.contains(&minuend.start()),
        subtrahend.contains(&minuend.end()),
    ) {
        (true, true) => return RangeDifference::Nothing,
        (true, false) => return RangeDifference::OneRange((subtrahend.end() + 1)..=*minuend.end()),
        (false, true) => {
            return RangeDifference::OneRange(*minuend.start()..=(subtrahend.start() - 1))
        }
        (false, false) => {}
    }

    if minuend.contains(&subtrahend.start()) && minuend.contains(&subtrahend.end()) {
        return RangeDifference::TwoRanges(
            *minuend.start()..=(subtrahend.start() - 1),
            (subtrahend.end() + 1)..=*minuend.end(),
        );
    }

    RangeDifference::Full
}

fn parse_coordinate(input: &str) -> i32 {
    let input = input.replace(",", "").replace(":", "");
    match input.split("=").collect::<Vec<&str>>()[..] {
        ["y" | "x", n] => n.parse::<i32>().expect("Expected i32, got {n}"),
        _ => unimplemented!("Could not parse input {input}"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args
        .get(1)
        .expect("Pass in the input file as the first argument");

    let scan_y = args
        .get(2)
        .expect("Pass in the the scanned row as the second argument")
        .parse::<i32>()
        .expect("Expected a valid i32 as the second argument");

    let content = fs::read_to_string(file_path).expect("Could not read the input file");

    let mut map = Map::new();

    for line in content.lines() {
        match line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["Sensor", "at", s_x, s_y, "closest", "beacon", "is", "at", b_x, b_y] => {
                let sensor = (parse_coordinate(s_x), parse_coordinate(s_y));
                let beacon = (parse_coordinate(b_x), parse_coordinate(b_y));

                map.add_pair(sensor, beacon);
            }
            _ => unimplemented!("Unexpected input: {line}"),
        }
    }

    println!("part 1: {:?}", map.scan_line(scan_y));

    let checked_range = 0..=4000000;
    for y in 0..=4000000 {
        if let Some(x) = map.check_range_at_line(y, &checked_range) {
            println!(
                "part 2: x={x} y={y} result={}",
                x as i64 * 4000000 + y as i64
            );
            break;
        }
    }
}
