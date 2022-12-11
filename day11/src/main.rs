use std::collections::HashMap;
use std::env;
use std::fs;

struct Monkey {
    items: Vec<i32>,
    operation: Operation,
    test: Test,
    inspection_count: i32,
}

struct Operation {
    first: String,
    op: String,
    second: String,
}

impl Operation {
    pub fn new(first: &str, op: &str, second: &str) -> Self {
        Operation {
            first: first.to_string(),
            op: op.to_string(),
            second: second.to_string(),
        }
    }

    pub fn call(self: &Self, i: i32) -> i32 {
        let first = match self.first.as_ref() {
            "old" => i,
            number => number
                .parse::<i32>()
                .expect(format!("Could not parse {} as i32.", self.first).as_ref()),
        };
        let op = Box::new(match self.op.as_ref() {
            "+" => |a: i32, b: i32| a + b,
            "-" => |a: i32, b: i32| a - b,
            "*" => |a: i32, b: i32| a * b,
            "/" => |a: i32, b: i32| a / b,
            _ => unimplemented!("Unexpected operator."),
        });
        let second = match self.second.as_ref() {
            "old" => i,
            number => number
                .parse::<i32>()
                .expect(format!("Could not parse {} as i32.", self.second).as_ref()),
        };

        op(first, second)
    }
}

struct Test {
    divisor: i32,
    on_true: usize,
    on_false: usize,
}

impl Test {
    pub fn new(divisor: i32, on_true: usize, on_false: usize) -> Self {
        Self {
            divisor,
            on_true,
            on_false,
        }
    }

    pub fn call(self: &Self, i: i32) -> usize {
        if i % self.divisor == 0 {
            return self.on_true;
        }
        self.on_false
    }
}

impl Monkey {
    pub fn new(items: Vec<i32>, operation: Operation, test: Test) -> Self {
        Monkey {
            items,
            operation,
            test,
            inspection_count: 0,
        }
    }

    pub fn from_string(string: &str) -> (usize, Self) {
        let mut lines = string.lines();
        let number = match lines
            .next()
            .unwrap()
            .replace(":", "")
            .split_whitespace()
            .collect::<Vec<&str>>()[..]
        {
            ["Monkey", number] => number.parse::<usize>().expect("Expected monkey's number."),
            _ => unimplemented!("Could not find the monkey's number."),
        };

        let items = match lines
            .next()
            .unwrap()
            .trim()
            .split(":")
            .collect::<Vec<&str>>()[..]
        {
            ["Starting items", items] => items
                .split(",")
                .map(|e| e.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
            _ => unimplemented!("Could not find the monkey's items."),
        };

        let (first, op, second) = match lines
            .next()
            .unwrap()
            .trim()
            .split(":")
            .collect::<Vec<&str>>()[..]
        {
            ["Operation", operation] => {
                let ops: Vec<&str> = operation.split_whitespace().collect();

                match ops[..] {
                    ["new", "=", first, op, second] => (first, op, second),
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!("Could not find the monkey's inspect operation."),
        };

        let test_divisor = match lines
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()[..]
        {
            ["Test:", "divisible", "by", number] => number.parse::<i32>().unwrap(),
            _ => unimplemented!(),
        };
        let test_true = match lines
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()[..]
        {
            ["If", "true:", "throw", "to", "monkey", number] => number.parse::<usize>().unwrap(),
            _ => unimplemented!(),
        };
        let test_false = match lines
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()[..]
        {
            ["If", "false:", "throw", "to", "monkey", number] => number.parse::<usize>().unwrap(),
            _ => unimplemented!(),
        };

        (
            number,
            Monkey::new(
                items,
                Operation::new(first, op, second),
                Test::new(test_divisor, test_true, test_false),
            ),
        )
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_path = args
        .get(1)
        .expect("Pass in input file path as the first argument!");

    let content = fs::read_to_string(file_path).expect("Could not read the input file!");

    let mut monkeys: Vec<Monkey> = Vec::new();

    for block in content.split("\n\n") {
        let (i, monkey) = Monkey::from_string(block);
        monkeys.insert(i, monkey);
    }

    for r in 0..10000 {
        println!("Round {r}");

        for i in 0..monkeys.len() {
            // println!("Monkey {i}");

            for j in 0..monkeys[i].items.len() {
                // println!(
                //     "\tMonkey inspects item with a worry level of {}.",
                //     monkeys[i].items[j]
                // );

                let item = monkeys[i].items[j];
                let result = monkeys[i].operation.call(item); // / 3;
                let new_index = monkeys[i].test.call(result);

                // println!(
                //     "\t\tNew worry level: {result} / 3 = {}, test {} = {} {}, passed to monkey {new_index}.",
                //     result / 3,
                //     monkeys[i].test.divisor,
                //     result % monkeys[i].test.divisor,
                //     result % monkeys[i].test.divisor == 0
                // );

                monkeys[i].inspection_count += 1;
                monkeys[new_index].items.push(result);
            }
            monkeys[i].items.clear();
        }
    }

    let mut monkey_business = monkeys
        .iter()
        .map(|e| e.inspection_count)
        .collect::<Vec<i32>>();
    monkey_business.sort();

    let result: i32 = monkey_business.iter().rev().take(2).product();

    println!("part 1: {result}");
}

// 23
// 19
// 13
// 17

// item is n
// n *
// nasobek min?

// n % _ == 0
// 23 * 19 * 13 * 17 * x % _ == 0
// 23 * 19 * 13 * 17 % _ == 0

// (23 * 19 * 13 * 17 * x) + 1 % _ == (23 * 19 * 13 * 17) + 1 % _

// 13 7 3 19 5 2 11 17

// (23 * 19 * 13 * 17 * x) + 1

// (A + B) mod C = (A mod C + B mod C) mod C
// => just store A? for each item for each monkey
// (A * B) mod C = (A mod C * B mod C) mod C

struct Item {
    tests: HashMap<i32, ItemTest>,
}

impl Item {
    pub fn new() -> Self {
        Item {
            tests: HashMap::new(),
        }
    }
}

struct ItemTest {
    divisor: i32,
    n: i32,
    divisible: bool,
}

impl ItemTest {
    pub fn new(divisor: i32, n: i32) -> Self {
        ItemTest {
            divisor,
            n,
            divisible: n % divisor == 0,
        }
    }

    pub fn update(self: &mut Self, operation: &Operation) {
        self.n = operation.call(self.n) % self.divisor;
    }
}
