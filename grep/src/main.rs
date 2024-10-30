use regex::Regex;
use std::env;
use std::fs::read_to_string;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: grep pattern filename");
        process::exit(1);
    }

    let [_, pattern, filename] = &args.as_slice() else {
        unreachable!();
    };

    let content = read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file: {}", err);
        process::exit(1);
    });

    let regex = Regex::new(pattern).unwrap();

    for (index, line) in content.lines().enumerate() {
        if let Some(mat) = regex.find(line) {
            println!("{}:{} {}", index + 1, mat.start(), line);
        };
    }
}

// $ rgrep Hello a.txt
// 55: Hello world. This is an exmaple text
