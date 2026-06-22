use clap::{Parser, Subcommand};

/// leviticus - unified runtime/state daemon for views, files, and tool resolution
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

/// All supported CLI commands
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Start the leviticus daemon in the foreground
    ///
    /// Example:
    ///   leviticus start
    #[command(alias = "up")]
    Start,

    /// Show current daemon state (health, runtime, stats)
    ///
    /// Example:
    ///   leviticus status
    ///   leviticus st
    #[command(alias = "st")]
    Status,

    /// Explain why the system resolved to its current state
    ///
    /// This is your "why did this happen?" layer:
    /// - dependency overrides
    /// - view resolution
    /// - config precedence
    ///
    /// Example:
    ///   leviticus explain
    ///   leviticus why
    #[command(alias = "why")]
    Explain,

    #[command(alias = "why")]
    ExplainDoc,

    /// (future) switch active view
    ///
    /// Example:
    ///   leviticus view rust-dev
    // #[command(alias = "v")]
    // View {
    //     /// Name of the view to activate
    //     name: String,
    // },
    View {
        #[arg(value_name = "VIEW_NAME")]
        name: String,
    },

    ViewFork {
        #[arg(value_name = "VIEW_NAME")]
        name: String,
    },

    /// (future) inspect dependency resolution graph
    ///
    /// Example:
    ///   leviticus deps
    #[command(alias = "d")]
    Deps,

    // #[command(alias = "d")]
    Stop,
    // #[command(alias = "d")]
    Reload,
    // #[command(alias = "d")]
    ViewList,
}

/// Convenience wrapper so main.rs stays clean
pub fn parse() -> Cli {
    Cli::parse()
}
