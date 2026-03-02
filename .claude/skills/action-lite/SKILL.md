---
name: action-lite
description: "Use when creating, editing, or working with action files in the actions/ directory, or when managing action task tracking. Examples: create a new action, update action state, add action specifications, make a graph of action tree"
---

# Action-Lite Framework Skill

Use this skill when working with the action-lite task tracking system. Action-lite is an agile, file-based task management system using acyclic directed metagraphs to track requirements, state, strategy, and dependencies.

## Core Concepts

**Actions** are markdown documents stored in the `actions/` directory that declare expected outcomes and how to achieve them. Each action links architecture requirements directly to actionable tasks.

**Metagraphs** are subdirectories sharing the same name as an action file (without `.md`), containing sub-actions that break down complex tasks.

## Action Document Structure

Every action file must contain these sections in order:

```markdown
---
owners: []
state: discovery
priority: false
continuous: false
---

# Notes
General notes, research, POCs, learnings

# Statement of Action
Detailed task description with context and background. More in-depth than the title, may include why the action is needed.

# Statement of Inputs
- [Dependency Action Name](./Dependency%20Action%20Name.md)
Markdown links to other actions this depends on. Only actions at the same graph level.

# Statement of Specifications
- Requirement 1
- Requirement 2
List of requirements that must be met for completion.

# Statement of Design
Detailed design for how the action will be completed. Can include UML, diagrams, plain language explanations.
```

## Action States

Actions progress through these states:

| State | Type | Description | Outputs |
|-------|------|-------------|---------|
| **discovery** | Divergent | Research, POCs, understand problem space | Notes, Statement of Action, Inputs, Specifications |
| **design** | Convergent | Plan implementation 1-2 levels deep | Statement of Design |
| **implementation** | Convergent | Execute the design | Working output |
| **test** | Divergent | Stress test, verify against specs | Learnings (added to Notes) |
| **document** | Convergent | User/developer documentation | External docs |
| **published** | End state | Available for use by other actions | - |

### State Transition Rules
- Actions start in `discovery`
- Cannot enter `implementation` until all input actions are `published` AND discovery + design are complete
- Failed `test` can return to any previous state
- `continuous` actions may never reach `published`

## Working with Actions

### Creating a New Action
1. Create a new `.md` file in `actions/` with an actionable name (verb + noun)
2. Add frontmatter with initial state `discovery`
3. Fill in Notes and Statement of Action
4. Identify dependencies and add to Statement of Inputs
5. Define requirements in Statement of Specifications

### Naming Guidelines
- **Good**: "Create user authentication", "Remediate bug in payment flow", "Develop dashboard page"
- **Bad**: "User page", "Bug fix", "Authentication" (too vague, not actionable)
- Avoid naming actions after workflow stages: "Design X" or "Research Y" are part of the action workflow itself

### Creating Metagraphs
When an action needs to be broken down:
1. Create a directory with the same name as the action file (without `.md`)
2. Add sub-action `.md` files inside this directory
3. Sub-actions can reference each other as inputs (same level only)
4. Sub-actions cannot reference parent or sibling-of-parent actions as inputs

### Design Guidelines
- Plan 1-2 conceptual levels deep only
- State required behaviors and functions
- Leave irrelevant implementation details to the implementer
- Be verbose - clarity over brevity
- Avoid code except interfaces, type signatures, or pseudocode for complex algorithms
- Use diagrams, UML, charts as needed with plain language explanations

## Directory Structure Example

```
actions/
├── Create user system.md           # Parent action
├── Create user system/             # Metagraph directory
│   ├── Implement user model.md     # Sub-action
│   ├── Create auth endpoints.md    # Sub-action (can depend on user model)
│   └── Build user UI.md            # Sub-action
├── Setup database.md               # Another root action
└── Create API framework.md         # Another root action
```

## Workflow for Completing an Action

### Phase 1: Discovery (Divergent Thinking)

**Purpose**: Build a foundation of understanding before committing to a solution.

**Activities**:
1. **Research the problem space** - Read relevant documentation, explore similar solutions, understand constraints
2. **Take notes** - Document findings, potential approaches, tools considered, and rejected ideas with rationale
3. **Build POCs** - Test assumptions with small proof-of-concept implementations
4. **Define the action** - Write a clear Statement of Action explaining what needs to be done and why
5. **Identify dependencies** - Find existing actions this work depends on; add them to Statement of Inputs
6. **Define success criteria** - Write specific, testable requirements in Statement of Specifications

**Completion Criteria**:
- Notes section contains meaningful research
- Statement of Action clearly explains the task and its purpose
- All dependencies are identified and linked
- Specifications are specific enough to verify completion

**Transition**: Update frontmatter `state: design`

---

### Phase 2: Design (Convergent Thinking)

**Purpose**: Create a concrete plan that can be followed during implementation.

**Activities**:
1. **Verify inputs are ready** - All actions in Statement of Inputs should be `published`
2. **Review specifications** - Ensure you understand all requirements
3. **Plan the architecture** - Design 1-2 conceptual levels deep
4. **Consider breaking down** - If design is complex, create a metagraph with sub-actions
5. **Write the design** - Document in Statement of Design with enough detail for implementation

**Design Content Should Include**:
- Required behaviors and functions
- Data structures and interfaces (type signatures acceptable)
- Component interactions and data flow
- Pseudocode for complex algorithms only
- Diagrams (UML, flowcharts) with plain language explanations

**Design Should NOT Include**:
- Full code implementations
- Irrelevant implementation details
- Decisions that don't affect the outcome

**When to Create a Metagraph**:
- Design exceeds reasonable scope for a single implementation session
- Multiple distinct components need their own discovery/design cycles
- Different expertise needed for different parts
- Parallelization would benefit from separate tracking

**Completion Criteria**:
- Statement of Design is detailed enough for someone else to implement
- All complex decisions are documented with rationale
- Metagraph created if needed, with sub-actions in discovery state

**Transition**: Update frontmatter `state: implementation`

---

### Phase 3: Implementation (Convergent Thinking)

**Purpose**: Execute the design to produce working output.

**Pre-Implementation Checklist**:
- [ ] All input actions are `published`
- [ ] Discovery state is complete (notes, action, inputs, specs filled)
- [ ] Design state is complete (Statement of Design written)
- [ ] If metagraph exists, all sub-actions are `published`

**Activities**:
1. **Read the design thoroughly** - Understand the full plan before starting
2. **Follow the design** - Implement as specified, don't deviate without cause
3. **Document deviations** - If design issues arise, note them and consider returning to design state
4. **Keep scope focused** - Only implement what the design specifies

**Handling Issues**:
- **Minor clarification needed**: Make reasonable decision, document in Notes
- **Design flaw discovered**: Return to `design` state, update design, then resume
- **Requirements unclear**: Return to `discovery` state, clarify specs
- **Dependency issue**: Ensure input actions are truly complete; if not, wait or address

**Completion Criteria**:
- All designed components are implemented
- Implementation matches the design intent
- Code/output is ready for testing

**Transition**: Update frontmatter `state: test`

---

### Phase 4: Test (Divergent Thinking)

**Purpose**: Verify the implementation meets all specifications.

**Activities**:
1. **Review specifications** - Create test cases for each requirement
2. **Functional testing** - Verify each specification is met
3. **Stress testing** - Test edge cases, error conditions, performance limits
4. **Integration testing** - Verify it works with dependent/consuming actions
5. **Document findings** - Add learnings to Notes section

**Test Against Each Specification**:
For each item in Statement of Specifications:
- Design a test that would prove the requirement is met
- Execute the test
- Document pass/fail and any observations

**Handling Failures**:
- **Implementation bug**: Return to `implementation`, fix, re-test
- **Design flaw**: Return to `design`, revise approach, re-implement
- **Specification issue**: Return to `discovery`, clarify requirements
- **Add learnings**: Document what was learned in Notes regardless of outcome

**Completion Criteria**:
- All specifications verified as met
- Edge cases tested
- No critical issues remaining
- Learnings documented

**Transition**: Update frontmatter `state: document`

---

### Phase 5: Document (Convergent Thinking)

**Purpose**: Create user-facing and developer documentation.

**Activities**:
1. **User documentation** - How to use the output (if applicable)
2. **Developer documentation** - How to maintain/extend (if applicable)
3. **Code comments** - Ensure implementation is properly commented
4. **API documentation** - Document interfaces for consumers

**What NOT to Document Here**:
- Architecture documentation (already in the action itself)
- Design decisions (already in Statement of Design)
- Don't duplicate action content externally

**Completion Criteria**:
- Users can understand how to use the output
- Future maintainers can understand the code
- Documentation is discoverable

**Transition**: Update frontmatter `state: published`

---

### Phase 6: Published (End State)

**Purpose**: Mark the action as complete and available for consumption.

**Verification Before Publishing**:
- [ ] All specifications from Statement of Specifications are met
- [ ] Output is usable by other actions that depend on this one
- [ ] Documentation is complete
- [ ] If metagraph exists, all sub-actions are `published`

**For Continuous Actions**:
Some actions are never meant to be published (e.g., ongoing maintenance tasks). Set `continuous: true` in frontmatter. These cycle through discovery → design → implementation → test repeatedly.

**After Publishing**:
- Other actions can now use this action as an input
- Action can be referenced as a dependency
- Changes should spawn new actions rather than modifying published ones

---

## Handling Blocked Actions

When an action cannot proceed:

1. **Missing inputs**: Wait for dependency actions to reach `published`
2. **Unclear requirements**: Return to `discovery` for clarification
3. **External blockers**: Document in Notes, consider creating a new action for the blocker
4. **Scope creep**: Create new actions for out-of-scope work discovered during implementation

## Agent Commands

When working with actions:

### To list all actions and their states:
```bash
find actions -name "*.md" -exec sh -c 'echo "=== {} ===" && head -10 "{}"' \;
```

### To find actions by state:
```bash
grep -r "state: discovery" actions/ --include="*.md" -l
```

### To find priority actions:
```bash
grep -r "priority: true" actions/ --include="*.md" -l
```

### To find dependencies of an action:
Read the "Statement of Inputs" section of the action file.

### To find what depends on an action:
```bash
grep -r "ActionName.md" actions/ --include="*.md"
```

## Frontmatter Reference

```yaml
---
owners: ["user1", "user2"]  # List of responsible parties
state: discovery            # Current workflow state
priority: true              # High priority flag
continuous: false           # Never-ending action flag
---
```
