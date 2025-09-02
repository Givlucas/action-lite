# Action Lite Workspace

Welcome to your Action Lite workspace! This directory contains all your projects and actions organized using the Action Lite methodology.

## Structure

```
workspace/
├── .action-lite          # Workspace marker file
├── README.md            # This file
├── project-1/           # Project directory
│   ├── action-1.md      # Action file
│   ├── action-2.md      # Another action file
│   └── action-1/        # Meta-graph directory (if exists)
│       ├── sub-action-1.md
│       └── sub-action-2.md
└── project-2/           # Another project directory
    └── action-3.md
```

## Action Files

Each action is stored as a markdown file with the following structure:

### Tags
- `#project` - Indicates this is a project action
- `#action` - Indicates this is an action
- `#<status>` - Current status (discovery, design, implement, test, document, publish, published)
- `#<project-name>` - The project this action belongs to
- `#priority` - Optional priority marker

### Sections
1. **Notes** - General notes on the task
2. **Statement of Action** - The task to be performed, more in-depth than title
3. **Statement of Inputs** - A list of .md links to other markdown files
4. **Statement of Design** - Output and design sections per output
5. **Analysis of Impact** - Impact analysis (required for document stage and later)

## Status Flow

Actions progress through these statuses:

1. **discovery** - All actions start here
2. **design** - Design the solution
3. **implement** - Follow design to implement
4. **test** - Test implementation (go back to design if tests fail)
5. **document** - Document the results
6. **publish** - Deploy changes / make available
7. **published** - Available for use

## Usage

Use the `action` command-line tool to manage your workspace:

```bash
# Create a new action
action new project-name "Action Title"

# List all actions
action list

# Filter actions by project or status
action list --project project-name --status discovery

# Show action details
action show project-name "Action Title"

# Update action status
action status project-name "Action Title" design

# Set priority
action priority project-name "Action Title" --set

# Edit an action
action edit project-name "Action Title"

# Create a meta-graph
action meta-graph project-name "Action Title"

# Validate workspace
action validate
```

## Meta-graphs

When an action needs to be broken down into sub-actions, create a meta-graph directory. This directory has the same name as the action file (without .md extension) and contains all related sub-actions.
