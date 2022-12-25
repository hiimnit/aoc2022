use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = args
        .get(1)
        .expect("Input file path expected as the first argument!");

    let content = fs::read_to_string(input_file_path).expect("Could not read the input file!");

    let (map, instructions_str) = match content.split("\n\n").collect::<Vec<&str>>()[..] {
        [map, instructions] => (map, instructions),
        _ => unimplemented!("Invalid input!"),
    };

    let mut map = Map::new(map);
    // map.pretty_print();

    let mut instructions: Vec<Instruction> = vec![];
    let mut current_number: Option<u32> = None;
    for ch in instructions_str.chars() {
        match ch {
            'R' => {
                if let Some(number) = current_number {
                    instructions.push(Instruction::Move(number));
                    current_number = None;
                };
                instructions.push(Instruction::Turn(Turn::Clockwise));
            }
            'L' => {
                if let Some(number) = current_number {
                    instructions.push(Instruction::Move(number));
                    current_number = None;
                };
                instructions.push(Instruction::Turn(Turn::Counterclockwise));
            }
            n => {
                let digit = n.to_digit(10).unwrap();
                current_number = match current_number {
                    Some(number) => Some(number * 10 + digit),
                    None => Some(digit),
                }
            }
        }
    }
    if let Some(number) = current_number {
        instructions.push(Instruction::Move(number));
    };

    for instruction in instructions {
        map.process_instruction(instruction);
    }

    println!("part 1: {}", map.calculate_password());
}

#[derive(Clone, PartialEq)]
enum Point {
    Nothing,
    Open,
    Solid,
}

impl Point {
    pub fn to_string(self: &Self) -> String {
        match self {
            Point::Nothing => " ".to_owned(),
            Point::Open => ".".to_owned(),
            Point::Solid => "#".to_owned(),
        }
    }
}

enum Instruction {
    Move(u32),
    Turn(Turn),
}

enum Turn {
    Clockwise,
    Counterclockwise,
}

#[derive(Clone, PartialEq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    pub fn to_string(self: &Self) -> String {
        match self {
            Direction::Right => ">".to_owned(),
            Direction::Down => "v".to_owned(),
            Direction::Left => "<".to_owned(),
            Direction::Up => "^".to_owned(),
        }
    }

    pub fn turn(self: &Self, turn: Turn) -> Self {
        match turn {
            Turn::Clockwise => match self {
                Self::Right => Self::Down,
                Self::Down => Self::Left,
                Self::Left => Self::Up,
                Self::Up => Self::Right,
            },
            Turn::Counterclockwise => match self {
                Self::Right => Self::Up,
                Self::Down => Self::Right,
                Self::Left => Self::Down,
                Self::Up => Self::Left,
            },
        }
    }

    pub fn delta(self: &Self) -> (i32, i32) {
        match self {
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
        }
    }
}

struct Map {
    points: Vec<Vec<Point>>,
    position: (usize, usize),
    direction: Direction,
}

impl Map {
    pub fn new(map: &str) -> Self {
        let width = map.lines().map(|e| e.len()).max().unwrap();
        let mut points = vec![];

        for line in map.lines() {
            let mut row = vec![Point::Nothing; width];

            for (i, ch) in line.chars().enumerate() {
                row[i] = match ch {
                    ' ' => Point::Nothing,
                    '.' => Point::Open,
                    '#' => Point::Solid,
                    _ => unimplemented!("Unknown map point {ch}!"),
                };
            }

            points.push(row);
        }

        let player_position = points[0].iter().position(|e| *e == Point::Open).unwrap();

        Self {
            points,
            position: (0, player_position),
            direction: Direction::Right,
        }
    }

    pub fn pretty_print(self: &Self) {
        for (i, row) in self.points.iter().enumerate() {
            println!(
                "{}",
                row.iter()
                    .enumerate()
                    .map(|(j, e)| if i == self.position.0 && j == self.position.1 {
                        self.direction.to_string()
                    } else {
                        e.to_string()
                    })
                    .collect::<String>()
            );
        }
    }

    pub fn process_instruction(self: &mut Self, instruction: Instruction) {
        let number = match instruction {
            Instruction::Move(number) => number,
            Instruction::Turn(turn) => {
                self.direction = self.direction.turn(turn);
                return;
            }
        };

        for _ in 0..number {
            let mut next_position = self.get_next_position(self.position);

            while self.points[next_position.0][next_position.1] == Point::Nothing {
                next_position = self.get_next_position(next_position);
            }

            match self.points[next_position.0][next_position.1] {
                Point::Nothing => unimplemented!("Unexpected input!"),
                Point::Open => {}
                Point::Solid => break,
            }

            self.position = next_position;
        }
    }

    fn get_next_position(self: &Self, position: (usize, usize)) -> (usize, usize) {
        let delta = self.direction.delta();
        let mut next_position = (position.0 as i32 + delta.0, position.1 as i32 + delta.1);

        if next_position.0 < 0 {
            next_position.0 = (self.points.len() - 1) as i32;
        }
        if next_position.0 >= self.points.len() as i32 {
            next_position.0 = 0;
        }

        if next_position.1 < 0 {
            next_position.1 = (self.points[0].len() - 1) as i32;
        }
        if next_position.1 >= self.points[0].len() as i32 {
            next_position.1 = 0;
        }

        (next_position.0 as usize, next_position.1 as usize)
    }

    pub fn calculate_password(self: &Self) -> usize {
        let direction = match self.direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };

        (self.position.0 + 1) * 1000 + (self.position.1 + 1) * 4 + direction
    }
}
