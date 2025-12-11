# Action-Lite Protocol Specification

This document contains the original specification for the action-lite file-based task tracking system.

## Overview

Action-lite is a file-based task tracking system that uses acyclic directed meta-graphs to track task requirements. It is designed to be simple, version-controllable, and aligned with the Action development methodology.

## Design Goals

Action-lite is similar to the action design system in goal but reduced in scope to accommodate the restrictions of its medium. The system must be file-based to ensure it can be version controlled and accessed without specialized software.

## File Structure

### Directory Layout

Action-lite uses a main directory to store all actions:
- Each action is stored in a markdown file
- Actions use specific tags for identification and sorting
- Meta-graph dependencies are stored in directories sharing the action's filename
- A meta-graph directory can exist without an associated action file

### File Naming

The action title becomes the filename (with `.md` extension).

### Tags

Each action file must include these tags:
- `#action` - Identifies this as an action file
- `#(state)` - Current state (see State Progression below)
- `#(project-name)` - Project identifier
- `#priority` - Optional, for important actions (remove when completed)

Tags should be lowercase and placed at the top of the file.

## File Sections

Each action file contains these sections (denoted by markdown `#` headers):

### 1. Notes
General notes on the task, context, and any relevant observations.

### 2. Statement of Action
The task to be performed, more in-depth than the title. Should include why the action is needed and what problem it solves.

### 3. Statement of Inputs
A markdown bullet list of `.md` links to other action files or wiki links. These represent dependencies that must be completed before this action.

### 4. Statement of Specifications
Requirements and constraints that must be met. Critical for the testing phase.

### 5. Statement of Design
One or more output/design pairs. Each output should have its own design section:

```markdown
# Statement of Design

## Output: [description of what will be produced]

#### Design
[Detailed design for how to proceed]
```

Multiple outputs can be specified with additional output/design pairs.

**Design Guidelines:**
- Write designs in **plain English**, not code
- Describe what will be built and how it will work conceptually
- Code snippets are acceptable for clarification (e.g., showing an API signature)
- Key logic should use **pseudocode ONLY** if absolutely necessary
- Focus on the approach and architecture, not the implementation details
- The design should guide implementation, not be the implementation

### 6. Analysis of Impact
How the implementation affects the system, lessons learned, side effects, and considerations for future work.

## State Progression

Actions progress through these states. State tags may be present at any point in the file and should be updated as work progresses.

### State Definitions

1. **discovery** - All actions start here
   - **Required outputs**: Title, Notes, Statement of Action, Statement of Inputs, Statement of Specifications

2. **design** - Design phase
   - **Required outputs**: Statement of Design
   - Design should be written in plain English, not code (code snippets acceptable, pseudocode only if necessary)

3. **implementation** - Follow design to completion
   - Build according to specifications and design

4. **test** - Verification phase
   - Go back to design if tests fail
   - Verify against Statement of Specifications

5. **document** - Documentation phase
   - **Required outputs**: Analysis of Impact

6. **publish** - Deployment/release
   - Deploy changes or make available

7. **published** - Completed and available for use
   - Final state

## Meta-Graphs

Meta-graphs represent task hierarchies and dependencies:
- Create a directory with the same name as the action file (without `.md` extension)
- Store dependent/sub-actions inside this directory
- All actions in the meta-graph directory are dependencies of the parent action
- Dependencies must form an acyclic directed graph (no circular dependencies)
- A meta-graph directory can exist without a corresponding action file
- **Sub-actions can only enter design or implementation states after their parent action has entered the implementation phase**

### Example Structure

```
actions/
├── Implement Authentication.md
├── Implement Authentication/
│   ├── Research OAuth.md
│   ├── Design Token Storage.md
│   └── Implement Login Flow.md
└── Fix Database Bug.md
```

In this example, "Implement Authentication" has three sub-actions that should be referenced in its Statement of Inputs.

## Rules and Constraints

- Actions can stay uncompleted for an infinite amount of time - no artificial deadlines
- State tags should be lowercase
- Priority tags should be removed once the action is completed
- Meta-graphs must be acyclic (no circular dependencies)
- File names should be descriptive and match the action title
- All sections should use markdown header syntax (`#`)
- **Sub-actions cannot progress to design or implementation phases until their parent action enters the implementation phase**
  - This ensures parent design is complete before sub-action decomposition
  - Prevents premature work that may not align with final parent design
  - Sub-actions may remain in discovery phase while gathering requirements

## Comparison with Full Action System

The action-lite system is a simplified, file-based version of the full Action development system. It maintains the core methodology (discovery, design, implementation, test, document, publish) while being implementable entirely through markdown files and directory structures.

Key differences:
- Full Action: May use databases or specialized software
- Action-Lite: Pure file-based system using markdown
- Full Action: May have additional features and tooling
- Action-Lite: Minimal tooling requirements, works with any text editor

## Tools and Integration

Action-lite can be used with:
- Any text editor that supports markdown
- Version control systems (git, etc.)
- File browsers for navigating meta-graphs
- grep/ripgrep for searching actions
- Tag-aware editors for filtering by state or project
