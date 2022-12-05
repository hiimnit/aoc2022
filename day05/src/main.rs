use std::env;
use std::fs;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_path = args
        .get(1)
        .expect("Pass in input file path as the first argument!");

    let content = fs::read_to_string(file_path).expect("Could not read the input file!");

    let mut lines = content.lines();

    let crates_block = lines
        .by_ref()
        .take_while(|&e| e != "")
        .collect::<Vec<&str>>();

    let crates_block_reversed = crates_block.iter().rev().collect::<Vec<&&str>>();

    let mut crates: Vec<Vec<char>> = Vec::new();

    for i in 0..crates_block_reversed[0].len() {
        let line = (0..crates_block_reversed.len()).fold(String::new(), |mut a, e| {
            a.push(crates_block_reversed[e].chars().nth(i).unwrap());
            a
        });

        let first_char = line.chars().nth(0).unwrap();
        if let Some(column) = first_char.to_string().parse::<usize>().ok() {
            // using parsed column to index crates is bad
            crates.push(Vec::new());

            line.chars().skip(1).filter(|&e| e != ' ').for_each(|e| {
                let _ = &crates[column - 1].push(e);
                ()
            });
        }
    }

    for line in lines {
        match line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["move", count_str, "from", from_str, "to", to_str] => {
                let maybe_usize: Vec<Result<usize, _>> = vec![count_str, from_str, to_str]
                    .into_iter()
                    .map(|e| e.parse::<usize>())
                    .collect();

                match maybe_usize[..] {
                    [Ok(count), Ok(from), Ok(to)] => {
                        // part one ->
                        // for _ in 0..count {
                        //     let moved_char = crates[from - 1].pop().unwrap();
                        //     crates[to - 1].push(moved_char);
                        // }
                        // part one <-

                        // part two ->
                        let removed_chars: Vec<char> = {
                            let from_len = &crates[from - 1].len();
                            let result = crates[from - 1]
                                .drain((from_len - count)..)
                                .into_iter()
                                .collect::<Vec<char>>();
                            result
                        };
                        let _ = &crates[to - 1].extend(removed_chars);
                        // part two <-
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
    }

    for (i, crt) in crates.iter().enumerate() {
        println!("stack #{}: {:?}", i, crt);
    }

    print!("result: ");
    crates
        .iter()
        .for_each(|e| print!("{}", e.last().unwrap_or(&' ')));
    println!("");
}
