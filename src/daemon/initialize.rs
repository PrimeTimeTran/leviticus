use std::{fs, io::Result, path::PathBuf};

use crate::daemon::resolver::leviticus_dir;

pub fn init() -> Result<()> {
    let root = leviticus_dir();

    let dirs = [
        root.join("registry"),
        root.join("views"),
        root.join("cache/indexes"),
        root.join("cache/projections"),
        root.join("logs"),
    ];

    for dir in dirs {
        fs::create_dir_all(dir)?;
    }

    let files = [
        (
            root.join("config.toml"),
            r#"# leviticus configuration

            version = 1
            "#,
        ),
        (
            root.join("state.json"),
            r#"{
                "starts": 0,
                "started_at": 0,
                "longest_run": 0
                }
                "#,
        ),
        (root.join("daemon.pid"), ""),
        (root.join("socket"), ""),
        (root.join("registry/graph.db"), ""),
        (
            root.join("views/default.toml"),
            r#"# Default view

            name = "default"
            "#,
        ),
        (root.join("logs/leviticus.log"), ""),
    ];

    for (path, contents) in files {
        if !path.exists() {
            fs::write(path, contents)?;
        }
    }

    println!("Initialized leviticus at {}", root.display());

    Ok(())
}
