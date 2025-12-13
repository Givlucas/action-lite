#action #test #action-lite #priority

# Notes

This action was created to build a command-line tool for managing and visualizing actions within the action-lite methodology framework. The stakeholder requirements gathering revealed the following context:

**Project Context:**
- Part of the action-lite project for structured development workflows
- Marked as priority work

**Background:**
The action-lite methodology uses markdown files to structure development work through phases (discovery, design, implementation, test, document, publish). This tool will help developers visualize and understand the dependency graph of actions, making it easier to see what work needs to be done and in what order.

**Key Technical Decisions:**
- Rust chosen for implementation (performance, reliability)
- Nix for build and packaging (reproducible builds, dev environment)
- Linux-only target platform (simplified scope)
- Read-only tool (no creation or editing capabilities)

**Visualization Approach:**
The graph visualization treats the action structure as a DAG (Directed Acyclic Graph) rather than a tree, meaning actions that have multiple parent dependencies should only appear once in the visualization. This is similar to how Mermaid graphs work, preventing duplicate nodes when an action is depended upon by multiple other actions.

**Risks and Considerations:**
- Need to handle malformed action files gracefully (requirement is to fail on malformed files)
- Unicode rendering for tree structure requires proper terminal support
- DAG visualization logic will be more complex than simple tree traversal
- Tag parsing must be robust to properly detect phases and priority status

# Statement of Action

Build a command-line tool in Rust that visualizes and lists actions within the action-lite methodology framework.

**What:** A CLI tool that can:
1. List all priority tasks (actions tagged with #priority)
2. Generate a tree-style graph visualization of the action dependency structure

**Why:** Enable developers using action-lite to quickly understand:
- Which tasks are marked as priority and need attention
- How actions depend on each other (the dependency graph)
- The overall structure of work to be done

This solves the problem of managing complex, interdependent actions by providing clear visualization of relationships and priorities, making it easier to plan work and understand project structure.

# Statement of Inputs

This action has been broken down into sub-actions (meta-graph):

**Sub-Actions:**

1. [Configure Nix Build with Naersk](./Build Action-Lite CLI Tool/Configure Nix Build with Naersk.md) - Set up Nix flake with naersk for building Rust CLI (#published - Complete)
2. [Implement File System Scanner](./Build Action-Lite CLI Tool/Implement File System Scanner.md) - Recursive directory traversal and markdown file discovery (#published - Complete)
3. [Implement Action Metadata Parser](./Build Action-Lite CLI Tool/Implement Action Metadata Parser.md) - Parse tags, titles, and section structure from markdown (#published - Complete)
4. [Implement CLI Command Interface](./Build Action-Lite CLI Tool/Implement CLI Command Interface.md) - Parse arguments and dispatch to list/graph commands (#published - Complete)
5. [Implement List Formatter](./Build Action-Lite CLI Tool/Implement List Formatter.md) - Format priority actions for list command (#published - Complete)
6. [Implement Dependency Parser and Graph Builder](./Build Action-Lite CLI Tool/Implement Dependency Parser and Graph Builder.md) - Extract dependencies from markdown links and build DAG (#published - Complete)
7. [Implement Graph Visualizer](./Build Action-Lite CLI Tool/Implement Graph Visualizer.md) - Render DAG as unicode tree structure (#published - Complete)

**Knowledge Dependencies:**
- action-lite format specification (tag structure, file organization, meta-graphs)
- action-lite phase definitions (discovery, design, implementation, test, document, publish)
- Understanding of markdown file parsing requirements
- DAG traversal and visualization algorithms
- Unicode box-drawing characters for tree rendering

Note: Formal documentation of these inputs should be created if it doesn't already exist.

# Statement of Specifications

**Functional Requirements:**

1. **Command: list**
   - Display all actions tagged with #priority
   - Show only action titles, not full file paths
   - Output to stdout only

2. **Command: graph**
   - Visualize action dependency tree using unicode characters (├──, └──, │)
   - Display action titles only, not full file paths
   - Default to showing full depth from selected node
   - Render as DAG: each action appears only once even if it has multiple parent dependencies
   - Support filtering options (specific filtering requirements to be refined in design phase)

3. **Action Discovery**
   - Scan all markdown files under the actions root directory
   - Include files in subdirectories (meta-graph directories)
   - Recursively traverse directory structure

4. **Metadata Parsing**
   - Parse tags from first line of action files
   - Validate tag format and presence
   - Detect phase based on phase tags (#discovery, #design, #implementation, #test, #document, #publish)
   - Identify priority status via #priority tag
   - No additional metadata sources required beyond tags

5. **Dependency Resolution**
   - Parse "Statement of Inputs" section for dependencies
   - Support markdown link format: [Link Text](./path/to/action.md)
   - Support wiki link format: [[Action Title]]
   - Build dependency graph from parsed relationships

**Technical Constraints:**

6. Must be written in Rust
7. Must use Nix for build and packaging configuration
8. Must target Linux platform only (no cross-platform support required)
9. Must be read-only (no creation, editing, or modification of action files)
10. Must fail on malformed action files (no silent error recovery)
11. Must output to stdout only (no file output, no JSON/markdown format options)

**Success Criteria:**

12. Tool can successfully parse all valid action files in the actions directory
13. Priority list command accurately identifies and displays all #priority actions
14. Graph command correctly visualizes the dependency DAG structure
15. Graph rendering uses proper unicode tree characters for visual hierarchy
16. Each action node appears exactly once in graph output (DAG property)
17. Tool fails with clear error message when encountering malformed action files
18. Tool runs without errors or bugs on valid action file sets
19. Can be built and run using Nix on Linux systems

**Non-Requirements (Explicitly Out of Scope):**

20. No JSON or markdown output formats
21. No verbose/quiet mode flags (standard output only)
22. No action file creation or editing capabilities
23. No support for non-Linux platforms

# Statement of Design

This design defines the high-level architecture and component breakdown for the action-lite CLI tool. The design follows the principle of defining components one conceptual level above implementation details.

## Architectural Overview

The CLI tool consists of six major conceptual components that work together in a pipeline:

```
File System → Action Parser → Dependency Resolver → Command Handler → Output Formatter → stdout
```

Each component has a single, well-defined responsibility and communicates through clear data structures.

## Component Breakdown

### 1. Build System Component
**Purpose:** Provide the infrastructure to compile, package, and run the Rust CLI tool.

**Sub-Actions:**
- [Configure Nix Build with Naersk](./Build Action-Lite CLI Tool/Configure Nix Build with Naersk.md) - Existing sub-action in #design phase

**Responsibilities:**
- Set up Nix flake with naersk for reproducible builds
- Configure development environment with Rust toolchain
- Enable `nix build` and `nix run` commands
- Provide Cargo integration for development workflow

**Interface:** Produces an executable binary that can be invoked from the command line.

---

### 2. File System Discovery Component
**Purpose:** Locate all action markdown files within the actions/ directory structure.

**Sub-Actions:** (To be created)
- "Implement File System Scanner" - Recursive directory traversal and markdown file discovery

**Responsibilities:**
- Recursively scan the actions/ directory
- Identify all .md files (action files and meta-graph structures)
- Filter out non-action markdown files (README.md, documentation)
- Handle file system errors gracefully
- Provide a collection of file paths to the parser

**Interface:** Accepts a root directory path, outputs a list of file paths to action markdown files.

---

### 3. Action Parsing Component
**Purpose:** Extract structured metadata and content from action markdown files.

**Sub-Actions:** (To be created)
- "Implement Action Metadata Parser" - Parse tags, titles, and section structure from markdown

**Responsibilities:**
- Parse the first line to extract tags (#action, phase tags, #priority, project tags)
- Validate tag format and fail on malformed files
- Extract action title (first heading)
- Identify and extract the "Statement of Inputs" section
- Detect current phase from phase tags
- Detect priority status from #priority tag
- Represent each action as a structured data object

**Interface:** Accepts file path and content, outputs a structured Action object with metadata and sections.

**Design Principle:** Fail fast on malformed files. If tags are invalid or required structure is missing, report error and exit.

---

### 4. Dependency Resolution Component
**Purpose:** Parse dependency references and construct the directed acyclic graph (DAG) of action relationships.

**Sub-Actions:** (To be created)
- "Implement Dependency Parser and Graph Builder" - Extract dependencies from markdown links and build DAG

**Responsibilities:**
- Parse "Statement of Inputs" section for dependency references
- Support markdown link format: `[Link Text](./path/to/action.md)`
- Support wiki link format: `[[Action Title]]`
- Resolve relative file paths to absolute action identifiers
- Match wiki links to action titles
- Build directed graph structure with edges representing dependencies
- Validate that the graph is acyclic (detect circular dependencies)
- Handle missing dependencies gracefully (report error)

**Interface:** Accepts a collection of parsed Action objects, outputs a DAG data structure with nodes and edges.

**Design Principle:** The graph is a DAG, not a tree. Actions with multiple parents appear only once.

---

### 5. Command Interface Component
**Purpose:** Parse command-line arguments and dispatch to appropriate command handlers.

**Sub-Actions:** (To be created)
- "Implement CLI Command Interface" - Parse arguments and dispatch to list/graph commands

**Responsibilities:**
- Parse command-line arguments (subcommands: list, graph)
- Validate command syntax
- Provide help text and usage information
- Dispatch to appropriate command handler
- Handle errors in command parsing

**Interface:** Accepts argv, outputs which command to execute with any parameters.

**Supported Commands:**
- `action-lite list` - Show all priority actions
- `action-lite graph` - Show dependency graph visualization
- `action-lite help` - Display usage information

---

### 6. Output Formatting Component
**Purpose:** Transform action data and graph structures into human-readable text output.

**Sub-Actions:** (To be created)
- "Implement List Formatter" - Format priority actions for list command
- "Implement Graph Visualizer" - Render DAG as unicode tree structure

**Responsibilities:**

**For list command:**
- Filter actions to only those with #priority tag
- Format as simple list of action titles
- Output to stdout

**For graph command:**
- Traverse DAG structure
- Render using unicode tree characters (├──, └──, │)
- Show action titles only (not file paths)
- Handle DAG visualization (each node appears once)
- Ensure visual clarity and proper indentation

**Interface:** Accepts either a filtered list of actions or a DAG structure, outputs formatted text to stdout.

**Design Principle:** Output is always to stdout. No file output, no alternate formats.

---

## Component Interaction Flow

### Flow for `list` command:
1. Command Interface parses arguments → identifies "list" command
2. File System Discovery scans actions/ → produces file paths
3. Action Parser reads each file → produces Action objects
4. Filter Action objects where priority == true
5. Output Formatter renders list → stdout

### Flow for `graph` command:
1. Command Interface parses arguments → identifies "graph" command
2. File System Discovery scans actions/ → produces file paths
3. Action Parser reads each file → produces Action objects
4. Dependency Resolver builds DAG from Actions → produces graph structure
5. Output Formatter traverses and renders DAG → stdout

## Data Flow and Dependencies

**Key Data Structures** (conceptual, implementation details in sub-actions):
- `ActionFile`: Represents a parsed action with metadata (title, tags, phase, priority, dependencies)
- `DependencyGraph`: Represents the DAG structure with nodes and edges
- `Command`: Enum of supported commands (List, Graph, Help)

**Component Dependencies:**
- Action Parser depends on File System Discovery (needs file paths)
- Dependency Resolver depends on Action Parser (needs parsed Action objects)
- Output Formatting depends on either parsed Actions (list) or Dependency Graph (graph)
- Command Interface is independent and coordinates the pipeline

## Error Handling Strategy

Each component must handle errors at its conceptual level:
- **File System Discovery:** Report IO errors, inaccessible directories
- **Action Parser:** Fail on malformed tag lines, invalid structure
- **Dependency Resolver:** Report circular dependencies, missing referenced actions
- **Command Interface:** Report invalid commands, show usage help
- **Output Formatting:** Report rendering errors (should be rare)

**General Principle:** Fail fast with clear error messages. No silent error recovery per specification #10.

## Testing Strategy

Each component should be testable independently:
- File System Discovery: Test with sample directory structures
- Action Parser: Test with valid and malformed markdown files
- Dependency Resolver: Test with various graph structures (simple trees, DAGs, cycles)
- Command Interface: Test argument parsing with various inputs
- Output Formatting: Test rendering with known graph structures

Integration tests should verify the complete pipeline works with realistic action file sets.

## Implementation Order

Recommended order for sub-action implementation (based on dependencies):

1. **Configure Nix Build with Naersk** (already in progress) - Foundation for development
2. **Implement File System Scanner** - Needed by all other components
3. **Implement Action Metadata Parser** - Needed by dependency resolver and formatters
4. **Implement CLI Command Interface** - Needed to make the tool usable
5. **Implement List Formatter** - Simpler output format, good for testing parser
6. **Implement Dependency Parser and Graph Builder** - More complex, needs parser working
7. **Implement Graph Visualizer** - Most complex output format, needs full DAG

## Design Principles Summary

- **Separation of Concerns:** Each component has one clear responsibility
- **Single Pipeline:** Data flows in one direction through the components
- **Fail Fast:** Invalid input causes immediate error, no recovery
- **Testability:** Components can be tested independently
- **Simplicity:** No complex features, no configuration files, no plugins
- **Read-Only:** Tool never modifies action files
- **Stdout Only:** All output goes to stdout, no file output
