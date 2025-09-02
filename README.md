# Action Lite

A file-based task tracking system implementing the Action Lite methodology. Action Lite provides a structured approach to managing projects and tasks using markdown files organized in a hierarchical directory structure.

## Features

- **File-based storage**: All actions stored as markdown files for easy version control
- **Project organization**: Actions grouped by projects in subdirectories  
- **Status tracking**: Seven-stage workflow from discovery to published
- **Priority management**: Mark actions as high priority
- **Meta-graphs**: Break down complex actions into sub-actions
- **Tag-based organization**: Automatic tagging for easy filtering and sorting
- **CLI interface**: Complete command-line tool for workspace management
- **Validation**: Ensure workspace integrity and required sections

## Installation

### Using Nix Flakes

```bash
# Run directly
nix run github:yourusername/action-lite

# Install to profile
nix profile install github:yourusername/action-lite

# Use in development
nix develop github:yourusername/action-lite
```

### Building from Source

```bash
git clone https://github.com/yourusername/action-lite
cd action-lite
cargo build --release
```

## Quick Start

```bash
# Initialize a new workspace
action init

# Create your first action
action new "my-project" "Setup development environment" --priority

# List all actions
action list

# Update action status
action status "my-project" "Setup development environment" design

# Edit an action
action edit "my-project" "Setup development environment"
```

## Action Lifecycle

Actions progress through these statuses:

1. **discovery** - Define the task, inputs, and specifications
2. **design** - Create detailed design and output specifications  
3. **implement** - Execute the design
4. **test** - Validate implementation (return to design if tests fail)
5. **document** - Analyze impact and create documentation
6. **publish** - Deploy or make changes available
7. **published** - Task is complete and available for use

## File Structure

```
workspace/
├── .action-lite              # Workspace marker
├── README.md                # Workspace documentation
├── project-1/               # Project directory
│   ├── action-1.md          # Action file
│   ├── action-2.md          # Another action
│   └── action-1/            # Meta-graph directory
│       ├── sub-action-1.md  # Sub-action
│       └── sub-action-2.md  # Sub-action
└── project-2/
    └── action-3.md
```

## Action File Format

Each action is a markdown file with frontmatter and structured sections:

```markdown
---
id: 550e8400-e29b-41d4-a716-446655440000
created_at: 2025-01-15T10:30:00Z
updated_at: 2025-01-15T15:45:00Z
---

# Setup Development Environment

#project #action #discovery #my-project #priority

## Notes

Setting up the development environment for the new web application project.

## Statement of Action

Install and configure all necessary development tools including Node.js, database, 
and development server. This is needed to begin implementation of the web application.

## Statement of Inputs

- [Project Requirements](./project-requirements.md)
- [Technology Stack Decision](./tech-stack.md)

## Statement of Design

### Output

Fully configured development environment ready for implementation.

### Design

1. Install Node.js 18+ and npm
2. Set up PostgreSQL database  
3. Configure development server with hot reload
4. Install and configure linting tools
5. Set up testing framework

## Analysis of Impact

The development environment setup enables the team to begin implementation work
efficiently and establishes consistent tooling across all developers.
```

## Commands

### Workspace Management

```bash
# Initialize workspace
action init [path]

# Validate workspace
action validate
```

### Action Management

```bash
# Create new action
action new <project> <title> [--priority]

# List actions
action list [--project <name>] [--status <status>] [--priority]

# Show action details  
action show <project> <title>

# Edit action
action edit <project> <title>
```

### Status and Priority

```bash
# Update status
action status <project> <title> <new-status>

# Set/unset priority
action priority <project> <title> --set/--unset
```

### Meta-graphs

```bash
# Create meta-graph directory
action meta-graph <project> <title>
```

## Development

### Prerequisites

- Rust 1.70+
- Nix (optional, for flake-based development)

### Building

```bash
# Standard cargo build
cargo build

# Using Nix
nix build
```

### Testing

```bash
cargo test
```

### Development Shell

```bash
# Using Nix
nix develop

# Standard cargo
cargo run -- help
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
