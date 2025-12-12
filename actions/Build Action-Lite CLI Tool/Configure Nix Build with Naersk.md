#action #published #action-lite #priority

# Notes

This sub-action focuses on setting up the Nix build infrastructure using naersk for the action-lite CLI tool. Naersk is the recommended approach for building Rust projects with Nix because it leverages Cargo's native build system rather than reimplementing it, resulting in more maintainable and reliable builds.

**Why naersk:**
- Integrates directly with Cargo.toml and Cargo.lock
- Handles Rust dependency resolution automatically
- Provides incremental builds in Nix
- Well-maintained and widely used in the Nix+Rust ecosystem
- Simpler than alternatives like buildRustPackage for most use cases

**Technical Context:**
- User is on NixOS (Linux 6.12.60)
- Project is a new Rust CLI application
- Target platform: Linux only (simplifies configuration)
- Need development environment with rust-analyzer, cargo, etc.
- Flake-based Nix configuration (modern approach)

**Parent Action:**
This is a sub-action of "Build Action-Lite CLI Tool" which is in the discovery phase. The parent action specifies that Nix must be used for build and packaging configuration (specification #7).

# Statement of Action

**What:** Configure a Nix flake using naersk to build the action-lite Rust CLI tool and provide a development environment.

**Why:** Enable reproducible builds and development on NixOS. Using naersk ensures the Rust build process integrates seamlessly with Nix while maintaining compatibility with standard Cargo workflows. This is essential infrastructure that must be in place before any Rust code can be developed or tested on the target platform.

# Statement of Inputs

This action depends on:
- naersk (Nix library for building Rust projects)
- nixpkgs (specifically nixpkgs-unstable for recent Rust toolchain)
- Rust toolchain (cargo, rustc, rust-analyzer)
- Understanding of Nix flakes syntax and structure
- Parent action specifications (requirements #6, #7, #8 from parent)

External dependencies from nixpkgs:
- naersk.lib
- rustc
- cargo
- rust-analyzer
- clippy
- rustfmt

# Statement of Specifications

**Flake Configuration Requirements:**

1. **Flake Structure**
   - Must use Nix flakes format (flake.nix at project root)
   - Must declare naersk as an input from GitHub
   - Must declare nixpkgs as an input (preferably nixpkgs-unstable for recent Rust)
   - Must support Linux x86_64 system only (per parent spec #8)
   - Must use flake-utils or manual system handling for platform specification

2. **Build Derivation**
   - Must use naersk.lib.${system}.buildPackage function
   - Must point to project root as source (src = ./.)
   - Must respect Cargo.toml metadata (name, version, etc.)
   - Must produce a working executable in the Nix store
   - Must be accessible via `nix build` command

3. **Development Shell**
   - Must provide devShell with all necessary development tools
   - Required tools: cargo, rustc, rust-analyzer, clippy, rustfmt
   - Should enable immediate `cargo build` and `cargo test` after entering shell
   - Must support LSP functionality (rust-analyzer) for IDE integration
   - Should set any necessary environment variables for Rust development

4. **Cargo.toml Initialization**
   - Must create initial Cargo.toml for the action-lite CLI project
   - Package name: "action-lite" (or as specified)
   - Version: "0.1.0" (initial development version)
   - Edition: "2021" (current stable Rust edition)
   - Must include bin target configuration for CLI
   - Should include basic metadata (authors, description, license TBD)

5. **Project Structure Integration**
   - flake.nix must be at repository root
   - Cargo.toml must be at repository root
   - src/ directory must be created for Rust source files
   - Must not conflict with existing actions/ and .claude/ directories
   - .gitignore must include Rust-specific entries (target/, Cargo.lock considerations)

6. **Verification Requirements**
   - `nix flake check` must pass without errors
   - `nix develop` must successfully enter development shell
   - `cargo --version` must work within dev shell
   - `rust-analyzer --version` must work within dev shell
   - `nix build` must successfully build the package (even if minimal)
   - Resulting binary must be executable on Linux

**Non-Requirements (Out of Scope):**

7. No cross-compilation support needed (Linux only per parent spec)
8. No Docker integration required
9. No CI/CD configuration in this sub-action (separate concern)
10. No Nix overlays or custom Rust toolchain versions
11. No incremental build caching optimization (naersk default is sufficient)

**Success Criteria:**

12. Developer can run `nix develop` and immediately begin Rust development
13. All standard cargo commands work within the development shell
14. `nix build` produces a working executable
15. Configuration is maintainable and follows naersk best practices
16. Documentation comments in flake.nix explain key configuration decisions

# Statement of Design

This design provides a complete naersk-based Nix flake configuration for building the action-lite Rust CLI tool.

## Design Overview

The configuration consists of three main components:
1. **flake.nix** - Nix flake with naersk integration for builds and development shell
2. **Cargo.toml** - Rust project manifest
3. **src/main.rs** - Minimal Rust binary entry point

The design uses naersk's `buildPackage` function which reads Cargo.toml and Cargo.lock to handle all Rust-specific build logic, providing a clean integration between Nix and Cargo.

## File: flake.nix

Location: `/home/lucas/.core/projects/action-lite/flake.nix`

```nix
{
  description = "action-lite CLI - Command-line tool for managing action-lite methodology workflows";

  inputs = {
    # Use nixpkgs-unstable for recent Rust toolchain and packages
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    # naersk - Nix build system for Rust projects using Cargo
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # flake-utils for ergonomic multi-system support
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, naersk, flake-utils }:
    # Only support x86_64-linux as per specification #8
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        pkgs = import nixpkgs { inherit system; };

        # Initialize naersk for this system
        naersk-lib = pkgs.callPackage naersk { };

      in {
        # Default package: the action-lite CLI tool
        packages.default = naersk-lib.buildPackage {
          # Build from project root
          src = ./.;

          # naersk will read package name and metadata from Cargo.toml
          # No need to specify pname and version explicitly

          # Build only for Linux (no cross-compilation)
          # naersk handles this automatically via Cargo
        };

        # Development shell with Rust toolchain and tools
        devShells.default = pkgs.mkShell {
          # Development dependencies
          buildInputs = with pkgs; [
            # Rust compiler and package manager
            rustc
            cargo

            # Development tools
            rust-analyzer  # LSP server for IDE integration
            clippy         # Rust linter
            rustfmt        # Code formatter

            # Useful for debugging Nix issues
            nix-tree
          ];

          # Environment setup for development
          shellHook = ''
            echo "action-lite development environment"
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo ""
            echo "Available commands:"
            echo "  cargo build    - Build the project"
            echo "  cargo test     - Run tests"
            echo "  cargo run      - Run the CLI"
            echo "  nix build      - Build with Nix"
          '';
        };

        # Make the package available via 'nix run'
        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/action-lite";
        };
      }
    );
}
```

**Design Rationale:**
- Uses `nixpkgs-unstable` for recent Rust toolchain
- `flake-utils.lib.eachSystem` provides clean system handling, restricted to `x86_64-linux`
- naersk's `buildPackage` is the core build function - it reads Cargo.toml/Cargo.lock automatically
- Development shell includes all tools needed for Rust development
- `shellHook` provides helpful information when entering the dev shell
- `apps.default` enables `nix run` to execute the built CLI

## File: Cargo.toml

Location: `/home/lucas/.core/projects/action-lite/Cargo.toml`

```toml
[package]
name = "action-lite"
version = "0.1.0"
edition = "2021"
authors = ["action-lite contributors"]
description = "Command-line tool for visualizing and managing action-lite methodology workflows"
license = "MIT OR Apache-2.0"  # Standard Rust dual-license
repository = "https://github.com/Givlucas/action-lite"  # Adjust if different

# Binary target configuration
[[bin]]
name = "action-lite"
path = "src/main.rs"

[dependencies]
# Dependencies will be added as implementation progresses
# Initially empty - just enough to build a minimal binary

[dev-dependencies]
# Test dependencies will be added later
```

**Design Rationale:**
- Uses Rust 2021 edition (current stable)
- Standard dual-license (MIT OR Apache-2.0) common in Rust ecosystem
- Binary target explicitly configured to point to main.rs
- Empty dependencies initially - will be populated during implementation
- Package metadata provides context for the tool

## File: src/main.rs

Location: `/home/lucas/.core/projects/action-lite/src/main.rs`

```rust
//! action-lite CLI tool
//!
//! Command-line interface for visualizing and managing action-lite
//! methodology workflows. This tool helps developers understand
//! dependency graphs and prioritize work.

fn main() {
    println!("action-lite CLI v{}", env!("CARGO_PKG_VERSION"));
    println!("Rust + Nix build system configured successfully!");
}
```

**Design Rationale:**
- Minimal but valid Rust binary that can compile and run
- Uses `env!("CARGO_PKG_VERSION")` to pull version from Cargo.toml
- Provides immediate feedback that the build system works
- Doc comments establish documentation structure
- Will be expanded during implementation phase

## File: .gitignore updates

Location: `/home/lucas/.core/projects/action-lite/.gitignore`

Add Rust-specific entries:

```gitignore
# Rust build artifacts
/target/
**/*.rs.bk
Cargo.lock  # Lock file is typically gitignored for binaries

# Nix build artifacts
/result
/result-*
```

**Design Rationale:**
- `/target/` is where Cargo places build artifacts
- `Cargo.lock` should be included for libraries but gitignored for binaries (CLI tool is a binary)
- `/result` is the symlink created by `nix build`

## Project Directory Structure

After implementation, the project will have this structure:

```
/home/lucas/.core/projects/action-lite/
├── .git/
├── .gitignore (updated)
├── .claude/
├── actions/
│   └── Build Action-Lite CLI Tool/
│       └── Configure Nix Build with Naersk.md (this file)
├── flake.nix (new)
├── Cargo.toml (new)
└── src/
    └── main.rs (new)
```

## Verification Steps

After implementation, verify with these commands:

1. **Check flake validity:**
   ```bash
   nix flake check
   ```

2. **Enter development shell:**
   ```bash
   nix develop
   ```

3. **Verify Rust tools within shell:**
   ```bash
   cargo --version
   rustc --version
   rust-analyzer --version
   ```

4. **Build with Cargo (within dev shell):**
   ```bash
   cargo build
   ./target/debug/action-lite
   ```

5. **Build with Nix:**
   ```bash
   nix build
   ./result/bin/action-lite
   ```

6. **Run directly with Nix:**
   ```bash
   nix run
   ```

All commands should succeed and the binary should print its version message.

## Implementation Notes

- The flake uses `follows` for naersk's nixpkgs input to ensure consistency
- No custom Rust toolchain configuration - uses nixpkgs default (recent from unstable)
- naersk automatically generates Cargo.lock if it doesn't exist
- The design assumes the project will use standard Cargo dependency management
- Future implementation phases will add actual CLI functionality and dependencies
