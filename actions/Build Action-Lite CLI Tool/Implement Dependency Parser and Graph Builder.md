#action #design #action-lite #priority

# Notes

This sub-action implements the Dependency Resolution Component for the action-lite CLI tool. It transforms the Statement of Inputs sections from multiple actions into a directed acyclic graph (DAG) that represents the dependency structure.

**Purpose:** Parse dependency references from the Statement of Inputs section of action files and build a directed acyclic graph (DAG) representing the dependency relationships between actions.

**Dependencies on Other Sub-Actions:**
- Depends on: **Implement Action Metadata Parser** - This component needs parsed Action objects with Statement of Inputs content to extract dependencies from

**Technical Context:**
- Must support two link formats: markdown links `[Text](./path/to/action.md)` and wiki links `[[Action Title]]`
- Must resolve relative file paths to match against actual action files
- Must build a DAG, not a tree (actions with multiple parents appear only once)
- Must detect circular dependencies and report errors (DAGs must be acyclic)
- Must handle missing dependencies (referenced actions that don't exist)
- Part of the data pipeline: Parser → Dependency Resolver → Graph Visualizer

**How It Fits in the Overall System:**
This component is critical for the `graph` command. It takes parsed Action objects and builds the dependency graph structure that the Graph Visualizer will render. Without this component, we can't visualize relationships between actions or understand the overall project structure.

# Statement of Action

**What:** A dependency parsing and graph building component that extracts dependency references from Statement of Inputs sections, resolves them to actual actions, and constructs a directed acyclic graph (DAG) of action relationships.

**Why:** Actions depend on other actions, and understanding these relationships is crucial for planning work. This component builds the graph structure that enables visualization and analysis of dependencies. It answers: "What does this action depend on?" and "What's the overall structure of this project?"

# Statement of Inputs

This action depends on:

**Sub-Action Dependencies:**
- [Implement Action Metadata Parser](./Implement Action Metadata Parser.md) - Provides parsed Action objects with Statement of Inputs content

**Knowledge Dependencies:**
- Understanding of markdown link syntax: `[Link Text](./path/to/file.md)`
- Understanding of wiki link syntax: `[[Action Title]]`
- Knowledge of relative path resolution in markdown
- Understanding of directed acyclic graphs (DAG) and their properties
- Algorithm for detecting cycles in graphs
- Understanding of how meta-graphs work (subdirectories with related actions)

**Parent Action Specifications:**
This component implements specifications from the parent action:
- Specification #5: Dependency Resolution requirements
- Specification #16: Each action node appears exactly once (DAG property)

# Statement of Specifications

**Functional Requirements:**

1. **Statement of Inputs Parsing**
   - Must parse the Statement of Inputs section content from Action objects
   - Must identify dependency references within that section
   - Must handle cases where Statement of Inputs is empty or missing (no dependencies)

2. **Link Format Support**
   - Must support markdown link format: `[Link Text](./path/to/action.md)`
   - Must support wiki link format: `[[Action Title]]`
   - Must extract the target reference (file path or action title) from each link
   - Must handle relative paths (./path, ../path) correctly

3. **Reference Resolution**
   - Must resolve relative file paths to match against actual action files
   - Must resolve wiki links by matching against action titles
   - Must handle file extension variations (.md vs no extension)
   - Must match references case-sensitively (or define case rules in design)

4. **Graph Construction**
   - Must build a directed graph with actions as nodes and dependencies as edges
   - Edge direction: if Action A depends on Action B, there's an edge from A to B
   - Each action must appear as a single node even if referenced by multiple other actions
   - Graph structure must represent a DAG (directed acyclic graph)

5. **Cycle Detection**
   - Must detect circular dependencies in the graph
   - Must report error if circular dependency is found (not allowed in action-lite)
   - Error message should identify which actions form the cycle

6. **Missing Dependency Handling**
   - Must detect when a referenced action doesn't exist in the file set
   - Must report error with clear message identifying missing dependency
   - Must indicate which action references the missing dependency

**Technical Constraints:**

7. Must be implemented in Rust (parent specification #6)
8. Must fail on malformed references or circular dependencies (parent specification #10)
9. Must be read-only (parent specification #9)

**Success Criteria:**

10. Successfully parses both markdown link and wiki link formats
11. Correctly resolves relative paths to actual action files
12. Builds accurate DAG representing actual dependency relationships
13. Each action appears exactly once in the graph structure
14. Detects and reports circular dependencies with clear error messages
15. Detects and reports missing dependencies with clear error messages
16. Handles complex graph structures (multiple parents, multiple children, deep nesting)

**Non-Requirements:**

17. No need to parse dependency descriptions or reasons (just the links)
18. No need to handle external links (http://, https://)
19. No need to distinguish between different types of dependencies
20. No need to calculate transitive dependencies or dependency depth
21. No graph optimization or simplification (represent structure as-is)
22. No support for conditional or optional dependencies

# Statement of Design

[Leave empty - to be filled when action progresses to #design phase]
