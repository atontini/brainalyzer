mod brainfuck_parser;
mod cli;
mod commands;
mod config;
mod file_utils;

use brainfuck_parser::BrainfuckParser;
use clap::Parser;
use cli::Cli;
use commands::Commands;
use config::enable_debug_mode;
use file_utils::read_file_to_string;
use std::path::Path;

fn main() {
    let cli = Cli::parse();

    if cli.debug {
        enable_debug_mode();
    }

    match &cli.command {
        Commands::Parse { filepath } => {
            let mut parser = BrainfuckParser::new();
            parser.parse(&read_file_to_string(Path::new(&filepath)));
        }
    }
}
