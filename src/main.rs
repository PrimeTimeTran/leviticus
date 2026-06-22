use crate::router::execute;
mod daemon;
mod projection;
mod reg_command;
mod router;

use cli::Context;
use reg_command::{Cli, Command, parse};

#[tokio::main]
async fn main() {
    let cli = parse();

    let ctx = Context {
        verbose: cli.verbose,
    };
    execute(cli, ctx).await;
}
