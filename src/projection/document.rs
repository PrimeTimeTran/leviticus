use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    daemon::resolver::{leviticus_root, project_root},
    state::state,
};

fn file_link(path: &Path) -> String {
    let p = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

    format!("file://{}", p.to_string_lossy())
}

fn data_dir() -> PathBuf {
    PathBuf::from("src/daemon/data")
}

fn explain_json_path() -> PathBuf {
    data_dir().join("explain.json")
}

fn manifest_json_path() -> PathBuf {
    data_dir().join("manifest.json")
}

fn symbols_json_path() -> PathBuf {
    data_dir().join("symbols.json")
}

fn read_symbols() -> serde_json::Value {
    let path = symbols_json_path();

    let raw = fs::read_to_string(path).unwrap_or_else(|_| "[]".to_string());

    serde_json::from_str(&raw).unwrap_or_else(|_| serde_json::json!([]))
}

fn symbol_to_md(symbol: &serde_json::Value) -> String {
    let id = symbol
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    let path = symbol.get("path").and_then(|v| v.as_str()).unwrap_or("");

    let abs_path = format!("/Users/me/dev/leviticus/{}", path);

    format!(
        "- [`{}`]({}) — {}",
        id,
        file_link(&PathBuf::from(abs_path)),
        symbol.get("doc").and_then(|v| v.as_str()).unwrap_or("")
    )
}
fn render_symbols(json: &serde_json::Value) -> String {
    let mut out = String::new();

    out.push_str("## Symbols (Clickable)\n\n");

    // -------------------------
    // FILE REGISTRY (project)
    // -------------------------
    if let Some(fs) = json.get("filesystem") {
        if let Some(map) = fs.get("uid_mapping").and_then(|v| v.as_object()) {
            out.push_str("### File Registry\n\n");

            let project_root = project_root();

            for (uid, path) in map {
                let rel = path.as_str().unwrap_or("");

                let full = project_root.join(rel);

                out.push_str(&format!("- `{}` → [{}]({})\n", uid, rel, file_link(&full)));
            }

            out.push_str("\n");
        }
    }

    // -------------------------
    // GLOBAL SYMBOLS (leviticus system)
    // -------------------------
    let symbols_path = leviticus_root().join("symbols.json");

    if let Ok(raw) = std::fs::read_to_string(symbols_path) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&raw) {
            if let Some(arr) = json.get("symbols").and_then(|v| v.as_array()) {
                out.push_str("### Global Symbols\n\n");

                let root = leviticus_root();

                for s in arr {
                    let id = s.get("id").and_then(|v| v.as_str()).unwrap_or("unknown");
                    let path = s.get("path").and_then(|v| v.as_str()).unwrap_or("");

                    let full = if path.starts_with("~/.leviticus") {
                        root.join(path.trim_start_matches("~/.leviticus/"))
                    } else {
                        root.join(path)
                    };

                    out.push_str(&format!(
                        "- `{}` → [{}]({}) — {}\n",
                        id,
                        path,
                        file_link(&full),
                        s.get("doc").and_then(|v| v.as_str()).unwrap_or("")
                    ));
                }

                out.push_str("\n");
            }
        }
    }

    out
}
pub fn generate_explain_doc() -> std::io::Result<()> {
    let workspace = std::env::current_dir()?;
    state::save_workspace(&workspace);
    let out = workspace.join("explain.md");
    let path = explain_json_path();

    if !path.exists() {
        println!("missing explain.json at {:?}", path);
        return Ok(());
    }

    let raw = fs::read_to_string(&path)?;
    let json: serde_json::Value =
        serde_json::from_str(&raw).unwrap_or_else(|_| serde_json::json!({}));

    let md = render_explain(&json);

    let root = "/Users/me/dev/leviticus";

    format!("[Open Root]({})", file_link(&PathBuf::from(root)));

    println!("writing → {:?}", out);

    fs::write(out, md)?;

    Ok(())
}

pub fn generate_runtime_views() {
    println!("▶ generate_runtime_views CALLED");
    generate_explain_doc();
}

fn render_explain(json: &serde_json::Value) -> String {
    let mut out = String::new();

    out.push_str("# leviticus Explain\n\n");

    // -------------------------
    // SUMMARY
    // -------------------------
    if let Some(summary) = json.get("summary") {
        out.push_str("## Summary\n\n");
        out.push_str(summary.as_str().unwrap_or("N/A"));
        out.push_str("\n\n");
    }

    // -------------------------
    // MANIFEST SECTION
    // -------------------------
    if let Some(manifest) = json.get("manifest") {
        out.push_str("## Manifest (Root Config)\n\n");

        if let Some(obj) = manifest.as_object() {
            for (k, v) in obj {
                out.push_str(&format!("- **{}**: {}\n", k, v));
            }
        }

        out.push_str("\n");
    }

    // -------------------------
    // SYMBOLS SECTION
    // -------------------------
    if let Some(symbols) = json.get("symbols") {
        out.push_str("## Symbols (Navigation Roots)\n\n");

        if let Some(arr) = symbols.as_array() {
            for s in arr {
                let id = s.get("id").and_then(|v| v.as_str()).unwrap_or("unknown");
                let path = s.get("path").and_then(|v| v.as_str()).unwrap_or("");

                out.push_str(&format!("- `{}` → `{}`\n", id, path));
            }
        }

        out.push_str("\n");
    }

    // -------------------------
    // STATE SECTION
    // -------------------------
    if let Some(state) = json.get("state") {
        out.push_str("## Runtime State\n\n");

        if let Some(obj) = state.as_object() {
            for (k, v) in obj {
                out.push_str(&format!("- **{}**: {}\n", k, v));
            }
        }

        out.push_str("\n");
    }

    out.push_str("## Symbols (Clickable)\n\n");

    out.push_str(&render_symbols(json));
    out.push_str("\n");

    out
}

pub fn generate_explain_doc2() {
    todo!("generate_explain_doc2")
}
pub fn open_explain_doc() {
    todo!("open_explain_doc")
}
