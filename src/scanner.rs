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

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanError::DirectoryNotFound(path) => {
                write!(f, "Directory not found: {}", path.display())
            }
            ScanError::PermissionDenied(path) => {
                write!(f, "Permission denied accessing: {}", path.display())
            }
            ScanError::IoError(path, err) => {
                write!(f, "IO error at {}: {}", path.display(), err)
            }
        }
    }
}

impl std::error::Error for ScanError {}

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
    // Validate that root_path exists
    if !root_path.exists() {
        return Err(ScanError::DirectoryNotFound(root_path.to_path_buf()));
    }

    // Validate that root_path is a directory
    if !root_path.is_dir() {
        return Err(ScanError::DirectoryNotFound(root_path.to_path_buf()));
    }

    // Initialize empty results vector
    let mut results = Vec::new();

    // Call recursive scanner
    scan_directory_recursive(root_path, &mut results)?;

    Ok(results)
}

/// Helper function to determine if a file is an action file
fn is_action_file(path: &Path) -> bool {
    // Check extension is ".md"
    if path.extension().and_then(|s| s.to_str()) != Some("md") {
        return false;
    }

    // Get filename without path
    let filename = match path.file_name().and_then(|s| s.to_str()) {
        Some(name) => name,
        None => return false,
    };

    // Exclude "README.md" (case-insensitive comparison)
    if filename.to_lowercase() == "readme.md" {
        return false;
    }

    true
}

/// Recursive helper for directory traversal
fn scan_directory_recursive(dir: &Path, results: &mut Vec<PathBuf>) -> ScanResult<()> {
    // Use fs::read_dir() to get directory entries
    let entries = fs::read_dir(dir).map_err(|err| {
        // Handle permission denied errors
        if err.kind() == std::io::ErrorKind::PermissionDenied {
            ScanError::PermissionDenied(dir.to_path_buf())
        } else {
            ScanError::IoError(dir.to_path_buf(), err)
        }
    })?;

    // Iterate over each entry
    for entry in entries {
        let entry = entry.map_err(|err| {
            ScanError::IoError(dir.to_path_buf(), err)
        })?;

        let path = entry.path();

        if path.is_dir() {
            // For directories: recurse into them
            scan_directory_recursive(&path, results)?;
        } else if path.is_file() {
            // For files: check if they're action files
            if is_action_file(&path) {
                // Convert to absolute path using canonicalize
                let absolute_path = fs::canonicalize(&path).map_err(|err| {
                    ScanError::IoError(path.clone(), err)
                })?;
                results.push(absolute_path);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    /// Helper to create a temporary test directory
    fn create_temp_dir(name: &str) -> PathBuf {
        let temp_base = std::env::temp_dir();
        let test_dir = temp_base.join(format!("action_lite_test_{}", name));

        // Clean up if it exists from previous test
        if test_dir.exists() {
            fs::remove_dir_all(&test_dir).ok();
        }

        fs::create_dir_all(&test_dir).expect("Failed to create test directory");
        test_dir
    }

    /// Helper to create a file with content
    fn create_file(dir: &Path, name: &str, content: &str) {
        let file_path = dir.join(name);
        let mut file = File::create(file_path).expect("Failed to create test file");
        file.write_all(content.as_bytes()).expect("Failed to write test file");
    }

    #[test]
    fn test_scan_empty_directory() {
        let test_dir = create_temp_dir("empty");

        let result = scan_actions(&test_dir).expect("Should succeed on empty directory");

        assert_eq!(result.len(), 0, "Empty directory should return no files");

        // Cleanup
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_scan_single_action_file() {
        let test_dir = create_temp_dir("single");
        create_file(&test_dir, "test-action.md", "# Test Action");

        let result = scan_actions(&test_dir).expect("Should succeed with single file");

        assert_eq!(result.len(), 1, "Should find one action file");
        assert!(result[0].ends_with("test-action.md"), "Should be the correct file");

        // Cleanup
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_scan_readme_filtered() {
        let test_dir = create_temp_dir("readme");
        create_file(&test_dir, "README.md", "# README");
        create_file(&test_dir, "readme.md", "# readme");

        let result = scan_actions(&test_dir).expect("Should succeed");

        assert_eq!(result.len(), 0, "README.md files should be filtered out");

        // Cleanup
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_scan_nested_structure() {
        let test_dir = create_temp_dir("nested");

        // Create nested structure
        let sub_dir = test_dir.join("subdir");
        fs::create_dir(&sub_dir).expect("Failed to create subdirectory");

        let deep_dir = sub_dir.join("deeper");
        fs::create_dir(&deep_dir).expect("Failed to create deep directory");

        // Create files at various depths
        create_file(&test_dir, "root-action.md", "# Root");
        create_file(&sub_dir, "sub-action.md", "# Sub");
        create_file(&deep_dir, "deep-action.md", "# Deep");

        let result = scan_actions(&test_dir).expect("Should succeed with nested structure");

        assert_eq!(result.len(), 3, "Should find all three action files");

        // Cleanup
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_scan_mixed_file_types() {
        let test_dir = create_temp_dir("mixed");

        create_file(&test_dir, "action.md", "# Action");
        create_file(&test_dir, "notes.txt", "Notes");
        create_file(&test_dir, "code.rs", "fn main() {}");
        create_file(&test_dir, "README.md", "# README");
        create_file(&test_dir, "another-action.md", "# Another");

        let result = scan_actions(&test_dir).expect("Should succeed with mixed types");

        assert_eq!(result.len(), 2, "Should find only .md files excluding README");

        // Cleanup
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_scan_nonexistent_directory() {
        let nonexistent = PathBuf::from("/nonexistent/path/that/does/not/exist");

        let result = scan_actions(&nonexistent);

        assert!(result.is_err(), "Should return error for nonexistent directory");

        if let Err(ScanError::DirectoryNotFound(path)) = result {
            assert_eq!(path, nonexistent);
        } else {
            panic!("Expected DirectoryNotFound error");
        }
    }

    #[test]
    fn test_is_action_file() {
        // Test markdown files
        assert!(is_action_file(Path::new("action.md")));
        assert!(is_action_file(Path::new("path/to/action.md")));

        // Test README filtering
        assert!(!is_action_file(Path::new("README.md")));
        assert!(!is_action_file(Path::new("readme.md")));
        assert!(!is_action_file(Path::new("path/README.md")));

        // Test non-markdown files
        assert!(!is_action_file(Path::new("action.txt")));
        assert!(!is_action_file(Path::new("action.rs")));
        assert!(!is_action_file(Path::new("action")));
    }
}
