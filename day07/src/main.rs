use std::collections::HashMap;
use std::env;
use std::fs;

struct FileSystem {
    directories: Vec<Directory>,

    current_dir: usize,
}

struct Directory {
    size: usize,
    parent: Option<usize>,
    children: HashMap<String, usize>,
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            directories: vec![Directory {
                size: 0,
                parent: None,
                children: HashMap::new(),
            }],
            current_dir: 0,
        }
    }

    pub fn get_root(&self) -> &Directory {
        &self.directories[0]
    }

    pub fn change_dir(&mut self, name: &str) {
        match name {
            "/" => {
                self.current_dir = 0;
            }
            ".." => {
                self.current_dir = self.directories[self.current_dir].parent.unwrap();
            }
            name => {
                let current_directory = &self.directories[self.current_dir];
                self.current_dir = *current_directory
                    .children
                    .get(name)
                    .expect(format!("Could not find child {name}").as_ref());
            }
        }
    }

    pub fn add_directory(&mut self, name: &str) {
        let new_directory = Directory {
            size: 0,
            parent: Some(self.current_dir),
            children: HashMap::new(),
        };

        self.directories.push(new_directory);

        let length = self.directories.len();
        self.directories[self.current_dir]
            .children
            .insert(name.to_string(), length - 1);
    }

    pub fn add_file(&mut self, size: usize) {
        self.add_size(size, self.current_dir);
    }

    fn add_size(&mut self, size: usize, i: usize) {
        self.directories[i].size += size;

        if let Some(i) = self.directories[i].parent {
            self.add_size(size, i);
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_path = args
        .get(1)
        .expect("Pass in input file path as the first argument!");

    let content = fs::read_to_string(file_path).expect("Could not read the input file!");

    let mut file_system = FileSystem::new();

    for line in content.lines() {
        match line.split_whitespace().into_iter().collect::<Vec<&str>>()[..] {
            ["$", "ls"] => {}
            ["$", "cd", name] => file_system.change_dir(name),
            ["dir", dir_name] => {
                file_system.add_directory(dir_name);
            }
            [size_str, _file_name] => {
                let size = size_str.parse::<usize>().unwrap();
                file_system.add_file(size);
            }
            _ => {
                unimplemented!();
            }
        }
    }

    let total = file_system
        .directories
        .iter()
        .filter(|e| e.size <= 100000)
        .fold(0, |acc, e| acc + e.size);

    println!("part 1: {}", total);

    let required_space = file_system.get_root().size - (70000000 - 30000000);

    let smallest_dir_over_req = file_system
        .directories
        .iter()
        .filter(|e| e.size >= required_space)
        .reduce(|acc, e| if acc.size < e.size { acc } else { e })
        .unwrap();

    println!(
        "part 2: {} (required_space: {})",
        smallest_dir_over_req.size, required_space
    );
}
