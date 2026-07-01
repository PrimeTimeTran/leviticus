mod vfs;
use crate::vfs::*;

// fn main() {
//     let home = std::env::var("HOME").expect("Could not find HOME directory");
//     let mountpoint = format!("{}/KB/project/app/loi/crates/leviticus/fuse_fs", home);
//     std::fs::create_dir_all(&mountpoint).expect("Failed to create mount point directory");
//     println!("Mounting at: {}", mountpoint);
//     fuser::mount2(LoiFs, mountpoint, &fuser::Config::default()).unwrap();
// }

// fn main() {
//     let home = std::env::var("HOME").expect("Could not find HOME directory");
//     let mountpoint = format!("{}/KB/project/app/loi/crates/leviticus/fuse_fs", home);

//     // 1. Don't panic if it exists; just ensure it's a directory.
//     if let Err(e) = std::fs::create_dir_all(&mountpoint) {
//         eprintln!("Failed to create directory: {}", e);
//         return;
//     }

//     println!("Mounting at: {}", mountpoint);

//     // 2. Use a match to handle potential mount errors (like the volume already being mounted)
//     // let options = vec![fuser::MountOption::Default];
//     if let Err(e) = fuser::mount2(LoiFs, &mountpoint, &fuser::Config::default()) {
//         eprintln!("Mounting failed (did you umount the old one?): {}", e);
//     }
// }

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
