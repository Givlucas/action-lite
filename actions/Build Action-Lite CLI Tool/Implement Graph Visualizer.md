#action #design #action-lite #priority

# Notes

This sub-action implements part of the Output Formatting Component for the action-lite CLI tool, specifically the graph command visualization.

**Purpose:** Render the dependency DAG as a unicode tree structure that visually represents the hierarchy and relationships between actions.

**Dependencies on Other Sub-Actions:**
- Depends on: **Implement Dependency Parser and Graph Builder** - This component needs the DAG structure to visualize

**Technical Context:**
- Must render using unicode box-drawing characters (├──, └──, │)
- Must handle DAG visualization where each node appears exactly once
- More complex than tree visualization because of multiple-parent scenarios
- Must display action titles only, not file paths
- Must ensure visual clarity and proper indentation
- Part of the data pipeline: Dependency Resolver → Graph Visualizer → stdout

**How It Fits in the Overall System:**
This component is the final step in the `graph` command pipeline. It takes the DAG structure produced by the Dependency Parser and transforms it into a human-readable visual representation. This is the most complex output formatting component because it must handle DAG visualization (not just simple trees).

# Statement of Action

**What:** A graph visualization component that traverses a dependency DAG and renders it as a unicode tree structure with proper indentation and visual connectors.

**Why:** Visual representation of complex dependency structures makes them much easier to understand than text descriptions. This component enables users to quickly see the overall project structure, identify which actions are foundational vs dependent, and understand the flow of work. It answers: "How do all these actions relate to each other?"

# Statement of Inputs

This action depends on:

**Sub-Action Dependencies:**
- [Implement Dependency Parser and Graph Builder](./Implement Dependency Parser and Graph Builder.md) - Provides the DAG structure to visualize

**Knowledge Dependencies:**
- Understanding of unicode box-drawing characters (├──, └──, │)
- Knowledge of DAG traversal algorithms (different from tree traversal)
- Understanding that nodes with multiple parents should only appear once
- Knowledge of how to represent "reference" or "shared" nodes in tree-style output
- Terminal character width considerations for visual alignment

**Parent Action Specifications:**
This component implements specifications from the parent action:
- Specification #2: Command: graph requirements
- Specification #14: Graph command correctly visualizes the dependency DAG structure
- Specification #15: Graph rendering uses proper unicode tree characters
- Specification #16: Each action node appears exactly once in graph output (DAG property)

# Statement of Specifications

**Functional Requirements:**

1. **DAG Traversal**
   - Must traverse the dependency DAG structure
   - Must ensure each node is rendered exactly once (DAG property)
   - Must handle nodes with multiple parents correctly
   - Must determine appropriate traversal order (to be defined in design: depth-first, breadth-first, topological sort, etc.)

2. **Unicode Tree Rendering**
   - Must use unicode box-drawing characters for visual structure:
     - ├── for intermediate children
     - └── for last child
     - │ for continuation lines (vertical connectors)
   - Must maintain proper indentation for hierarchy levels
   - Must ensure visual alignment is correct

3. **Title Display**
   - Must display action titles only, not file paths
   - Must preserve title formatting (capitalization, special characters)
   - Must handle long titles without breaking layout

4. **DAG-Specific Visualization**
   - When a node has multiple parents, it should appear only once
   - Must visually indicate when a node is referenced elsewhere (e.g., "→ See: Action Title" or similar)
   - Must not duplicate nodes even if they have multiple incoming edges
   - Visual representation should make it clear which nodes are shared dependencies

5. **Output Format**
   - Must output to stdout only
   - Must be clear and readable in standard terminals
   - Must show full depth by default (as per parent specification #2)
   - Output should maintain visual consistency

**Technical Constraints:**

6. Must be implemented in Rust (parent specification #6)
7. Must output to stdout only (parent specification #11)
8. Must be read-only (parent specification #9)
9. Unicode rendering requires proper terminal support (Linux environment)

**Success Criteria:**

10. Correctly visualizes simple trees (no shared dependencies)
11. Correctly visualizes DAGs where nodes have multiple parents (each node appears once)
12. Uses proper unicode characters for visual hierarchy
13. Indentation and alignment are visually clear and consistent
14. Action titles are displayed correctly
15. Output is readable and intuitive for understanding dependencies
16. Handles edge cases (root with no children, deeply nested structures, wide graphs)

**Non-Requirements:**

17. No JSON or markdown output formats (parent specification #20)
18. No color or terminal formatting beyond unicode characters
19. No interactive features (expanding/collapsing nodes)
20. No graph filtering options in initial version
21. No depth limiting (show full depth by default)
22. No horizontal layout or alternate visualization styles
23. No statistics or metrics display (node counts, depth, etc.)

# Statement of Design

[Leave empty - to be filled when action progresses to #design phase]
