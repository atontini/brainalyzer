mod brainfuck_parser;
mod cli;
mod commands;
mod file_utils;

use brainfuck_parser::BrainfuckParser;
use clap::Parser;
use cli::Cli;
use commands::Commands;
use file_utils::read_file_to_string;
use once_cell::sync::Lazy;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

static DEBUG_MODE: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

fn main() {
    let cli = Cli::parse();

    if cli.debug {
        println!("Debug mode enabled");
        DEBUG_MODE.store(true, Ordering::Relaxed);
    }

    match &cli.command {
        Commands::Parse { filepath } => {
            let mut parser = BrainfuckParser::new();
            parser.parse(&read_file_to_string(Path::new(&filepath)));
        }
    }
}
