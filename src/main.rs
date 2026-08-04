mod vfs;
use crate::vfs::*;
use std::process::Command;

fn main() {
	let home = std::env::var("HOME").expect("Could not find HOME directory");
	let mountpoint = format!("{}/KB/project/app/loi/crates/leviticus/fuse_fs", home);
	let storage_path = format!("{}/KB/project/app/loi/data", home);
	std::fs::create_dir_all(&storage_path).unwrap();
	let _ = std::process::Command::new("diskutil")
		.args(["unmount", "force", &mountpoint])
		.status();

	println!("Mounting at: {}", mountpoint);
	let loi = LoiFs::load_from_disk(&storage_path);
	let config = fuser::Config::default();
	fuser::mount2(loi, &mountpoint, &config).unwrap();
}

enum Context {
	ZedEditor,
	CompilerPipeline,
	KnowledgeBase,
}

// impl LoiFs {
//     // This function returns the list of files based on current mode
//     fn get_projection(&self) -> Vec<Projection> {
//         match self.current_context {
//             Context::ZedEditor => vec![...], // Projects zed/ and rustc/ to root
//             Context::CompilerPipeline => vec![...], // Projects specific .md files
//             Context::KnowledgeBase => vec![...], // Projects snippets/
//         }
//     }
// }
