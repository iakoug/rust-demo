use clap::Parser;
use colored::*;
use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read, Stdout, Write},
    ops::Range,
    path::Path,
};

mod error;

pub use error::GrepError;

pub type StrategyFn<W, R> = fn(&Path, BufReader<R>, &Regex, &mut W) -> Result<(), GrepError>;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Your Name <your.email@example.com>")]
#[command(color = clap::ColorChoice::Always)]
pub struct GrepConfig {
    pattern: String,
    glob: String,
}

impl GrepConfig {
    pub fn match_with_default_strategy(&self) -> Result<(), GrepError> {
        self.match_with(default_strategy)
    }

    pub fn match_with(&self, strategy: StrategyFn<Stdout, File>) -> Result<(), GrepError> {
        let regex = Regex::new(&self.pattern)?;
        let files: Vec<_> = glob::glob(&self.glob)?.collect();

        files.into_par_iter().for_each(|v| {
            if let Ok(filename) = v {
                if let Ok(file) = File::open(&filename) {
                    let reader = BufReader::new(file);
                    let mut stdout = io::stdout();

                    if let Err(e) = strategy(&filename, reader, &regex, &mut stdout) {
                        eprintln!("Internal error:{:?}", e);
                    }
                }
            }
        });

        Ok(())
    }
}

pub fn default_strategy<W: Write, R: Read>(
    path: &Path,
    reader: BufReader<R>,
    pattern: &Regex,
    writer: &mut W,
) -> Result<(), GrepError> {
    let matches: String = reader
        .lines()
        .enumerate()
        .map(|(lineno, line)| {
            line.ok().map(|line| {
                pattern
                    .find(&line)
                    .map(|m| format_line(&line, lineno + 1, m.range()))
            })
        })
        .filter_map(|v| v.flatten())
        .join("\n");

    if !matches.is_empty() {
        writer.write(path.display().to_string().green().as_bytes())?;
        writer.write(b":")?;
        writer.write(matches.as_bytes())?;
        writer.write(b"\n")?;
    }

    Ok(())
}

pub fn format_line(line: &str, lineno: usize, range: Range<usize>) -> String {
    let Range { start, end } = range;
    let prefix = &line[..start];

    format!(
        "{0: >6}:{1: <3} {2}{3}{4}",
        lineno.to_string().blue(),
        (prefix.chars().count() + 1).to_string().cyan(),
        prefix,
        &line[start..end].red(),
        &line[end..],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_line_should_work() {
        let result = format_line("hello, world!", 1, 5..8);

        let expected = format!(
            "{0: >6}:{1: <3} {2}{3}{4}",
            "1".blue(),
            "6".cyan(),
            "hello",
            ", w".red(),
            "orld!",
        );

        assert_eq!(result, expected);
    }

    // #[test]
    // fn default_strategy_should_work() {
    //     let path = Path::new("src/main.rs");
    //     let input = b"hello world!\nhey Tyr!";
    //     let reader = BufReader::new(&input[..]);
    //     let pattern = Regex::new(r"he\w+").unwrap();
    //     let mut writer = Vec::new();

    //     default_strategy(path, reader, &pattern, &mut writer).unwrap();

    //     let result = String::from_utf8(writer).unwrap();
    //     let expected = format!(
    //         "{}{}\n{}\n",
    //         "src/main.rs".green(),
    //         format_line("hello world!", 1, 0..5),
    //         format_line("hey Tyr!", 2, 0..3)
    //     );

    //     assert_eq!(result, expected);
    // }
}
