use std::{fs, io::Result, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::daemon::{config::derive_runtime_context, initialize::init, resolver::leviticus_dir};

const LEVITICUS_VERSION: &str = env!("CARGO_PKG_VERSION");
const SCHEMA_VERSION: u32 = 1;

fn manifest_path() -> PathBuf {
    leviticus_dir().join("manifest.json")
}

fn symbols_path() -> PathBuf {
    leviticus_dir().join("symbols.json")
}

fn write_manifest() -> Result<()> {
    let path = manifest_path();

    let manifest = Manifest {
        installed_version: LEVITICUS_VERSION.to_string(),
        schema_version: SCHEMA_VERSION,
        initialized_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
    };

    fs::write(path, serde_json::to_string_pretty(&manifest).unwrap())?;

    Ok(())
}

fn write_symbols() -> Result<()> {
    let path = symbols_path();
    let root = leviticus_dir();

    let symbols = serde_json::json!({
        "symbols": [
            {
                "id": "leviticus.root",
                "path": root.to_string_lossy(),
                "immutable": true,
                "doc": "Root directory of leviticus system"
            },
            {
                "id": "leviticus.manifest",
                "path": manifest_path().to_string_lossy(),
                "doc": "Installation + version state"
            },
            {
                "id": "leviticus.symbols",
                "path": path.to_string_lossy(),
                "doc": "Symbol registry for navigation"
            }
        ]
    });

    fs::write(path, serde_json::to_string_pretty(&symbols).unwrap())?;

    Ok(())
}

fn run_migrations(from: u32) -> Result<()> {
    println!("Running schema migrations from version {from}");

    match from {
        0 => {
            // legacy layout migration
        }
        1 => {
            // future
        }
        _ => {}
    }

    Ok(())
}

fn run_version_migrations(from: &str) -> Result<()> {
    println!("Upgrading leviticus from version {from} → {LEVITICUS_VERSION}");

    // future:
    // - rewrite symbol format
    // - update state schema
    // - rebuild caches

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Manifest {
    installed_version: String,
    schema_version: u32,
    initialized_at: i64,
}

fn ensure_initialized() -> Result<()> {
    let manifest_file = manifest_path();

    // -----------------------------
    // FIRST RUN
    // -----------------------------
    if !manifest_file.exists() {
        init()?; // creates ~/.leviticus structure

        write_manifest()?;
        write_symbols()?;

        return Ok(());
    }

    // -----------------------------
    // LOAD EXISTING
    // -----------------------------
    let raw = fs::read_to_string(&manifest_file)?;
    let manifest: Manifest = serde_json::from_str(&raw)?;

    // -----------------------------
    // SCHEMA MIGRATION
    // -----------------------------
    if manifest.schema_version < SCHEMA_VERSION {
        run_migrations(manifest.schema_version)?;
    }

    // -----------------------------
    // VERSION MIGRATION
    // -----------------------------
    if manifest.installed_version != LEVITICUS_VERSION {
        run_version_migrations(&manifest.installed_version)?;
        write_manifest()?;
    }

    Ok(())
}

pub fn bootstrap(path_workspace: PathBuf) {
    let _ = path_workspace;
    ensure_initialized();
    derive_runtime_context();
}
