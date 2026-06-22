# Leviticus — Concept Overview

The following is a 10,000 overview of the vision of Leviticus codebase.

| Concept                     | Problem It Solves                                              | Core Idea                                                   | Similar Existing Concepts                  | Leviticus Difference                       |
| --------------------------- | -------------------------------------------------------------- | ----------------------------------------------------------- | ------------------------------------------ | ------------------------------------------ |
| **Daemon**                  | Tools are fragmented and only run when invoked                 | A persistent process understands the workspace continuously | Language servers, background build tools   | One universal workspace intelligence layer |
| **Workspace Reality**       | Config files often describe intent, not what actually happened | Generate a canonical explanation of the resolved state      | Lock files, build output, IDE indexes      | A human-readable reality layer             |
| **Genesis**                 | Developers lose the original intent behind a project           | Store "how this project was meant to start"                 | Templates, scaffolding configs             | Historical intent model                    |
| **Revelation**              | Developers don't know what finally won                         | Store "what the system became"                              | Lock files, resolved dependency graphs     | Runtime truth model                        |
| **Explain Files**           | Developers waste time discovering why things work              | Automatically generate docs describing the system           | README, generated docs, IDE metadata       | Documentation generated from reality       |
| **Symbol Registry**         | Jumping around projects requires manual knowledge              | Maintain a map of important files/symbols                   | Language server indexes                    | Universal cross-tool navigation            |
| **Generated Markdown Docs** | Developers don't know where configs live                       | Create clickable explanations into the filesystem           | IDE navigation, docs                       | The filesystem explains itself             |
| **Views**                   | One filesystem cannot represent every workflow                 | Multiple interpretations of the same project                | Git branches, IDE profiles, database views | No checkout, no duplication                |
| **View Switching**          | Changing workflows requires changing state                     | Swap perspective, not files                                 | Xcode schemes, VSCode profiles             | Workspace remains untouched                |
| **Projection Layer**        | Raw files are not always the best representation               | Show a filtered/organized workspace                         | Virtual filesystems, database views        | Same files, different reality              |
| **VFS Layer**               | Applications expect normal files                               | Provide filesystem abstraction                              | OS filesystems, containers                 | Enables views without moving files         |
| **Resolver**                | Multiple configs compete                                       | Understand precedence and explain decisions                 | Package managers                           | Cross-language explanation engine          |
| **Conflict Detection**      | Developers guess which config wins                             | Show conflicts and resolution rules                         | Compiler warnings                          | Explain "why"                              |
| **Dependency Graph**        | Dependencies become invisible complexity                       | Map dependency sources and relationships                    | Cargo tree, npm graph                      | Unified across ecosystems                  |
| **Configuration Trace**     | "Where did this setting come from?"                            | Track origin of every behavior                              | CSS cascade, env variables                 | Universal provenance tracking              |
| **Profiles**                | Developers have different modes of work                        | Save environment perspectives                               | Cargo profiles, IDE configs                | Applies to entire workspace reality        |
| **Global Definitions**      | Repeated imports/config create friction                        | Allow workspace-level semantic availability                 | Rust prelude, globals                      | Managed + explainable globals              |
| **Implicit Context**        | Humans think in projects, not files                            | Make workspace knowledge available                          | IDE indexing                               | Context layer above languages              |
| **Explicit Truth Layer**    | Hidden magic becomes dangerous                                 | Preserve actual source paths and ownership                  | Rust modules                               | Convenience without losing traceability    |
| **Editor Integration**      | Users need interaction                                         | Provide UI for the daemon                                   | VSCode, Zed, Xcode                         | Editor becomes a Leviticus client          |
| **Permanent Panel**         | Important context disappears                                   | Always-available workspace control surface                  | Xcode inspectors                           | First-class workspace control              |
| **Command System**          | Too many tools have different commands                         | One command surface                                         | CLI wrappers                               | Unified developer interface                |
| **IPC**                     | Editor and daemon need communication                           | Standard communication channel                              | Language servers                           | General workspace protocol                 |
| **HMR / Live Updates**      | State changes constantly                                       | Update views without restarting                             | Dev servers                                | Workspace-level live state                 |
| **Language Agnostic Layer** | Every language has different tooling                           | Understand common concepts above languages                  | LSP                                        | Build once, support many                   |
| **No Tool Replacement**     | Rewriting ecosystems is impossible                             | Sit above existing tools                                    | IDEs, shells                               | Adds intelligence without replacing        |
| **File Ownership Model**    | People don't know what controls something                      | Track origin and authority                                  | Git history                                | "Who owns this behavior?"                  |
| **Resolution Rules**        | Multiple sources create confusion                              | Define precedence explicitly                                | CSS cascade                                | Universal configuration hierarchy          |
| **Reality Lock File**       | Lock files are too noisy                                       | Show important resolved state                               | Cargo.lock                                 | Human-focused lock explanation             |
| **Learning Layer**          | Developers forget tool behavior                                | Record discoveries automatically                            | Documentation                              | The environment teaches itself             |
| **IDE as Consumer**         | Editors currently duplicate intelligence                       | Move intelligence into daemon                               | LSP model                                  | Any editor can connect                     |

---

# The Core Mental Model

```
              Developer
                  |
                  v

              Editor/UI
                  |
                  v

              Leviticus Daemon
                  |
      --------------------------
      |            |           |
      v            v           v

 Filesystem    Toolchains   Configs

 Rust          Cargo        Cargo.toml
 TS            npm          package.json
 Dart          pub          pubspec.yaml
```

The daemon becomes the "brain".

The editor becomes the "face".

---

# Current Systems vs Leviticus

| Existing Approach | Problem                     | Leviticus Approach              |
| ----------------- | --------------------------- | ------------------------------- |
| package.json      | User writes intent manually | Generate resolved understanding |
| Cargo.toml        | Configuration source only   | Configuration + explanation     |
| Cargo.lock        | Complete but noisy          | Human-readable resolution       |
| Git branches      | Require switching state     | Switch perspectives             |
| IDE settings      | Editor-specific             | Workspace-level                 |
| Language servers  | Language-specific           | Cross-language                  |
| README            | Gets stale                  | Generated from reality          |

---

# The One Sentence Version

**Leviticus is a persistent workspace intelligence layer that observes the real state of a project, explains why it is that way, and lets users create different views of the same reality without changing the underlying system.**

## Get more details in `./ai/*`

- ./ai/fs-concepts-structure.md
- ./ai/architecture-guide.md
