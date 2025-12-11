#action #design #action-lite #priority

# Notes

This sub-action implements the Action Parsing Component for the action-lite CLI tool. It transforms raw markdown files into structured data objects that can be used by downstream components for filtering, dependency resolution, and visualization.

**Purpose:** Parse action markdown files to extract tags, title, sections, phase, and priority status, producing structured Action objects.

**Dependencies on Other Sub-Actions:**
- Depends on: **Implement File System Scanner** - This component needs file paths to know which files to parse

**Technical Context:**
- Must parse the action-lite markdown format with its specific structure
- Tag line is always the first line with specific format: #action #phase #project-tags [#priority]
- Must validate tag structure and fail on malformed files (per parent specification #10)
- Phase tags are: #discovery, #design, #implementation, #test, #document, #publish
- Must extract "Statement of Inputs" section for dependency parsing
- Part of the data pipeline: File System → Action Parser → Dependency Resolver/Formatters

**How It Fits in the Overall System:**
This component transforms unstructured markdown into structured data that all other components can work with. It's the bridge between raw files and the tool's internal data model. The dependency resolver, list formatter, and graph visualizer all depend on this component producing well-structured Action objects.

# Statement of Action

**What:** A markdown parsing component that reads action files, validates their structure, extracts metadata (tags, title, phase, priority), and produces structured Action objects for use by other components.

**Why:** Raw markdown files aren't directly useful for building dependency graphs or filtering priority actions. This component transforms them into a structured format that enables all other operations. It also enforces the action-lite format by failing on malformed files, ensuring data quality throughout the tool.

# Statement of Inputs

This action depends on:

**Sub-Action Dependencies:**
- [Implement File System Scanner](./Implement File System Scanner.md) - Provides the file paths that need to be parsed

**Knowledge Dependencies:**
- action-lite markdown format specification (tag structure, required sections)
- Phase tag definitions (#discovery, #design, #implementation, #test, #document, #publish)
- Understanding of markdown heading and section structure
- Tag validation rules

**Parent Action Specifications:**
This component implements specifications from the parent action:
- Specification #4: Metadata Parsing requirements
- Specification #10: Must fail on malformed action files

# Statement of Specifications

**Functional Requirements:**

1. **Tag Line Parsing**
   - Must parse the first line of the file to extract all tags
   - Must validate that #action tag is present
   - Must detect phase from phase tags (#discovery, #design, etc.)
   - Must detect priority status from #priority tag
   - Must extract project tags (e.g., #action-lite)

2. **Tag Validation**
   - Must fail if #action tag is missing
   - Must fail if no phase tag is present
   - Must fail if multiple phase tags are present
   - Must fail if tag line format is invalid
   - Error messages must clearly indicate what validation failed

3. **Title Extraction**
   - Must extract the action title from the first markdown heading (# Heading)
   - Title should be extracted after the tag line
   - Must handle titles with special characters or spaces

4. **Section Parsing**
   - Must identify and extract the "Statement of Inputs" section
   - Must handle cases where section is missing (report as no dependencies)
   - Section content should be extracted for dependency parsing
   - Must recognize section boundaries (next heading or end of file)

5. **Structured Output**
   - Must produce a structured Action object containing:
     - File path
     - Title
     - Current phase
     - Priority status (boolean)
     - Project tags
     - Statement of Inputs content
   - Data structure should be easily consumable by other components

**Technical Constraints:**

6. Must be implemented in Rust (parent specification #6)
7. Must fail on malformed action files - no silent error recovery (parent specification #10)
8. Must be read-only - no file modifications (parent specification #9)

**Success Criteria:**

9. Can successfully parse all well-formed action files in a test set
10. Correctly extracts tags, title, phase, and priority from valid files
11. Fails with clear error messages on malformed files
12. Produces Action objects that contain all necessary information for downstream components
13. Handles edge cases (missing sections, special characters in titles, etc.)

**Non-Requirements:**

14. No need to parse the full markdown structure (only tags, title, and Statement of Inputs)
15. No need to validate Statement of Inputs content (that's the dependency parser's job)
16. No need to parse other sections (Statement of Action, Statement of Design, etc.)
17. No need to handle multiple file formats (only action-lite markdown)
18. No need to support malformed files gracefully - failing is correct behavior

# Statement of Design

[Leave empty - to be filled when action progresses to #design phase]
