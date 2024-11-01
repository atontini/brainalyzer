mod brainfuck_parser;
mod command;
mod file_utils;

use brainfuck_parser::BrainfuckParser;
use file_utils::read_file_to_string;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let input = args.join(" ");

    match command::parse_run_command(&input) {
        Ok((_, filepath)) => {
            match read_file_to_string(filepath) {
                Ok(contents) => {
                    let mut parser = BrainfuckParser::new();
                    parser.parse(&contents);
                },
                Err(error) => eprintln!("Error reading file: {}", error),
            }
        },
        Err(err) => eprintln!("Error parsing command: {:?}", err),
    }
}