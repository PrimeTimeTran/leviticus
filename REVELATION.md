### src/cli/command.rs

```rs
        // ENUMS:
        enum Command { Start, Status, Explain, ExplainDoc, View(name: String), ViewFork(name: String), Deps, Stop, Reload, ViewList }

        // FUNCTIONS:
        fn parse() -> Cli

        // STRUCTS:
        struct Cli
            // PROPERTIES:
            verbose: bool, command: Command
```

### src/daemon/bootstrap.rs

```rs
        // FUNCTIONS:
        fn bootstrap(
            path_workspace: PathBuf
        )
        fn ensure_initialized() -> Result<()>
        fn manifest_path() -> PathBuf
        fn run_migrations(
            from: u32
        ) -> Result<()>
        fn run_version_migrations(
            from: &str
        ) -> Result<()>
        fn symbols_path() -> PathBuf
        fn write_manifest() -> Result<()>
        fn write_symbols() -> Result<()>

        // STRUCTS:
        struct Manifest
            // PROPERTIES:
            installed_version: String, schema_version: u32, initialized_at: i64
```

### src/daemon/config.rs

```rs
        // FUNCTIONS:
        fn derive_runtime_context()
        fn read_cargo_toml() -> CargoToml
        fn sync_project_metadata()
        fn write_cargo_toml(
            cfg: &CargoToml
        )

        // STRUCTS:
        struct CargoToml
            // PROPERTIES:
            package: Package
        struct Package
            // PROPERTIES:
            name: String, version: String, edition: String
```

### src/daemon/initialize.rs

```rs
        // FUNCTIONS:
        fn init() -> Result<()>
```

### src/daemon/reload.rs

```rs
        // FUNCTIONS:
        fn reload()
```

### src/daemon/resolver.rs

```rs
        // FUNCTIONS:
        fn loid_dir() -> PathBuf
        fn loid_root() -> std::path::PathBuf
        fn project_root() -> std::path::PathBuf
```

### src/daemon/run.rs

```rs
        // FUNCTIONS:
        fn init_runtime_state()
        fn serve(
            listener: TcpListener
        )
        fn start()
```

### src/daemon/status.rs

```rs
        // FUNCTIONS:
        fn status()
```

### src/daemon/stop.rs

```rs
        // FUNCTIONS:
        fn stop()
```

## src/main.rs

```rs
    // FUNCTIONS:
    fn main()
```

### src/projection/document.rs

```rs
        // FUNCTIONS:
        fn data_dir() -> PathBuf
        fn explain_json_path() -> PathBuf
        fn file_link(
            path: &Path
        ) -> String
        fn generate_explain_doc() -> std::io::Result<()>
        fn generate_explain_doc2()
        fn generate_runtime_views()
        fn manifest_json_path() -> PathBuf
        fn open_explain_doc()
        fn read_symbols() -> serde_json::Value
        fn render_explain(
            json: &serde_json::Value
        ) -> String
        fn render_symbols(
            json: &serde_json::Value
        ) -> String
        fn symbol_to_md(
            symbol: &serde_json::Value
        ) -> String
        fn symbols_json_path() -> PathBuf
```

#### src/projection/view/mod.rs

```rs
            // FUNCTIONS:
            fn fork(
                name: String
            )
            fn list()
            fn set_active(
                name: String
            )
```

### src/state/state.rs

```rs
        // FUNCTIONS:
        fn load() -> DaemonState
        fn now() -> u64
        fn path() -> PathBuf
        fn save(
            state: &DaemonState
        )
        fn save_workspace(
            path: &PathBuf
        )

        // STRUCTS:
        struct DaemonState
            // PROPERTIES:
            starts: u64, started_at: u64, longest_run: u64
```

# EMPTY FILES

.gitignore
AGY.md
Cargo.toml
ai/architecture-guide.md
ai/fs-concepts-structure.md
explain.md
feedback.md
src/LONGEST.md
src/cli/mod.rs
src/core/mod.rs
src/daemon/data/explain.json
src/daemon/data/manifest.json
src/daemon/data/symbols.json
src/daemon/mod.rs
src/index/mod.rs
src/ipc/mod.rs
src/lib.rs
src/projection/mod.rs
src/registry.db
src/state/mod.rs
src/storage/mod.rs
