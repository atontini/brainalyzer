use clap::Parser;
use crate::commands::Commands;

#[derive(Parser)]
#[command(name = "brainalyzer")]
#[command(about = "A simple brainfuck parser", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
