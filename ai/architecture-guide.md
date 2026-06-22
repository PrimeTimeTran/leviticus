# Loid Project Context / Architecture Guide

## Project Overview

Loid is a system-level development environment layer designed to sit above existing languages, package managers, editors, and build systems.

The core idea:

> Do not replace Rust, TypeScript, Cargo, npm, Git, editors, etc.
> Add a semantic runtime layer that understands the workspace and exposes a better developer experience.

Loid acts as:

- daemon
- workspace analyzer
- filesystem intelligence layer
- view/projection system
- configuration explanation system
- future IDE integration layer

The goal is to make the actual state of a project discoverable without requiring developers to remember every tool's conventions.

---

# Core Philosophy

## Current Developer Pain

Modern development environments have many competing sources of truth:

Examples:

- Cargo.toml
- Cargo.lock
- workspace Cargo.toml
- package.json
- tsconfig.json
- bundler configs
- IDE settings
- environment variables
- hidden tool defaults

The problem:

Developers often ask:

- "Which config actually won?"
- "Why is this dependency version being used?"
- "Why is this file included/excluded?"
- "Where did this behavior come from?"

Loid should answer these questions automatically.

---

# The Big Idea

Create a generated runtime explanation of the workspace.

Instead of:

Developer writes configuration →
Tool interprets configuration →
Developer guesses result

Loid does:

Developer works normally →
Loid observes →
Loid generates reality →
Developer inspects reality

The generated files are the source of understanding.

---

# Important Concepts

## Genesis vs Revelation

Future concept.

Two layers of truth:

## Genesis

"What was intended?"

Examples:

- original project setup
- developer preferences
- ideal architecture
- profiles
- defaults

## Revelation

"What is actually happening?"

Examples:

- resolved dependencies
- active configs
- overrides
- conflicts
- runtime state

The difference matters because many systems only expose the intended configuration, not the final reality.

---

# Daemon

Current crate:

```

crates/loid

```

Purpose:

A background process responsible for:

- maintaining workspace state
- generating explanations
- tracking views
- serving IPC
- future editor communication

Current communication:

TCP localhost:

```

127.0.0.1:7788

```

Commands:

- start
- status

Future:

- view switching
- workspace analysis
- editor requests

---

# Current Rust Structure

```

src
├── cli
│   ├── command.rs
│   └── mod.rs
│
├── daemon
│   ├── bootstrap.rs
│   ├── config.rs
│   ├── document.rs
│   ├── initialize.rs
│   ├── resolver.rs
│   ├── run.rs
│   ├── status.rs
│   └── data
│       ├── explain.json
│       ├── manifest.json
│       └── symbols.json
│
├── index
├── ipc
├── state
├── storage
└── view

```

---

# ~/.loid

Global Loid state.

Current:

```

~/.loid/

config.toml
manifest.json
symbols.json
state.json

cache/
registry/
views/
logs/

```

Purpose:

Persistent daemon data.

---

# Initialization

First run:

```

loid start

```

creates:

- global directories
- runtime state
- manifests
- symbol registry
- documentation files

Initialization should only happen once.

Version migrations handle future upgrades.

---

# Manifest

Example:

```

~/.loid/manifest.json

```

Tracks:

- installed version
- schema version
- initialization timestamp

Purpose:

Allow migrations.

---

# Symbols

Example:

```

~/.loid/symbols.json

```

Tracks important paths and concepts.

Example:

```json
{
  "id": "loid.root",
  "path": "~/.loid",
  "doc": "Root directory of loid system"
}
```

The goal:

Generated markdown can link directly into the filesystem.

---

# Explain System

Current generated file:

```
workspace/explain.md
```

Generated from:

```
src/daemon/data/explain.json
```

Purpose:

Human-readable explanation of the current project.

Example sections:

- project info
- toolchain
- active view
- dependencies
- conflicts
- resolution graph
- symbols
- navigation links

This is intended to become the "manual that writes itself".

---

# Explain Philosophy

Tools should expose:

Not:

"Here is the config file"

But:

"Here is what happened"

Example:

Instead of:

```
Cargo.toml has serde 1.0
```

Show:

```
serde resolved from:
./crates/foo/Cargo.toml

overrides:
./Cargo.toml

winner:
crate scope

reason:
local precedence rule
```

---

# Views

Core future feature.

A view is a projection of the workspace.

Similar to:

- git branches
- IDE profiles
- database views

But without changing the underlying files.

Examples:

## rust-dev

Shows:

- source
- tests
- docs hidden

## docs-only

Shows:

- documentation
- comments
- architecture

## minimal

Shows:

- important files only

---

# Views are NOT copies

Important:

Do not duplicate projects.

The desired model:

```
real filesystem
        |
        |
     Loid view
        |
        |
    editor/UI
```

The filesystem remains canonical.

Views are interpretations.

---

# Git Branch Analogy

Similar:

- fork
- experiment
- merge

Different:

No checkout.

The user should be able to switch views without:

- clean working tree
- commits
- moving files

A view should preserve the workspace exactly.

---

# Future Editor

Goal:

Do not rewrite an editor.

Possible integrations:

- Zed fork
- plugin
- VSCode extension

The editor should mostly become:

A UI for Loid.

Features:

- permanent Loid panel
- view switching shortcut
- workspace explanation
- dependency graph
- config tracing
- command prompt

Similar to Xcode permanent inspectors.

---

# Why Not Replace Imports?

Loid should not remove language rules.

Example:

Rust:

```rust
crate::daemon::run()
```

is still valid.

The problem is not imports.

The problem is understanding.

Loid should provide:

- symbol discovery
- explanation
- automatic navigation
- generated context

Not hide reality.

---

# Global Symbols Idea

Future language support:

Loid can maintain a semantic index.

Potential behavior:

A developer can see:

"this symbol exists globally because..."

instead of:

"why isn't this imported?"

---

# Dependency Resolution Layer

Future:

Support:

- Cargo
- npm
- pnpm
- Dart
- etc.

Not by replacing them.

By observing them.

Example output:

```
Dependency Conflict:

swc

Found:
./Cargo.toml
./crates/evaluator/Cargo.toml

Resolution:
crate override wins

Reason:
workspace precedence rule
```

---

# Current MVP Direction

Do NOT build everything.

Next milestones:

## Phase 1

Daemon:

- starts
- stores state
- generates explain.md
- tracks workspace root

DONE / IN PROGRESS

---

## Phase 2

Resolver:

- detect configs
- detect languages
- detect dependencies
- explain choices

---

## Phase 3

Views:

- create view
- switch active view
- store metadata

---

## Phase 4

Editor integration:

- command palette
- panel
- tree projection
- shortcuts

---

# Design Rule

The daemon should know everything.

The editor should know nothing.

The editor asks:

"show me the active view"

"explain this symbol"

"why is this dependency here?"

Loid answers.

---

# Final Goal

A developer should be able to open a project and immediately understand:

- what exists
- why it exists
- what configuration produced it
- what overrides what
- how to change it

without learning every tool's hidden rules.
