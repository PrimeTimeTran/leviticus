// use crate::router::execute;
// mod daemon;
// mod projection;
// mod reg_command;
// mod router;

// use cli::Context;
// use reg_command::{Cli, Command, parse};

// #[tokio::main]
// async fn main() {
//     let cli = parse();

//     let ctx = Context {
//         verbose: cli.verbose,
//     };
//     execute(cli, ctx).await;
// }

mod vfs;
use crate::vfs::*;

fn main() {
    let home = std::env::var("HOME").expect("Could not find HOME directory");
    let mountpoint = format!("{}/KB/project/app/loi/crates/leviticus/fuse_fs", home);
    std::fs::create_dir_all(&mountpoint).expect("Failed to create mount point directory");
    println!("Mounting at: {}", mountpoint);
    fuser::mount2(LoiFs, mountpoint, &fuser::Config::default()).unwrap();
}
