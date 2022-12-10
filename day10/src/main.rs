use std::env;
use std::fs;

fn print_display(display: &Vec<Vec<char>>) {
    for line in display {
        println!(
            "{}",
            line.iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join("")
        );
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_path = args
        .get(1)
        .expect("Pass in input file path as the first argument!");

    let content = fs::read_to_string(file_path).expect("Could not read the input file!");

    let mut instructions: Vec<(&str, i32)> = Vec::new();
    for line in content.lines() {
        match line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["noop"] => instructions.push(("noop", 0)),
            ["addx", inc] => instructions.push(("addx", inc.parse::<i32>().unwrap())),
            _ => unimplemented!("Unknown instruction {line}."),
        }
    }
    instructions.reverse();

    let mut cycle: usize = 0;
    let mut reg_x: i32 = 1;

    let mut part1_result = 0;
    let mut part1_result_breakpoint = 20;

    let mut instruction: Option<(&str, i32)> = None;

    let mut display = vec![vec!['.'; 40]; 6];

    loop {
        cycle += 1;

        if cycle % part1_result_breakpoint == 0 {
            part1_result += (cycle as i32) * reg_x;
            part1_result_breakpoint += 40;
        }

        let pixel = ((cycle - 1) / 40, (cycle - 1) % 40);
        if (reg_x - 1..=reg_x + 1).contains(&(pixel.1 as i32)) {
            display[pixel.0][pixel.1] = '#';
        }

        if instruction != None {
            match instruction.unwrap() {
                ("addx", inc) => {
                    instruction = None;
                    reg_x += inc;
                }
                _ => unimplemented!(),
            }
        } else {
            instruction = instructions.pop();
            if instruction == None {
                println!("Execution finished!");
                break;
            }

            match instruction.unwrap() {
                ("noop", 0) => {
                    instruction = None;
                }
                ("addx", _) => {}
                _ => unimplemented!(),
            }
        }
    }

    println!("part 1: {part1_result}");

    print_display(&display);
}
