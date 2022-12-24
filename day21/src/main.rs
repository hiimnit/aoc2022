use std::cmp::max;
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    pub fn from(op: &str) -> Option<Self> {
        match op {
            "+" => Some(Self::Add),
            "-" => Some(Self::Subtract),
            "*" => Some(Self::Multiply),
            "/" => Some(Self::Divide),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Job<'a> {
    Yell(i64),
    Calculate(&'a str, Operation, &'a str),
    Human,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = args
        .get(1)
        .expect("Input file path expected as the first argument!");

    let content = fs::read_to_string(input_file_path).expect("Could not read the input file!");

    let mut monkeys: HashMap<&str, Job> = HashMap::new();

    for line in content.lines() {
        let (monkey, calculation) = match line.split(": ").collect::<Vec<&str>>()[..] {
            [monkey, calculation] => (monkey, calculation),
            _ => unimplemented!("Unexpected input {line}!"),
        };

        let job = match calculation.split_whitespace().collect::<Vec<&str>>()[..] {
            [number] => Job::Yell(number.parse().unwrap()),
            [m1, operation, m2] => Job::Calculate(
                m1,
                Operation::from(operation).expect("Could not parse operation {operation}!"),
                m2,
            ),
            _ => unimplemented!("Unexpected calculation {calculation}!"),
        };

        monkeys.insert(monkey, job);
    }

    // comment this line to get the part 1 solution
    *monkeys.get_mut("humn").unwrap() = Job::Human;

    let depths = calc_depths(&monkeys);

    let mut sorted_depths = depths.keys().collect::<Vec<&i64>>();
    sorted_depths.sort();
    for depth in sorted_depths {
        for monkey in &depths[depth] {
            let result: Option<i64> = match &monkeys[monkey] {
                Job::Yell(n) => Some(n.to_owned()),
                Job::Human => None,
                Job::Calculate(m1, operation, m2) => {
                    let operand1 = match &monkeys[m1] {
                        Job::Yell(n) => Some(n),
                        _ => None,
                    };
                    let operand2 = match &monkeys[m2] {
                        Job::Yell(n) => Some(n),
                        _ => None,
                    };

                    match (operand1, operand2) {
                        (Some(operand1), Some(operand2)) => Some(match operation {
                            Operation::Add => operand1 + operand2,
                            Operation::Subtract => operand1 - operand2,
                            Operation::Multiply => operand1 * operand2,
                            Operation::Divide => operand1 / operand2,
                        }),
                        _ => None,
                    }
                }
            };

            if let Some(number) = result {
                *monkeys.get_mut(monkey).unwrap() = Job::Yell(number);
            }
        }
    }

    let root = monkeys.get("root").unwrap();

    match root {
        Job::Yell(number) => println!("root yells {number}"),
        Job::Calculate(m1, _, m2) => {
            let result = match (&monkeys[m1], &monkeys[m2]) {
                (Job::Yell(expected_number), Job::Calculate(_, _, _)) => {
                    find_human(&monkeys, m2, *expected_number)
                }
                (Job::Calculate(_, _, _), Job::Yell(expected_number)) => {
                    find_human(&monkeys, m1, *expected_number)
                }
                _ => unimplemented!("Unexpected state!"),
            };
            println!("human should yell {result}");
        }
        Job::Human => unimplemented!("Root should not be the human!"),
    };
}

fn calc_depths<'a>(monkeys: &HashMap<&'a str, Job>) -> HashMap<i64, Vec<&'a str>> {
    let mut depths: HashMap<i64, Vec<&str>> = HashMap::new();

    for (&monkey, _) in monkeys {
        let depth = calc_depth(monkeys, monkey);
        if !depths.contains_key(&depth) {
            depths.insert(depth, vec![]);
        }
        depths.get_mut(&depth).unwrap().push(monkey);
    }

    depths
}

fn calc_depth(monkeys: &HashMap<&str, Job>, monkey: &str) -> i64 {
    match monkeys[monkey] {
        Job::Yell(_) | Job::Human => 0,
        Job::Calculate(m1, _, m2) => max(calc_depth(monkeys, m1), calc_depth(monkeys, m2)) + 1,
    }
}

fn find_human<'a>(
    monkeys: &'a HashMap<&'a str, Job>,
    monkey: &'a str,
    expected_number: i64,
) -> i64 {
    let (first_monkey, operation, second_monkey) = match &monkeys[monkey] {
        Job::Yell(_) => unimplemented!("Unexpected state!"),
        Job::Calculate(m1, operation, m2) => (m1, operation, m2),
        Job::Human => return expected_number,
    };

    match (&monkeys[first_monkey], &monkeys[second_monkey]) {
        (Job::Yell(n), Job::Calculate(_, _, _)) | (Job::Yell(n), Job::Human) => match operation {
            Operation::Add => return find_human(monkeys, second_monkey, expected_number - n),
            Operation::Subtract => return find_human(monkeys, second_monkey, n - expected_number),
            Operation::Multiply => return find_human(monkeys, second_monkey, expected_number / n),
            Operation::Divide => return find_human(monkeys, second_monkey, n / expected_number),
        },
        (Job::Calculate(_, _, _), Job::Yell(n)) | (Job::Human, Job::Yell(n)) => match operation {
            Operation::Add => return find_human(monkeys, first_monkey, expected_number - n),
            Operation::Subtract => return find_human(monkeys, first_monkey, n + expected_number),
            Operation::Multiply => return find_human(monkeys, first_monkey, expected_number / n),
            Operation::Divide => return find_human(monkeys, first_monkey, n * expected_number),
        },

        _ => unimplemented!("Unexpected state!"),
    }
}
