use std::collections::HashMap;
use std::env;
use std::fs;

// this was originally supposed to be much nicer/cleaner
// but then some closure issues appeared
// and then part 2 happened

#[derive(Debug)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test: Test,
    inspection_count: i64,
}

#[derive(Debug)]
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

    pub fn call(self: &Self, i: i64) -> i64 {
        let first = match self.first.as_ref() {
            "old" => i,
            number => number
                .parse::<i64>()
                .expect(format!("Could not parse {} as i64.", self.first).as_ref()),
        };
        let op = Box::new(match self.op.as_ref() {
            "+" => |a: i64, b: i64| a + b,
            "-" => |a: i64, b: i64| a - b,
            "*" => |a: i64, b: i64| a * b,
            "/" => |a: i64, b: i64| a / b,
            _ => unimplemented!("Unexpected operator."),
        });
        let second = match self.second.as_ref() {
            "old" => i,
            number => number
                .parse::<i64>()
                .expect(format!("Could not parse {} as i64.", self.second).as_ref()),
        };

        op(first, second)
    }
}

#[derive(Debug)]
struct Test {
    divisor: i64,
    on_true: usize,
    on_false: usize,
}

impl Test {
    pub fn new(divisor: i64, on_true: usize, on_false: usize) -> Self {
        Self {
            divisor,
            on_true,
            on_false,
        }
    }

    pub fn call(self: &Self, i: i64) -> usize {
        if i % self.divisor == 0 {
            return self.on_true;
        }
        self.on_false
    }
}

impl Monkey {
    pub fn new(items: Vec<Item>, operation: Operation, test: Test) -> Self {
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
                .map(|e| Item::new(e.trim().parse::<i64>().unwrap()))
                .collect::<Vec<Item>>(),
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
            ["Test:", "divisible", "by", number] => number.parse::<i64>().unwrap(),
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

    let divisors = monkeys.iter().map(|e| e.test.divisor).collect::<Vec<i64>>();

    for monkey in &mut monkeys {
        for item in &mut monkey.items {
            for divisor in &divisors {
                item.add_test(divisor.clone())
            }
        }
    }

    for r in 0..10000 {
        if r % 1000 == 0 {
            println!("round {r}");
        }

        for i in 0..monkeys.len() {
            loop {
                let item_option = monkeys[i].items.pop();
                if let None = item_option {
                    break;
                }
                let mut item = item_option.unwrap();

                item.update(&monkeys[i].operation);

                let new_index = if item.tests[&monkeys[i].test.divisor].divisible {
                    monkeys[i].test.on_true
                } else {
                    monkeys[i].test.on_false
                };

                monkeys[i].inspection_count += 1;
                monkeys[new_index].items.push(item);
            }
        }
    }

    let mut monkey_business = monkeys
        .iter()
        .map(|e| e.inspection_count)
        .collect::<Vec<i64>>();
    monkey_business.sort();

    let result: i64 = monkey_business.iter().rev().take(2).product();

    println!("part 2: {result}");
}

// the idea is that since the only operations are multiplication and addition
// we can store only the remainder for each monkey
// each inspection then updates all posible results for each monkey for current item

// (A * B) mod C = (A mod C * B mod C) mod C
// (A + B) mod C = (A mod C + B mod C) mod C

#[derive(Debug)]
struct Item {
    original: i64,
    tests: HashMap<i64, ItemTest>,
}

impl Item {
    pub fn new(original: i64) -> Self {
        Item {
            original,
            tests: HashMap::new(),
        }
    }

    pub fn add_test(self: &mut Self, divisor: i64) {
        self.tests
            .insert(divisor, ItemTest::new(divisor, self.original));
    }

    pub fn update(self: &mut Self, operation: &Operation) {
        for (_, test) in &mut self.tests {
            test.update(operation);
        }
    }
}

#[derive(Debug)]
struct ItemTest {
    divisor: i64,
    n: i64,
    divisible: bool,
}

impl ItemTest {
    pub fn new(divisor: i64, n: i64) -> Self {
        ItemTest {
            divisor,
            n,
            divisible: n % divisor == 0,
        }
    }

    pub fn update(self: &mut Self, operation: &Operation) {
        self.n = operation.call(self.n) % self.divisor;
        self.divisible = self.n == 0;
    }
}
