use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    Add { path: Option<String> },
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}
