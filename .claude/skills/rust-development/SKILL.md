---
name: rust-development-philosophy
description: "REQUIRED before writing or planning ANY rust code. Contains crucial project standards and guidelines"
---

## Rust CLI Project Layout Overview

**Project Root**
- `Cargo.toml` - Project manifest with dependencies, metadata, and build configuration
- `Cargo.lock` - Locked dependency versions (should be committed for binaries)
- `flake.nix` - Nix flake for reproducible builds and dev environment

**Source Directory (`src/`)**

- `main.rs` - Minimal entry point, calls into library code
- `lib.rs` - Application orchestration, CLI argument definitions, public API
- `config.rs` - Configuration file parsing and environment variable handling

**Test Directory (`tests/`)**

- Integration tests that exercise the CLI as a whole
- Each file compiles as a separate test crate
- Tests the public API exposed by lib.rs

**Examples Directory (`examples/`)**

- Runnable examples demonstrating usage
- Useful for documentation and testing common workflows

**Design Principles**

- main.rs stays thin, all logic lives in the library
- Commands are isolated modules with a shared trait or enum
- Un-handlable errors propagate up with context, handled at the top level
- Configuration is loaded once and passed down, not accessed globally
- Logic is broken out into seperate modules.
