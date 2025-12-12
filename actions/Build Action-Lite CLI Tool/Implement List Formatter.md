#action #design #action-lite #priority

# Notes

This sub-action implements part of the Output Formatting Component for the action-lite CLI tool, specifically the list command output format.

**Purpose:** Format and display priority actions in a simple, readable list format for the `list` command.

**Dependencies on Other Sub-Actions:**
- Depends on: **Implement Action Metadata Parser** - This component needs parsed Action objects with priority information to format

**Technical Context:**
- Must filter actions to show only those tagged with #priority
- Must display action titles, not file paths
- Must output to stdout only (no file output, no alternate formats)
- Simpler than graph visualization, making it a good testing ground for the parser
- Part of the data pipeline: Parser → List Formatter → stdout

**How It Fits in the Overall System:**
This component transforms parsed Action objects into human-readable output for the `list` command. It's the final step in the pipeline for displaying priority actions. It depends on the parser producing correct Action objects with priority flags.

# Statement of Action

**What:** A formatting component that filters Action objects to those with priority status and displays them as a simple list of titles to stdout.

**Why:** Users need a quick way to see which actions are marked as priority. This component provides that by taking parsed action data and presenting it in a clear, scannable format. It answers the question: "What should I work on next?"

# Statement of Inputs

This action depends on:

**Sub-Action Dependencies:**
- [Implement Action Metadata Parser](./Implement Action Metadata Parser.md) - Provides structured Action objects with priority information

**Knowledge Dependencies:**
- Understanding of what constitutes "priority" (presence of #priority tag)
- Knowledge of desired output format (simple list vs. other formats)
- Understanding that output must be action titles only, not file paths

**Parent Action Specifications:**
This component implements specifications from the parent action:
- Specification #1: Command: list requirements
- Specification #11: Must output to stdout only

# Statement of Specifications

**Functional Requirements:**

1. **Priority Filtering**
   - Must filter Action objects to only those with #priority tag
   - Must handle case where no priority actions exist (display appropriate message)
   - Must not display non-priority actions

2. **Title Display**
   - Must display action titles only, not file paths
   - Must display one action per line
   - Must preserve title formatting (capitalization, special characters)

3. **Output Format**
   - Must output to stdout only
   - Format should be simple and scannable
   - No additional decoration or formatting (just titles)
   - Output should be in a consistent order (to be determined in design: alphabetical, by file path, by phase, etc.)

4. **Empty Result Handling**
   - Must handle case where no priority actions exist
   - Should display helpful message like "No priority actions found"
   - Should not output error or empty output silently

**Technical Constraints:**

5. Must be implemented in Rust (parent specification #6)
6. Must output to stdout only - no file output (parent specification #11)
7. Must be read-only (parent specification #9)

**Success Criteria:**

8. Displays all and only priority actions from a test set
9. Shows action titles, not file paths
10. Output is clear and easy to scan
11. Handles empty results gracefully
12. Output format is consistent across runs

**Non-Requirements:**

13. No JSON or markdown output formats (parent specification #20)
14. No verbose mode or additional metadata display (parent specification #21)
15. No sorting options or filtering beyond priority
16. No color or terminal formatting (keeps output simple and portable)
17. No pagination or limiting of output
18. No interactive features

# Statement of Design

## Overview

The list formatter is a straightforward component that transforms a collection of parsed Action objects into a simple, scannable text list. The design prioritizes simplicity and clarity over features.

**Architecture:** The list command follows a linear pipeline:
```
actions/ directory → Scanner → Parser → Filter → Sort → Format → stdout
```

The formatter itself only handles steps 4-6 (filter, sort, format). Steps 1-3 are handled by the scanner and parser modules.

## Data Flow

**Input:** `Vec<Action>` from parser containing all actions found in the actions/ directory
**Output:** Text lines to stdout showing priority action titles

**Processing steps:**
1. Filter: Keep only actions where `action.priority == true`
2. Sort: Alphabetically by `action.title` (case-sensitive lexicographic ordering)
3. Format: Output each title as a single line, or "No priority actions found." if empty

## Design Decisions

### Decision 1: Alphabetical Sorting

**Choice:** Sort priority actions alphabetically by title (case-sensitive).

**Rationale:**
- Provides consistent output across runs (deterministic)
- Makes it easy to scan and find specific actions
- No clear natural ordering exists (by phase? by file path? arbitrary)
- Alphabetical is the most predictable and familiar ordering

**Alternatives considered:**
- Sort by phase (discovery first, published last): Would group related actions but makes finding specific actions harder
- Sort by file path: Would maintain directory structure but file paths don't always reflect logical order
- Preserve file system order: Non-deterministic, changes based on file system

### Decision 2: Simple Title-Only Output

**Choice:** Output only action titles, one per line, with no additional decoration.

**Rationale:**
- Specification #3 requires "simple and scannable" format
- Specification #16 excludes color and terminal formatting
- Title-only output is easiest to scan visually
- Can be easily piped to other tools (grep, wc, etc.)
- Reduces noise and cognitive load

**Format:**
```
Action Title 1
Action Title 2
Action Title 3
```

**Alternatives considered:**
- Show phase or other metadata: Violates specification #15 (no verbose mode)
- Show file paths: Violates specification #9 (display titles, not paths)
- Number the list: Adds visual noise without clear benefit
- Add headers or separators: Violates simplicity requirement

### Decision 3: Empty Result Message

**Choice:** When no priority actions exist, output: "No priority actions found."

**Rationale:**
- Specification #4 requires helpful message for empty results
- Silent output would be confusing (did it work? are there really no actions?)
- Clear message confirms the command worked correctly
- Follows convention from other CLI tools (grep "no matches", find "nothing found")

**Message format:** Simple declarative sentence ending with period, outputting to stdout (not stderr).

### Decision 4: Error Handling Strategy

**Choice:** Propagate scanner and parser errors up through CommandError enum. Don't catch or suppress errors.

**Rationale:**
- Specification #7 (parent): Must fail on malformed actions
- Better to fail loudly than silently skip broken actions
- Scanner and parser errors are already well-formatted and informative
- Keeps formatter code simple - just handle the happy path

**Error sources:**
- No actions/ directory: Scanner returns error
- File read failures: Scanner returns error
- Malformed action files: Parser returns error

All these errors are propagated via the `?` operator and returned as CommandError.

## Implementation Details

### Module Structure

**Location:** `src/commands/list.rs`

**Function signature:**
```rust
pub fn execute() -> CommandResult
```

**Dependencies:**
- `crate::scanner` - for scanning actions/ directory
- `crate::parser` - for parsing action files
- `std::path::Path` - for file system operations

### Algorithm

```rust
pub fn execute() -> CommandResult {
    // 1. Verify actions directory exists
    let actions_dir = Path::new("actions");
    if !actions_dir.exists() {
        return Err(CommandError::NoActionsDirectory);
    }

    // 2. Scan for action files
    let action_files = scanner::scan_actions(actions_dir)
        .map_err(CommandError::ScanError)?;

    // 3. Parse action files
    let actions = parser::parse_all_actions(action_files)
        .map_err(CommandError::ParseError)?;

    // 4. Filter for priority actions
    let mut priority_actions: Vec<_> = actions.iter()
        .filter(|action| action.priority)
        .collect();

    // 5. Sort alphabetically by title
    priority_actions.sort_by(|a, b| a.title.cmp(&b.title));

    // 6. Format and output
    if priority_actions.is_empty() {
        println!("No priority actions found.");
    } else {
        for action in priority_actions {
            println!("{}", action.title);
        }
    }

    Ok(())
}
```

### Testing Strategy

**Unit tests cover:**
1. Filtering priority actions from mixed list
2. Alphabetical sorting behavior
3. Empty priority list handling
4. Single priority action
5. All priority actions
6. Title preservation (special characters, capitalization)
7. Case-sensitive sorting behavior
8. Sorting with numbers (lexicographic order)

**Integration testing:**
- Requires actual action files in test fixtures
- Test via `cargo run -- list` with known action set
- Verify output matches expected titles

### Edge Cases

1. **No actions/ directory:** Return error (handled by scanner)
2. **Empty actions/ directory:** Return "No priority actions found."
3. **No priority actions:** Return "No priority actions found."
4. **Single priority action:** Output single title
5. **All actions are priority:** Output all titles
6. **Malformed action file:** Return parse error (handled by parser)
7. **Very long titles:** Output as-is (no truncation or wrapping)
8. **Titles with newlines:** Not possible - titles come from filenames, which can't contain newlines
9. **Identical titles:** Both appear in output (sorted stably)

## Performance Considerations

**Time complexity:**
- Scanning: O(n) where n = number of files
- Parsing: O(n × m) where m = average file size
- Filtering: O(n)
- Sorting: O(n log n) where n = number of priority actions
- Output: O(n)

**Overall:** O(n log n) dominated by sorting

**Memory:** O(n) to store all parsed actions

**Optimization notes:**
- For typical usage (< 100 actions), performance is not a concern
- No need for streaming or lazy evaluation
- Sorting is necessary for consistent output, can't be avoided
- Could optimize by parsing only priority actions, but complicates scanner/parser interface

## Open Questions

None - design is complete and straightforward.

## Design Approval

This design implements all specifications from the Statement of Specifications:
- ✅ Spec 1: Priority filtering
- ✅ Spec 2: Title display
- ✅ Spec 3: Output format (simple, stdout only)
- ✅ Spec 4: Empty result handling
- ✅ Spec 5-7: Technical constraints (Rust, stdout only, read-only)
- ✅ Spec 8-12: Success criteria
- ✅ Spec 13-18: Non-requirements acknowledged

Ready for implementation.
