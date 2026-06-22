use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DaemonState {
    pub starts: u64,
    pub started_at: u64,
    pub longest_run: u64,
}

impl DaemonState {
    pub fn load() -> DaemonState {
        let p = path();

        if !p.exists() {
            return Default::default();
        }

        serde_json::from_str(&fs::read_to_string(p).unwrap()).unwrap()
    }

    pub fn save(state: &DaemonState) {
        let p = path();

        fs::create_dir_all(p.parent().unwrap()).unwrap();

        fs::write(p, serde_json::to_string_pretty(state).unwrap()).unwrap();
    }

    pub fn save_workspace(path: &PathBuf) {
        println!("💾 save_workspace not implemented yet: {:?}", path);
    }

    pub fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

fn path() -> PathBuf {
    dirs::home_dir()
        .unwrap()
        .join(".leviticus")
        .join("state.json")
}
