use std::env;
use std::fs;
use std::process;

use rayon::prelude::*;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: grep pattern filename");
        process::exit(1);
    }

    let [_, content_pattern, file_pattern] = &args.as_slice() else {
        unreachable!();
    };

    let safe_content_pattern = wildcard_to_regex(content_pattern);

    let content_regex = Regex::new(&safe_content_pattern).unwrap_or_else(|err| {
        eprintln!("Invalid pattern: {}", err);
        process::exit(1);
    });

    let safe_file_pattern = wildcard_to_regex(file_pattern);

    let file_regex = Regex::new(&safe_file_pattern).unwrap_or_else(|err| {
        eprintln!("Invalid pattern: {}", err);
        process::exit(1);
    });

    let file_path_content_tuple: Vec<(std::path::PathBuf, String)> = fs::read_dir("./")
        .unwrap_or_else(|err| {
            eprintln!("Error reading directory: {}", err);
            process::exit(1);
        })
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file() && file_regex.is_match(path.file_name().unwrap().to_str().unwrap())
        })
        .par_bridge()
        .filter_map(|path| {
            fs::read_to_string(&path)
                .map(|content| (path, content))
                .ok()
        })
        .collect();

    for (path, content) in file_path_content_tuple {
        println!("file: {}", path.display());

        for (index, line) in content.lines().enumerate() {
            if let Some(mat) = content_regex.find(line) {
                println!(
                    "    {}:{} {}",
                    index + 1,
                    mat.start(),
                    highlight_keywords(&line, &content_regex)
                );
            };
        }
    }
}

fn wildcard_to_regex(pattern: &str) -> String {
    pattern
        .replace(".", "\\.")
        .replace("*", ".*")
        .replace("?", ".?")
}

fn highlight_keywords(content: &str, regex: &Regex) -> String {
    // 匹配字符丢失
    // .replace_all(content, "\x1b[31m$&\x1b[0m")
    regex
        .replace_all(content, |caps: &regex::Captures| {
            format!("\x1b[31m{}\x1b[0m", &caps[0])
        })
        .to_string()
}

// cargo run -p grep -- "Rust" "*.MD"
