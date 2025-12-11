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

[Leave empty - to be filled when action progresses to #design phase]
