use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "leviticus",
    version,
    about = "leviticus daemon + CLI for system state, views, and explanation layers",
    long_about = None
)]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub verbose: bool,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Start,
    Status,
    Explain,
    ExplainDoc,
    View { name: String },
    ViewFork { name: String },
    Deps { name: String },
    Stop,
    Reload,
    ViewList,
}

pub fn parse() -> Cli {
    Cli::parse()
}
