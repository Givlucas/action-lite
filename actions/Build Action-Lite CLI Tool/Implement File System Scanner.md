#action #document #action-lite #priority

# Notes

This sub-action implements the File System Discovery Component for the action-lite CLI tool. It is one of the foundational components that all other components depend on, as it provides the raw list of action markdown files to be processed.

**Purpose:** Recursively scan the actions/ directory to locate all action markdown files, filtering out non-action files like README.md and other documentation.

**Dependencies on Other Sub-Actions:** None - this is an independent component with no dependencies on other sub-actions.

**Technical Context:**
- Must work with Linux file systems
- Must handle nested directory structures (meta-graphs)
- Must filter markdown files to identify only action files
- Must handle file system errors gracefully (permission issues, missing directories)
- Part of the data pipeline: File System → Action Parser → other components

**How It Fits in the Overall System:**
This component is the entry point for all data processing. Both the `list` and `graph` commands start by scanning the file system to find action files. Without this component, the tool has no input data to work with.

# Statement of Action

**What:** A file system scanning component that recursively traverses the actions/ directory and identifies all action markdown files, returning their file paths for further processing.

**Why:** The CLI tool needs to discover all action files before it can parse them, build dependency graphs, or filter for priority actions. This component provides the foundation for all subsequent operations by answering the question: "What action files exist in this project?"

# Statement of Inputs

This action depends on:

**Sub-Action Dependencies:**
- None (independent component)

**Knowledge Dependencies:**
- Understanding of the actions/ directory structure and meta-graph organization
- Knowledge of which markdown files are action files vs documentation (README.md exclusion)
- Understanding of recursive directory traversal requirements

**Parent Action Specifications:**
This component implements specifications from the parent action:
- Specification #3: Action Discovery requirements
- Specification #4: Must scan all markdown files and subdirectories
- Specification #5: Must recursively traverse directory structure

# Statement of Specifications

**Functional Requirements:**

1. **Recursive Directory Traversal**
   - Must recursively scan the actions/ root directory
   - Must include files in subdirectories (meta-graph directories)
   - Must traverse to arbitrary depth

2. **Markdown File Identification**
   - Must identify all .md files within the scanned directories
   - Must filter out non-action markdown files (e.g., README.md)
   - File identification should be based on file extension (.md)

3. **File Path Collection**
   - Must collect and return file paths to all identified action files
   - Paths should be absolute or relative as appropriate for downstream parsing
   - Must preserve path information needed to resolve relative references

4. **Error Handling**
   - Must handle file system errors gracefully (permission denied, missing directories)
   - Must report errors clearly to enable debugging
   - Must fail if the actions/ root directory doesn't exist

**Technical Constraints:**

5. Must be implemented in Rust (parent specification #6)
6. Must target Linux platform only (parent specification #8)
7. Must be read-only - no file modifications (parent specification #9)

**Success Criteria:**

8. Can successfully scan a valid actions/ directory structure with nested subdirectories
9. Returns complete list of action markdown files, including those in meta-graphs
10. Correctly filters out README.md and other documentation files
11. Handles file system errors without crashing
12. Performance is acceptable for directories with hundreds of action files

**Non-Requirements:**

13. No need to parse file contents (that's the parser's job)
14. No need to validate that files are well-formed action files
15. No need to sort or order the returned file paths
16. No need to handle symbolic links or special file system features
17. No caching or performance optimization beyond basic recursive traversal

# Statement of Design

This design defines the file system scanning component that recursively discovers all action markdown files within the actions/ directory structure.

## Design Overview

The File System Scanner is a pure function that takes a root directory path and returns a collection of absolute file paths to action markdown files. It uses Rust's standard library `std::fs` for directory traversal and applies filtering rules to identify valid action files.

**Core Principle:** Simple recursive traversal with clear filtering rules. No caching, no state - just scan and return.

## Module Structure

Location: `src/scanner.rs`

The scanner will be a standalone module that can be used by both the `list` and `graph` commands.

```rust
// src/scanner.rs
//! File system scanner for discovering action markdown files.
//!
//! This module provides functionality to recursively scan a directory
//! structure and identify all action markdown files, filtering out
//! documentation and other non-action files.

use std::path::{Path, PathBuf};
use std::fs;

/// Errors that can occur during file system scanning
#[derive(Debug)]
pub enum ScanError {
    /// The actions directory does not exist
    DirectoryNotFound(PathBuf),
    /// Permission denied when accessing a directory or file
    PermissionDenied(PathBuf),
    /// Other IO error during traversal
    IoError(PathBuf, std::io::Error),
}

/// Result type for scanner operations
pub type ScanResult<T> = Result<T, ScanError>;

/// Scans a directory recursively for action markdown files
///
/// # Arguments
/// * `root_path` - The root directory to scan (typically "actions/")
///
/// # Returns
/// A vector of absolute paths to action markdown files
///
/// # Errors
/// Returns ScanError if the directory doesn't exist, permission is denied,
/// or other IO errors occur.
pub fn scan_actions(root_path: &Path) -> ScanResult<Vec<PathBuf>> {
    // Implementation details below
}

/// Helper function to determine if a file is an action file
fn is_action_file(path: &Path) -> bool {
    // Implementation details below
}

/// Recursive helper for directory traversal
fn scan_directory_recursive(dir: &Path, results: &mut Vec<PathBuf>) -> ScanResult<()> {
    // Implementation details below
}
```

## Data Structures

### Primary Types

**Input:**
- `&Path` - Reference to the root directory path (e.g., "actions/")

**Output:**
- `Vec<PathBuf>` - Collection of absolute file paths to action markdown files

**Error Types:**
```rust
pub enum ScanError {
    DirectoryNotFound(PathBuf),
    PermissionDenied(PathBuf),
    IoError(PathBuf, std::io::Error),
}
```

## Algorithm: Recursive Directory Traversal

### High-Level Algorithm

```
function scan_actions(root_path):
    if not root_path.exists():
        return error DirectoryNotFound

    if not root_path.is_dir():
        return error DirectoryNotFound

    results = empty vector
    scan_directory_recursive(root_path, results)
    return results

function scan_directory_recursive(dir, results):
    for entry in read_dir(dir):
        path = entry.path()

        if path.is_directory():
            scan_directory_recursive(path, results)  // Recurse into subdirectories

        else if path.is_file():
            if is_action_file(path):
                results.push(absolute_path(path))

function is_action_file(path):
    if not path.extension() == "md":
        return false

    filename = path.file_name()

    // Filter out README.md and similar documentation files
    if filename.to_lowercase() == "readme.md":
        return false

    return true
```

### Detailed Implementation Logic

**1. Entry Point (`scan_actions`)**
- Validate that root_path exists
- Validate that root_path is a directory
- Fail with `DirectoryNotFound` if validation fails
- Initialize empty results vector
- Call recursive scanner
- Return results or propagate errors

**2. Recursive Traversal (`scan_directory_recursive`)**
- Use `fs::read_dir()` to get directory entries
- Iterate over each entry
- For directories: recurse into them
- For files: check if they're action files
- Handle `PermissionDenied` errors gracefully (add to error context)
- Propagate other IO errors

**3. File Filtering (`is_action_file`)**
- Check extension is ".md"
- Get filename without path
- Exclude "README.md" (case-insensitive comparison)
- Exclude "readme.md"
- Future expansion: could exclude other patterns like "CHANGELOG.md"

**4. Path Handling**
- Convert all returned paths to absolute paths
- Use `fs::canonicalize()` for absolute path resolution
- This ensures paths can be used reliably by downstream components

## Error Handling Strategy

**Error Categories:**

1. **Missing Directory (Fatal)**
   - If the root actions/ directory doesn't exist, fail immediately
   - This is a user error - the tool requires an actions/ directory
   - Return `ScanError::DirectoryNotFound`

2. **Permission Denied (Contextual)**
   - If a subdirectory can't be accessed, capture the path
   - Continue scanning other directories
   - Report all permission errors in the final error
   - Consider: Should we fail-fast or accumulate errors?
   - **Decision:** Fail-fast on permission errors for simplicity

3. **Other IO Errors (Fatal)**
   - Unexpected IO errors should fail immediately
   - Wrap the error with context about which path caused it
   - Return `ScanError::IoError(path, error)`

**Error Reporting:**
- Each error variant includes the problematic path
- Error messages should be clear and actionable
- Use Rust's `std::fmt::Display` for user-friendly error messages

## Performance Considerations

**Scalability:**
- Linear time complexity: O(n) where n = total files and directories
- No recursion depth limits in Rust (uses heap, not stack)
- No sorting or filtering beyond basic file type checks
- Expected performance: thousands of files in milliseconds

**Memory:**
- Results vector grows with number of action files
- No caching or memoization needed
- Paths are `PathBuf` (owned strings) - acceptable overhead

**Optimization Decisions:**
- No parallel traversal (not needed for expected scale)
- No async IO (CLI tool, immediate results expected)
- No directory watching or incremental scanning
- No symlink following (specification #16: not required)

## Testing Approach

**Unit Tests:**

1. **Test: Scan empty directory**
   - Create temp directory with no files
   - Expect empty results vector

2. **Test: Scan directory with single action file**
   - Create temp directory with one .md file
   - Expect vector with one path

3. **Test: Scan directory with README.md only**
   - Create temp directory with README.md
   - Expect empty results (README filtered out)

4. **Test: Scan nested directory structure**
   - Create temp directory with subdirectories
   - Place .md files at various depths
   - Expect all action files found

5. **Test: Mixed file types**
   - Create directory with .md, .txt, .rs files
   - Expect only .md files (excluding README.md)

6. **Test: Non-existent directory**
   - Pass path to directory that doesn't exist
   - Expect `DirectoryNotFound` error

7. **Test: Permission denied (Linux-specific)**
   - Create directory with restricted permissions
   - Expect appropriate error handling

**Integration Tests:**
- Test with actual actions/ directory structure from the project
- Verify it finds all real action files
- Verify it excludes any README files in meta-graph directories

## Integration with Other Components

**Consumers of this module:**
- CLI Command Interface (calls scanner to get file list)
- Used by both `list` and `graph` commands
- Parser module receives the file paths from scanner

**Data Flow:**
```
User runs command
    → CLI parses args
    → CLI calls scanner.scan_actions("actions/")
    → Scanner returns Vec<PathBuf>
    → CLI passes paths to parser
    → Parser processes each file
```

## Dependencies

**Standard Library Only:**
- `std::fs` - File system operations (read_dir, metadata)
- `std::path` - Path manipulation (Path, PathBuf)
- `std::io` - Error types

**No External Crates Needed:**
- This component is simple enough for std lib only
- No need for walkdir or similar crates
- Keeps dependency tree minimal

## Example Usage

```rust
use scanner::scan_actions;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let actions_dir = Path::new("actions");

    let action_files = scan_actions(actions_dir)?;

    println!("Found {} action files:", action_files.len());
    for path in action_files {
        println!("  {}", path.display());
    }

    Ok(())
}
```

## Implementation Notes

**Rust-Specific Decisions:**

1. **Use `&Path` for input, `PathBuf` for output**
   - Input is borrowed (no ownership transfer needed)
   - Output is owned (caller needs to own the paths)

2. **Error handling with Result type**
   - Use custom `ScanError` enum for domain-specific errors
   - Implement `std::error::Error` trait for ScanError
   - Implement `Display` trait for user-friendly messages

3. **Recursive function design**
   - Helper function takes mutable reference to results vector
   - Avoids creating and merging vectors at each recursion level
   - More efficient for Rust's ownership model

4. **Path canonicalization**
   - Use `fs::canonicalize()` to get absolute paths
   - This resolves relative paths and symlinks (if we encounter them)
   - Downstream components can use paths reliably

## Future Considerations

**Not Implemented Initially (Can Add Later if Needed):**

1. **Configurable filtering**
   - Allow custom exclude patterns beyond README.md
   - Could accept a filter function as parameter

2. **Symlink handling**
   - Currently specification says no symlink support needed
   - Could add `fs::metadata()` vs `fs::symlink_metadata()` if required

3. **Parallel traversal**
   - Could use rayon for parallel directory scanning
   - Only worth it if performance becomes an issue (unlikely)

4. **Progress reporting**
   - Could yield partial results during scan
   - Would require iterator-based API instead of vector return

**Design Stability:**
The current design is intentionally simple and should not need significant changes. The interface (function signature) is stable and future enhancements can be internal implementation details.

## Summary

This design provides a straightforward, reliable file system scanner using Rust's standard library. It handles the core requirements (recursive traversal, markdown filtering, error handling) without unnecessary complexity. The module is testable, has clear error handling, and integrates cleanly with other components through a simple function interface.
