# Project Agent Rules

include:

- .ai/core/agents.md
- .ai/crates/vfs.md

tools:

- fs.read
- git.status

scope:

- crates/vfs
- crates/compiler

exclude:

- target/
- node_modules/
