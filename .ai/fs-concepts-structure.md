# Loid Import Files/Directories

- **Global Loid state** = "who Loid is + installed capabilities + reusable knowledge"
- **Workspace Loid state** = "what this project is + what Loid discovered"
- **Generated explain/docs** = "human navigation layer"
- **Deep analysis artifacts** = "zoom into any specific area"

---

# 1. Global System (`~/.loid`)

Purpose:

> Everything Loid needs to exist independently of any project.

| Path                         | Type           | Purpose                        | Example Content                          |
| ---------------------------- | -------------- | ------------------------------ | ---------------------------------------- |
| `~/.loid/config.toml`        | Config         | Global daemon/user preferences | logging level, enabled features, ports   |
| `~/.loid/manifest.json`      | State          | Loid installation metadata     | version, schema version, migrations      |
| `~/.loid/symbols.json`       | Registry       | Global clickable symbols       | `loid.root`, `loid.config`, daemon paths |
| `~/.loid/state.json`         | Runtime State  | Daemon lifecycle               | starts, uptime, last run                 |
| `~/.loid/registry/`          | Database       | Global indexes                 | known projects, symbols, metadata        |
| `~/.loid/registry/graph.db`  | Graph DB       | Relationship storage           | projects → files → symbols → tools       |
| `~/.loid/cache/`             | Cache          | Expensive computations         | parsed ASTs, indexes                     |
| `~/.loid/cache/indexes/`     | Cache          | Search/navigation indexes      | symbol lookup                            |
| `~/.loid/cache/projections/` | Cache          | View materializations          | generated workspace projections          |
| `~/.loid/logs/`              | Logs           | Daemon history                 | runtime/debug logs                       |
| `~/.loid/views/`             | Profiles       | Global view templates          | rust-dev, docs-only, minimal             |
| `~/.loid/docs/`              | Generated Docs | Loid itself documentation      | daemon commands, concepts                |
| `~/.loid/socket`             | IPC            | Communication endpoint         | editor ↔ daemon                          |
| `~/.loid/daemon.pid`         | Runtime        | Running process tracking       | PID                                      |

---

# 2. Workspace / Project (`./.loid`)

Purpose:

> Everything Loid knows about THIS repository.

Created when:

```bash
loid start
```

runs inside a project.

Example:

```
project/
├── src/
├── Cargo.toml
└── .loid/
```

---

| Path                          | Type             | Purpose                   | Example                 |
| ----------------------------- | ---------------- | ------------------------- | ----------------------- |
| `./.loid/config.toml`         | Workspace Config | Project-specific settings | active view, rules      |
| `./.loid/manifest.json`       | Snapshot         | Project identity          | root, language, tools   |
| `./.loid/explain.md`          | Human Overview   | Main entry point          | "what is this project?" |
| `./.loid/explain.json`        | Machine Overview | Structured explanation    | data source for docs    |
| `./.loid/symbols.json`        | Registry         | Project symbols           | files, modules, configs |
| `./.loid/state.json`          | Runtime          | Current workspace state   | active view, last scan  |
| `./.loid/cache/`              | Cache            | Project analysis cache    | parsed results          |
| `./.loid/cache/files/`        | Cache            | File metadata             | hashes, timestamps      |
| `./.loid/cache/ast/`          | Cache            | Language parsing          | Rust/TS trees           |
| `./.loid/cache/index/`        | Search           | Fast navigation           | symbol lookup           |
| `./.loid/graphs/`             | Analysis         | Relationships             | dependency graph        |
| `./.loid/graphs/modules.json` | Graph            | Module relationships      | imports                 |
| `./.loid/graphs/deps.json`    | Graph            | Dependency resolution     | cargo/npm/etc           |
| `./.loid/conflicts.json`      | Analysis         | Problems detected         | duplicate deps          |
| `./.loid/resolution.json`     | Analysis         | Final decisions           | why config won          |
| `./.loid/views/`              | Profiles         | Workspace views           | current projections     |

---

# 3. The "Overview" Layer

The file users should start with:

```
./.loid/explain.md
```

Purpose:

> The table of contents for reality.

Example:

```md
# Project Explanation

Project:
loi

Root:
/Users/me/dev/loi

Languages:
Rust
TypeScript

Active View:
rust-dev

Dependencies:
342 resolved

Conflicts:
2 warnings

More:

[Dependency Graph]
[Module Graph]
[Config Resolution]
[View Rules]
[Daemon Logs]
```

The user should not need to know:

- cargo commands
- npm commands
- compiler flags
- hidden configs

They start here.

---

# 4. Deep Dive / Zoom Files

The explain file points to deeper files.

Example:

```
.explain.md
      |
      |
      +---> graphs/deps.md
      |
      +---> graphs/modules.md
      |
      +---> analysis/cargo-resolution.md
      |
      +---> analysis/import-map.md
      |
      +---> analysis/config-trace.md
```

---

# Detailed Generated Files

| File                       | Purpose                               |
| -------------------------- | ------------------------------------- |
| `dependency-resolution.md` | Why each dependency version exists    |
| `config-trace.md`          | Where every config value came from    |
| `module-graph.md`          | Import/module relationships           |
| `symbol-index.md`          | All symbols + locations               |
| `filesystem-map.md`        | Files grouped by purpose              |
| `view-analysis.md`         | How current view transforms workspace |
| `conflicts.md`             | Problems + explanations               |
| `toolchain.md`             | Versions + detected environments      |

---

# 5. Future "Reality Lock"

Something like:

```
.loid/reality.json
```

Not a replacement for:

- Cargo.lock
- package-lock
- pnpm-lock

Instead:

> A human-readable summary of all lock files.

Example:

```json
{
  "rust": {
    "cargo_lock": "healthy",
    "resolved": 421
  },

  "typescript": {
    "package_lock": "healthy",
    "resolved": 892
  },

  "conflicts": [
    {
      "package": "swc",
      "reason": "workspace override"
    }
  ]
}
```

---

# 6. Long-Term Architecture

```
                 USER

                  |
                  v

             explain.md
          (human overview)

                  |
                  v

            explain.json
          (machine model)

                  |
     --------------------------------
     |              |               |

 dependencies    symbols        views

     |              |               |

 cargo/npm      files/code     projections

```

---

The important design choice:

**Generated files are not configuration.**

Users do not edit:

- `explain.md`
- `symbols.json`
- analysis files

They are the **observed reality of the workspace**.

The user edits intent:

- configs
- code
- views

Loid continuously generates the explanation layer.
