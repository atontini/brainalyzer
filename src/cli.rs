use clap::Parser;
use crate::commands::Commands;

#[derive(Parser)]
#[command(name = "brainalyzer")]
#[command(about = "A simple brainfuck parser", long_about = None)]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub debug: bool,
    
    #[command(subcommand)]
    pub command: Commands,
}
