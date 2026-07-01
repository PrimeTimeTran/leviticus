mod vfs;
use crate::vfs::*;
use std::process::Command;

fn main() {
    let home = std::env::var("HOME").expect("Could not find HOME directory");
    let mountpoint = format!("{}/KB/project/app/loi/crates/leviticus/fuse_fs", home);

    // 1. Force clear the mount point
    let _ = Command::new("diskutil")
        .args(["unmount", "force", &mountpoint])
        .status();

    // 2. Proceed
    println!("Mounting at: {}", mountpoint);
    let loi = LoiFs::default();

    // 3. Mount with background threads enabled for better stability
    if let Err(e) = fuser::mount2(loi, &mountpoint, &fuser::Config::default()) {
        eprintln!(
            "Mounting failed: {}. Try running: diskutil unmount force {}",
            e, mountpoint
        );
    }
}
