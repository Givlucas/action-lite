---
owners: []
state: discovery
priority: false
continuous: false
---

# Notes
- Look at the "beads" AI tool. (you have it stared in github)
- Frontmatter headers instead of tags?
- Assignment info?

# Statement of Action
Create a command-line interface tool for managing the action-lite task tracking system. This tool will provide an ergonomic way to create, list, query, and manage action files without manually editing markdown.

# Statement of Inputs
- [Create action-lite protocol](Create%20action-lite%20protocol.md)
- [Create action agent orchestrator workflow](Create%20action%20agent%20orchestrator%20workflow.md)
- [Create philosophy document and development tool context](Create%20philosophy%20document%20and%20development%20tool%20context.md)

# Statement of Specifications
- Built in Rust
- Contains a Nix flake for development and packaging
- Features:
  - Colored output
  - Create a new empty action with all sections and frontmatter
  - List all actions and their status
  - List priority items
  - List continuous items
  - List items by status
  - Verify an action's input tree are all published
  - Add new inputs to an action
  - Change the status of an action
  - Move actions and update all input references to them
  - Graph the actions in terminal as a flow chart for a single level; allow user to enter an action's metagraphs if present; should take over entire terminal screen

# Statement of Design
