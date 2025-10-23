# Action Lite

A file-based task tracking system using acyclic directed meta graphs, built in Rust.

## Overview

Action Lite is a CLI tool that helps you manage tasks and their dependencies using markdown files. It provides a simple yet powerful way to track project progress, visualize task dependencies, and maintain project documentation.

## Features

- **File-based storage**: All actions are stored as markdown files for easy version control
- **Dependency tracking**: Link actions together to create dependency graphs
- **Status management**: Track actions through different lifecycle states
- **Priority marking**: Mark critical actions as priority
- **Visual graphing**: Display action dependencies in a terminal-friendly tree view
- **Colored output**: Easy-to-read colored terminal output
- **Reference updating**: Move actions and automatically update all references

## Installation

### Using Nix (Recommended)

If you have Nix with flakes enabled:

```bash
# Development environment
nix develop

# Build and run
nix build
./result/bin/action-lite --help

# Run directly
nix run . -- --help
```

### Using Cargo

```bash
# Install dependencies and build
cargo build --release

# Run
./target/release/action-lite --help
```

## Action File Format

Each action is a markdown file with specific tags and sections:

### Tags

- `#action` - Required tag to identify the file as an action
- `#<status>` - One of: discovery, design, implementation, test, document, publish, published
- `#<project>` - Project name for grouping related actions
- `#priority` - Optional tag to mark as high priority

### Sections

1. **Notes** - General notes about the task
2. **Statement of Action** - Detailed description of what needs to be done
3. **Statement of inputs** - Bullet list of links to dependent actions (also accepts "Statement of specifications")
   - Supports wiki-links: `[[Action Name]]` or `[[Action Name.md]]` (Obsidian-compatible)
   - Also supports markdown links: `[Action Name](file.md)` or `[Action Name](file)`
   - Case-insensitive section header
4. **Statement of Design** - Design details for each output
   - Output - What will be produced
   - Design - How to implement
5. **Analysis of Impact** - Impact assessment

### Example Action

```markdown
# Implement Authentication

#action #implementation #auth-system #priority

# Notes

Critical security feature required before launch.

# Statement of Action

Implement secure user authentication with login/logout functionality.

# Statement of inputs

- [[Design Database Schema]]
- [[Setup API Framework]]

# Statement of Design

## Output

Functional authentication system with JWT tokens.

### Design

1. Implement password hashing with bcrypt
2. Create JWT token generation
3. Build login/logout endpoints

# Analysis of Impact

Enables all user-specific features and secures the application.
```

## CLI Commands

### List All Actions

Display all actions with their status and path:

```bash
action-lite list
```

### List Priority Actions

Show only actions marked with `#priority`:

```bash
action-lite priority
```

### List Actions by Status

Filter actions by their current status:

```bash
action-lite status discovery
action-lite status implementation
action-lite status published
```

Valid statuses: `discovery`, `design`, `implementation`, `test`, `document`, `publish`, `published`

### Create New Action

Create a new action with a template:

```bash
action-lite new "implement-feature" --project my-project
```

Notes:
- Action names cannot contain path separators (`/` or `\`)
- Project names must contain only letters, numbers, hyphens, and underscores
- Use the `move` command to organize actions into subdirectories after creation

### Graph Actions

Display a visual dependency graph in the terminal:

```bash
action-lite graph
```

This shows:
- Root actions (no dependencies)
- Child actions indented under their dependencies
- Status and project for each action
- Priority markers

### Move Action

Move an action file and update all references:

```bash
action-lite move old-name.md new-name.md
action-lite move action.md subdir/action.md
```

This will:
- Move the action file
- Move any associated meta-graph directory
- Update all link references in other actions (handles both wiki-links `[[name]]` and markdown links `[text](path)`)

### Specify Action Directory

By default, action-lite searches the current directory. You can specify a different path:

```bash
action-lite --path /path/to/actions list
action-lite -p ./my-actions graph
```

## Meta-Graphs

For complex actions that contain sub-actions, create a directory with the same name as the action file (without the `.md` extension). Place all sub-actions in this directory:

```
actions/
  build-user-dashboard.md
  build-user-dashboard/
    create-metric-widgets.md
    setup-navigation.md
    implement-settings.md
```

## Status Workflow

Actions progress through these states:

1. **discovery** - Initial planning, defining requirements
2. **design** - Creating detailed design and specifications
3. **implementation** - Actively building/coding
4. **test** - Testing and quality assurance
5. **document** - Writing documentation and impact analysis
6. **publish** - Deploying or releasing
7. **published** - Complete and available for use

## Examples

See the `examples/actions` directory for sample action files demonstrating:
- Basic action structure
- Dependency linking
- Priority marking
- Different status states
- Meta-graph organization

Try the examples:

```bash
# List all example actions
action-lite --path examples/actions list

# Show the dependency graph
action-lite --path examples/actions graph

# Filter by priority
action-lite --path examples/actions priority

# Create a new action in the examples directory
action-lite --path examples/actions new "my-new-action" --project example
```

## Development

### Setup Development Environment

```bash
# Using Nix
nix develop

# Or manually install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build and Test

```bash
# Build
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy

# Watch for changes and rebuild
cargo watch -x build
```

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.
