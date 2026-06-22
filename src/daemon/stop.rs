use cli::{CliCommand, Context};

pub struct StopDaemon;

#[async_trait::async_trait]
impl CliCommand for StopDaemon {
    async fn run(&self, ctx: &Context) {
        if ctx.verbose {
            println!("🛑 Stopping daemon (verbose)");
        } else {
            println!("🛑 Stopping daemon");
        }
    }
}
