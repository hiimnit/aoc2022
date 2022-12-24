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
    // TODO HumanDependant
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

    // println!("digraph G {{");
    // println!("\thumn [shape=Mdiamond];");
    // for (monkey, job) in &monkeys {
    //     if let Job::Calculate(m1, _, m2) = job {
    //         println!("\t{m1} -> {monkey};");
    //         println!("\t{m2} -> {monkey};");
    //     }
    // }
    // println!("}}");

    let depths = calc_depths(&monkeys);

    let mut sorted_depths = depths.keys().collect::<Vec<&i64>>();
    sorted_depths.sort();
    for depth in sorted_depths {
        for monkey in &depths[depth] {
            let result: i64 = match &monkeys[monkey] {
                Job::Yell(n) => n.to_owned(),
                Job::Calculate(m1, operation, m2) => {
                    let operand1 = match &monkeys[m1] {
                        Job::Yell(n) => n,
                        _ => unimplemented!("Monkey {m1} has not yelled its number yet!"),
                    };
                    let operand2 = match &monkeys[m2] {
                        Job::Yell(n) => n,
                        _ => unimplemented!("Monkey {m2} has not yelled its number yet!"),
                    };

                    match operation {
                        Operation::Add => operand1 + operand2,
                        Operation::Subtract => operand1 - operand2,
                        Operation::Multiply => operand1 * operand2,
                        Operation::Divide => operand1 / operand2,
                    }
                }
            };

            *monkeys.get_mut(monkey).unwrap() = Job::Yell(result);
        }
    }

    println!("root yells {:?}", monkeys.get("root").unwrap());
    println!("rmtt yells {:?}", monkeys.get("rmtt").unwrap());
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
        Job::Yell(_) => 0,
        Job::Calculate(m1, _, m2) => max(calc_depth(monkeys, m1), calc_depth(monkeys, m2)) + 1,
    }
}
