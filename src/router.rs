use cli::CliCommand;
use cli::Context;

use crate::daemon::{ReloadDaemon, StartDaemon, StatusDaemon, StopDaemon};

use crate::projection::command::{Deps, Explain, ExplainDoc, View, ViewFork, ViewList};
use crate::reg_command::{Cli, Command};

pub async fn execute(cli: Cli, ctx: Context) {
    match cli.command {
        Command::Start => StartDaemon.run(&ctx).await,
        Command::Status => StatusDaemon.run(&ctx).await,
        Command::Stop => StopDaemon.run(&ctx).await,
        Command::Reload => ReloadDaemon.run(&ctx).await,

        Command::Explain => Explain.run(&ctx).await,
        Command::ExplainDoc => ExplainDoc.run(&ctx).await,

        Command::View { name } => View { name }.run(&ctx).await,
        Command::ViewFork { name } => ViewFork { name }.run(&ctx).await,
        Command::ViewList => ViewList.run(&ctx).await,

        Command::Deps { name } => Deps { name }.run(&ctx).await,
    }
}
