---
name: action-framework
description: The Action development methodology for turning requirements into implemented features. Use when planning work, creating specifications, or when asked about development process. Includes action-lite, a file-based task tracking system using acyclic directed meta-graphs.
---

# Action Framework

A development methodology that ensures design and testing are not skipped. The action-lite system provides a practical file-based implementation of this methodology.

## Core Methodology

### Phases

#### 1. Discovery (Divergent)
- Gather user requirements
- Add specifications if not already present
- Allow thought process to branch out and expand
- Explore the problem space

#### 2. Design (Convergent)
- Bring all ideas together
- Design is baked into the action
- Cannot be skipped like in Scrum
- Create explicit design before implementation
- Only specify design 1-2 levels conceptually down.
- **Design should be written in plain English, not code**
  - Explain what will be built and how it will work
  - Key logic should use pseudocode ONLY if absolutely necessary
  - Focus on describing the approach, not writing the implementation

#### 3. Implementation
- Build according to design
- Follow the specifications

#### 4. Testing
- Verify against specifications
- If testing fails → cycle back to Discovery or Design
- Guaranteed test phase (not optional)

#### 5. Documentation
- Capture the analysis of impact
- Document what was learned
- Record how the implementation affects the system

#### 6. Publishing
- Deploy changes or make available for use
- Final release step

### Comparison with Scrum

| Aspect | Scrum | Action |
|--------|-------|--------|
| Design | Optional, can skip | Built-in, required |
| Testing | Not defined, optional | Guaranteed phase |
| Failure | Can one-shot a PBI even if flawed | Forces cycle back on failure |

### Key Insight

The Action cycle will force you back to Discovery or Design if implementation or testing fails. Since design is not optional, you cannot complete an action and later discover the design was flawed without addressing it.

### Divergent vs Convergent Thinking

- **Divergent (Discovery)**: Branching out, expanding possibilities
- **Convergent (Design)**: Bringing together, narrowing to solution

Sometimes setting aside specific time for divergent activities as their own work item is necessary to populate the backlog. Backlog refinement is a convergent flow and requires a divergent thinking step to align items.

## Action-Lite: File-Based Implementation

Action-lite is a practical file-based task tracking system that implements the Action methodology. It uses acyclic directed meta-graphs to track task dependencies and requirements.

### File Structure

Actions are stored in a main directory structure:
- Each action is a markdown file with the title as the filename
- Meta-graph dependencies are stored in a directory sharing the action's name
- A meta-graph directory can exist without an associated action file

**Example Structure:**
```
actions/
├── Feature Authentication.md
├── Feature Authentication/
│   ├── Research OAuth providers.md
│   ├── Design token storage.md
│   └── Implement login flow.md
└── Bug Fix Database Connection.md
```

### Tagging System

Each action file must include the following tags at the top:
- `#action` - Identifies this as an action file
- `#(state)` - Current state (discovery, design, implementation, test, document, publish, published)
- `#(project-name)` - Project identifier for grouping
- `#priority` - Optional, marks important actions (remove when completed)

**Example:**
```markdown
#action #design #authentication-system #priority
```

### Action File Sections

Each action file contains these sections (denoted by `#` markdown headers):

#### 1. Notes
General notes on the task, context, and observations.

#### 2. Statement of Action
The task to be performed, more in-depth than the title. Should include why the action is needed and what problem it solves.

#### 3. Statement of Inputs
A markdown bullet list of dependencies using `.md` links to other action files or wiki links.

**Example:**
```markdown
# Statement of Inputs
- [Research OAuth providers](./Feature%20Authentication/Research%20OAuth%20providers.md)
- [Design token storage](./Feature%20Authentication/Design%20token%20storage.md)
```

#### 4. Statement of Specifications
Requirements and constraints that must be met. This section is critical for testing phase.

#### 5. Statement of Design
One or more output/design pairs:

**Format:**
```markdown
# Statement of Design

## Output: [what will be produced]

#### Design
[Detailed design for how to proceed]

## Output: [another output if applicable]

#### Design
[Another design section]
```

**Design Guidelines:**
- Write designs in **plain English**, not code
- Describe what will be built and how it will work conceptually
- Key logic should use **pseudocode ONLY** if absolutely necessary
- Focus on the approach and architecture, not the implementation details
- The design should guide implementation, not be the implementation
- Only specify design 1-2 levels conceptually down.

#### 6. Analysis of Impact
How the implementation affects the system, what was learned, and any side effects or considerations for future work.

### State Progression

Actions progress through these states. State tags may be updated at any point in the file:

1. **discovery** - All actions start here
   - **Outputs**: Title, Notes, Statement of Action, Statement of Inputs, Statement of Specifications

2. **design** - Design phase
   - **Outputs**: Statement of Design

3. **implementation** - Follow design to completion
   - Build according to specifications and design

4. **test** - Verification phase
   - Go back to design if tests fail
   - Verify against Statement of Specifications

5. **document** - Documentation phase
   - **Outputs**: Analysis of Impact

6. **publish** - Deployment/release
   - Make changes available

7. **published** - Completed and available for use
   - Final state

### Meta-Graphs and Dependencies

Meta-graphs represent task hierarchies and dependencies:
- Create a directory with the same name as an action file to create a meta-graph
- Store dependent actions inside this directory
- Dependencies form an acyclic directed graph (no circular dependencies)
- Use Statement of Inputs to link to prerequisite actions
- **Sub-actions can only enter design or implementation states after their parent action has entered the implementation phase**

This structure allows you to:
- Break complex actions into smaller, manageable sub-actions
- Track which actions must be completed before others
- Maintain clear dependency chains
- Organize related work hierarchically

**Parent-Child State Constraint:**
Sub-actions in a meta-graph directory cannot progress to the design or implementation phases until their parent action has reached the implementation phase. This ensures:
- The parent's design is complete before breaking down into detailed sub-tasks
- Sub-actions align with the parent's overall design
- No premature optimization or misaligned work
- Clear understanding of the goal before decomposition into parts

### Working with Actions

**Creating a new action:**
1. Create a markdown file with a descriptive title
2. Add tags: `#action #discovery #project-name`
3. Fill in Notes and Statement of Action
4. Identify inputs/dependencies
5. Add specifications

**Progressing an action:**
1. Update the state tag as you move through phases
2. Complete required outputs for each phase
3. If testing fails, update state back to `#design` or `#discovery`
4. Remove `#priority` tag when action is completed

**Creating dependencies:**
1. Create a directory matching the action filename
2. Create sub-action files inside this directory
3. Reference sub-actions in the parent's Statement of Inputs

### Best Practices

- Actions can remain uncompleted indefinitely - no artificial time pressure
- Only add `#priority` tag when truly important, remove after completion
- Use descriptive filenames that clearly identify the action
- Keep Statement of Action focused on the "what" and "why"
- Make specifications testable and concrete
- Update state tags as soon as phase changes occur
- Use meta-graphs to break down complex actions into manageable pieces
- Maintain acyclic dependency graphs - no circular references
- Only specify design 1-2 levels conceptually down.
