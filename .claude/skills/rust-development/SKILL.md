---
name: rust-development-philosophy
description: "REQUIRED before writing or planning ANY rust code. Contains crucial project standards and guidelines"
---

# Rust Development Standards

## Clippy Configuration Requirements

When working on any Rust project, you MUST ensure maximum clippy strictness is configured. If the project's `Cargo.toml` does not have these settings, add them before writing any code.

### Required Cargo.toml Lints Section

```toml
[lints.clippy]
# Enable all lint groups at deny level
all = { level = "deny", priority = -1 }
pedantic = { level = "deny", priority = -1 }
nursery = { level = "deny", priority = -1 }
restriction = { level = "deny", priority = -1 }
cargo = { level = "deny", priority = -1 }

# Necessary exceptions for impractical or conflicting restriction lints
blanket_clippy_restriction_lints = "allow"
missing_docs_in_private_items = "allow"
implicit_return = "allow"
question_mark_used = "allow"
separated_literal_suffix = "allow"
mod_module_files = "allow"
single_call_fn = "allow"
pub_use = "allow"
multiple_crate_versions = "allow"
pub_with_shorthand = "allow"
single_char_lifetime_names = "allow"

[lints.rust]
unsafe_code = "deny"
missing_docs = "deny"
rust_2018_idioms = { level = "deny", priority = -1 }
```

### Why These Settings

| Lint Group | Purpose |
|------------|---------|
| `all` | Catches common mistakes and bad patterns |
| `pedantic` | Enforces stricter style and correctness |
| `nursery` | Experimental lints that catch subtle issues |
| `restriction` | Maximum strictness (selective allows required) |
| `cargo` | Ensures proper Cargo.toml hygiene |

### Allowed Restriction Lints Rationale

| Lint | Reason for Allowing |
|------|---------------------|
| `blanket_clippy_restriction_lints` | Meta-lint about restriction group usage |
| `implicit_return` | Conflicts with `needless_return`; implicit returns are idiomatic |
| `question_mark_used` | The `?` operator is idiomatic Rust |
| `separated_literal_suffix` | Conflicts with `unseparated_literal_suffix` |
| `mod_module_files` | Conflicts with `self_named_module_files` |
| `single_call_fn` | Impractical; single-use functions improve readability |
| `pub_use` | Re-exports are a valid API design pattern |
| `multiple_crate_versions` | Often unavoidable with transitive dependencies |
| `missing_docs_in_private_items` | Private items don't require documentation |
| `pub_with_shorthand` | Style preference, `pub(in crate)` vs `pub(crate)` |
| `single_char_lifetime_names` | `'a` is idiomatic for simple lifetimes |

## Clippy Usage

Run clippy frequently during development:

```bash
cargo clippy -- -D warnings
```

Run after:
- Adding new code
- Refactoring existing code
- Before considering any task complete

Fix all clippy warnings before proceeding. If you cannot fix a warning without supressing it stop and consult the developer. NEVER surpress a warning without consultation from the human

## Rust CLI Project Layout Overview

**Project Root**
- `Cargo.toml` - Project manifest with dependencies, metadata, and build configuration
- `Cargo.lock` - Locked dependency versions (should be committed for binaries)
- `flake.nix` - Nix flake for reproducible builds and dev environment

**Source Directory (`src/`)**

- `main.rs` - Minimal entry point, calls into library code
- `lib.rs` - Application orchestration, CLI argument definitions, public API, mostly re-pubs or uses. Logic should be broken out into modules
- `config.rs` - Configuration file parsing and environment variable handling
- ... - Break out logic int

**Test Directory (`tests/`)**

- Integration tests that exercise the CLI as a whole only
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

## Rust Unit Testing

### Basic Test Structure

Tests live inside the module they test using a `#[cfg(test)]` block:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_returns_sum() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn add_handles_negatives() {
        assert_eq!(add(-1, 1), 0);
    }
}
```

### Common Assertions

| Macro | Purpose |
|-------|---------|
| `assert!(expr)` | Passes if expression is true |
| `assert_eq!(a, b)` | Passes if values are equal |
| `assert_ne!(a, b)` | Passes if values are not equal |

### Testing Errors

```rust
#[test]
fn parse_rejects_invalid_input() {
    let result = parse("");
    assert!(result.is_err());
}

#[test]
fn parse_returns_expected_error() {
    let result = parse("");
    assert!(matches!(result, Err(ParseError::EmptyInput)));
}
```

### Expected Panics

```rust
#[test]
#[should_panic(expected = "index out of bounds")]
fn panics_on_invalid_index() {
    let v = vec![1, 2, 3];
    let _ = v[10];
}
```

### Test Organization

``` example
src/
  lib.rs          # pub API + unit tests
  config.rs       # config logic + unit tests
  commands/
    mod.rs        # command logic + unit tests
tests/
  integration.rs  # full CLI tests (separate crate)
```

### Running Tests

```bash
cargo test                    # all tests
cargo test --lib              # unit tests only
cargo test test_name          # specific test
cargo test -- --nocapture     # show println! output
```

### Integration Tests

Files in `tests/` compile as separate crates and test the public API:

```rust
// tests/cli_test.rs
use myapp::run;

#[test]
fn cli_accepts_valid_input() {
    let result = run(&["--input", "test.txt"]);
    assert!(result.is_ok());
}
```

### Tips

- Keep unit tests next to the code they test
- Integration tests go in `tests/` directory
- Use `#[ignore]` for slow tests, run with `cargo test -- --ignored`
- Tests run in parallel by default
