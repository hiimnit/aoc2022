use std::env;
use std::fs;
use std::str::Chars;
use std::vec;

const CAVERN_WIDTH: usize = 7;

#[derive(Clone, Debug)]
enum Point {
    Air,
    Rock,
}

impl Point {
    pub fn to_string(self: &Self) -> String {
        match self {
            Point::Air => ".".to_string(),
            Point::Rock => "#".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
enum Motion {
    Left,
    Right,
    Down,
}

impl Motion {
    pub fn from_char(c: char) -> Self {
        match c {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => unimplemented!("Unknown motion {c}!"),
        }
    }

    pub fn get_next_index(self: &Self, index: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Motion::Left => {
                if index.1 == 0 {
                    return None;
                }
                Some((index.0, index.1 - 1))
            }
            Motion::Right => {
                if index.1 >= CAVERN_WIDTH - 1 {
                    return None;
                }
                Some((index.0, index.1 + 1))
            }
            Motion::Down => {
                if index.0 == 0 {
                    return None;
                }
                Some((index.0 - 1, index.1))
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Shape {
    HorizontalLine,
    Cross,
    FlippedL,
    VerticalLine,
    Square,
}

impl Shape {
    pub fn is_motion_possible(
        self: &Self,
        cavern: &Cavern,
        index: (usize, usize),
        motion: Motion,
    ) -> Option<(usize, usize)> {
        let next_index = motion.get_next_index(index)?;

        // TODO add diferent `get_indices` just for collision checking?
        if self
            .get_indices(next_index)
            .into_iter()
            .all(|e| cavern.is_point_empty(e))
        {
            return Some(next_index);
        }

        None
    }

    pub fn get_indices(self: &Self, index: (usize, usize)) -> Vec<(usize, usize)> {
        match self {
            // ####
            Shape::HorizontalLine => vec![
                (index.0, index.1),
                (index.0, index.1 + 1),
                (index.0, index.1 + 2),
                (index.0, index.1 + 3),
            ],
            // .#.
            // ###
            // .#.
            Shape::Cross => vec![
                (index.0 + 1, index.1),
                (index.0 + 2, index.1 + 1),
                (index.0 + 1, index.1 + 1),
                (index.0, index.1 + 1),
                (index.0 + 1, index.1 + 2),
            ],
            // ..#
            // ..#
            // ###
            Shape::FlippedL => vec![
                (index.0, index.1),
                (index.0, index.1 + 1),
                (index.0 + 2, index.1 + 2),
                (index.0 + 1, index.1 + 2),
                (index.0, index.1 + 2),
            ],
            // #
            // #
            // #
            // #
            Shape::VerticalLine => vec![
                (index.0 + 3, index.1),
                (index.0 + 2, index.1),
                (index.0 + 1, index.1),
                (index.0, index.1),
            ],
            // ##
            // ##
            Shape::Square => vec![
                (index.0 + 1, index.1),
                (index.0, index.1),
                (index.0 + 1, index.1 + 1),
                (index.0, index.1 + 1),
            ],
        }
    }
}

#[derive(Debug)]
struct Cavern {
    motion_sequence: Vec<Motion>,
    motion_index: usize,
    board: Vec<Vec<Point>>,
    rock_counter: i64,
}

impl Cavern {
    pub fn new(air_sequence: &str) -> Self {
        Self {
            motion_sequence: air_sequence.chars().map(|e| Motion::from_char(e)).collect(),
            motion_index: 0,
            board: vec![],
            rock_counter: 0,
        }
    }

    pub fn height(self: &Self) -> usize {
        self.board.len()
    }

    pub fn pretty_print(self: &Self) {
        for line in self.board.iter().rev() {
            println!(
                "|{}|",
                line.iter().map(|e| e.to_string()).collect::<String>()
            );
        }
        println!("+{}+", vec!["-"; CAVERN_WIDTH].join(""));
    }

    pub fn is_point_empty(self: &Self, index: (usize, usize)) -> bool {
        if CAVERN_WIDTH <= index.1 {
            // index is out of bounds => NOT empty
            return false;
        }
        if self.board.len() <= index.0 {
            // line does not exist (yet) => empty
            return true;
        }

        match self.board[index.0][index.1] {
            Point::Air => true,
            Point::Rock => false,
        }
    }

    fn get_next_motion(self: &mut Self) -> Motion {
        // print state at breakpoints
        if self.rock_counter == 1748 || self.rock_counter == 3488 || self.rock_counter == 4660 {
            println!(
                "x, height is {}, rock_counter: {}",
                self.height(),
                self.rock_counter
            );
        }

        let next_motion = self.motion_sequence[self.motion_index].clone();
        self.motion_index = (self.motion_index + 1) % self.motion_sequence.len();
        next_motion
    }

    pub fn add_rock(self: &mut Self, shape: &Shape) {
        let mut index: (usize, usize) = (self.board.len() + 3, 2);

        loop {
            let motion = self.get_next_motion();

            if let Some(next_index) = shape.is_motion_possible(&self, index, motion) {
                index = next_index;
            }

            match shape.is_motion_possible(&self, index, Motion::Down) {
                Some(next_index) => index = next_index,
                None => {
                    self.place_rock(shape, index);
                    return;
                }
            }
        }
    }

    fn place_rock(self: &mut Self, shape: &Shape, index: (usize, usize)) {
        let indices = shape.get_indices(index);
        for index in indices {
            while index.0 >= self.board.len() {
                self.board.push(vec![Point::Air; CAVERN_WIDTH]);
            }

            self.board[index.0][index.1] = Point::Rock;
        }
        self.rock_counter += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = args
        .get(1)
        .expect("Input file path expected as the first argument!");

    let content = fs::read_to_string(input_file_path).expect("Could not read the input file!");

    let mut cavern = Cavern::new(&content);

    let shapes = vec![
        Shape::HorizontalLine,
        Shape::Cross,
        Shape::FlippedL,
        Shape::VerticalLine,
        Shape::Square,
    ];

    for i in 0..2022 {
        cavern.add_rock(&shapes[i % shapes.len()]);
    }

    println!("part 1: {}", cavern.height());

    let mut cavern = Cavern::new(&content);

    let shapes = vec![
        Shape::HorizontalLine,
        Shape::Cross,
        Shape::FlippedL,
        Shape::VerticalLine,
        Shape::Square,
    ];

    for i in 0..100000000 {
        cavern.add_rock(&shapes[i % shapes.len()]);
    }

    println!("part 2: {}", cavern.height());
}

// SAMPLE:
// after the first million rocks, the height is 1514288
// then each next sevent million rocks add 10600000
// 1514288 + 10600000 * 142857 = 1 514 285 714 288

// looking at input, first cycle of all inputs increases the height by 2802
// each subsequent cycle then by 2754
// 1748 + 574 712 642 * 1740 + 1172 = 1 000 000 000 000
// that leaves the us to look at the final 1172 rocks, which add up to height of 1831
// 2802 + 574 712 642 * 2754 + 1831 = 1 582 758 620 701
