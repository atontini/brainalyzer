use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Parse {
        #[arg(short, long)]
        filepath: String,
    },
}