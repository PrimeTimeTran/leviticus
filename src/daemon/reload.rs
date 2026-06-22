use cli::{CliCommand, Context};

pub struct ReloadDaemon;

#[async_trait::async_trait]
impl CliCommand for ReloadDaemon {
    async fn run(&self, ctx: &Context) {
        if ctx.verbose {
            println!("🔄 Reloading daemon (verbose)");
        } else {
            println!("🔄 Reloading daemon");
        }

        // reload logic here
    }
}
