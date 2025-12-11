//! Action metadata parser for extracting structured information from markdown files.
//!
//! This module parses action-lite markdown files to extract tags, title, phase,
//! priority status, and sections. It validates the action file format and fails
//! on malformed files.

use std::path::{Path, PathBuf};
use std::fs;

/// Represents a parsed action with all metadata
#[derive(Debug, Clone, PartialEq)]
pub struct Action {
    /// Absolute path to the action file
    pub path: PathBuf,
    /// Action title extracted from first heading
    pub title: String,
    /// Current phase of the action
    pub phase: Phase,
    /// Whether this is a priority action
    pub priority: bool,
    /// Project tags (e.g., "action-lite", "rust")
    pub project_tags: Vec<String>,
    /// Content of the Statement of Inputs section (if present)
    pub statement_of_inputs: Option<String>,
}

/// Valid phases for action-lite workflow
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Discovery,
    Design,
    Implementation,
    Test,
    Document,
    Publish,
}

impl Phase {
    /// Parse a phase from a tag string (e.g., "#discovery" -> Phase::Discovery)
    fn from_tag(tag: &str) -> Option<Phase> {
        match tag {
            "#discovery" => Some(Phase::Discovery),
            "#design" => Some(Phase::Design),
            "#implementation" => Some(Phase::Implementation),
            "#test" => Some(Phase::Test),
            "#document" => Some(Phase::Document),
            "#publish" => Some(Phase::Publish),
            _ => None,
        }
    }

    /// Convert phase to tag string
    pub fn to_tag(&self) -> &'static str {
        match self {
            Phase::Discovery => "#discovery",
            Phase::Design => "#design",
            Phase::Implementation => "#implementation",
            Phase::Test => "#test",
            Phase::Document => "#document",
            Phase::Publish => "#publish",
        }
    }
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_tag())
    }
}

/// Errors that can occur during action parsing
#[derive(Debug)]
pub enum ParseError {
    /// Failed to read the file
    IoError(PathBuf, std::io::Error),
    /// File is not valid UTF-8
    InvalidUtf8(PathBuf),
    /// Missing required #action tag
    MissingActionTag(PathBuf),
    /// No phase tag found
    MissingPhaseTag(PathBuf),
    /// Multiple phase tags found
    MultiplePhaseTag(PathBuf, Vec<String>),
    /// Invalid tag line format (first line must start with tags)
    InvalidTagLine(PathBuf),
    /// No title heading found
    MissingTitle(PathBuf),
    /// Empty file
    EmptyFile(PathBuf),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::IoError(path, err) => {
                write!(f, "Failed to read {}: {}", path.display(), err)
            }
            ParseError::InvalidUtf8(path) => {
                write!(f, "File is not valid UTF-8: {}", path.display())
            }
            ParseError::MissingActionTag(path) => {
                write!(f, "Missing required #action tag in first line: {}", path.display())
            }
            ParseError::MissingPhaseTag(path) => {
                write!(f, "No phase tag found in first line (expected one of: #discovery, #design, #implementation, #test, #document, #publish): {}", path.display())
            }
            ParseError::MultiplePhaseTag(path, phases) => {
                write!(f, "Multiple phase tags found in {}: {}", path.display(), phases.join(", "))
            }
            ParseError::InvalidTagLine(path) => {
                write!(f, "Invalid tag line format in {} (first line must contain tags starting with #)", path.display())
            }
            ParseError::MissingTitle(path) => {
                write!(f, "No title heading (# Heading) found in {}", path.display())
            }
            ParseError::EmptyFile(path) => {
                write!(f, "File is empty: {}", path.display())
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// Result type for parser operations
pub type ParseResult<T> = Result<T, ParseError>;

/// Parse an action file and extract all metadata
///
/// # Arguments
/// * `path` - Path to the action markdown file
///
/// # Returns
/// An `Action` struct with all parsed metadata
///
/// # Errors
/// Returns `ParseError` if the file cannot be read, is malformed,
/// or doesn't follow the action-lite format.
pub fn parse_action(path: &Path) -> ParseResult<Action> {
    // Read file contents
    let content = fs::read_to_string(path)
        .map_err(|e| ParseError::IoError(path.to_path_buf(), e))?;

    // Check for empty file
    if content.trim().is_empty() {
        return Err(ParseError::EmptyFile(path.to_path_buf()));
    }

    // Split into lines for parsing
    let lines: Vec<&str> = content.lines().collect();

    if lines.is_empty() {
        return Err(ParseError::EmptyFile(path.to_path_buf()));
    }

    // Parse first line for tags
    let (phase, priority, project_tags) = parse_tag_line(lines[0], path)?;

    // Extract title from first heading
    let title = extract_title(&lines, path)?;

    // Extract Statement of Inputs section
    let statement_of_inputs = extract_section(&lines, "Statement of Inputs");

    Ok(Action {
        path: path.to_path_buf(),
        title,
        phase,
        priority,
        project_tags,
        statement_of_inputs,
    })
}

/// Parse all action files in a list of paths
///
/// # Arguments
/// * `paths` - Vector of paths to action files
///
/// # Returns
/// A vector of successfully parsed `Action` objects
///
/// # Errors
/// Returns the first `ParseError` encountered. This ensures that
/// malformed files are not silently skipped.
pub fn parse_all_actions(paths: Vec<PathBuf>) -> ParseResult<Vec<Action>> {
    paths.iter()
        .map(|path| parse_action(path))
        .collect()
}

/// Parse the tag line (first line) to extract phase, priority, and project tags
fn parse_tag_line(line: &str, path: &Path) -> ParseResult<(Phase, bool, Vec<String>)> {
    let trimmed = line.trim();

    // Check if line contains any tags
    if !trimmed.starts_with('#') {
        return Err(ParseError::InvalidTagLine(path.to_path_buf()));
    }

    // Split by whitespace to get individual tags
    let tags: Vec<&str> = trimmed.split_whitespace().collect();

    if tags.is_empty() {
        return Err(ParseError::InvalidTagLine(path.to_path_buf()));
    }

    // Validate #action tag is present
    if !tags.contains(&"#action") {
        return Err(ParseError::MissingActionTag(path.to_path_buf()));
    }

    // Check for priority tag
    let priority = tags.contains(&"#priority");

    // Extract phase tags
    let phase_tags: Vec<&str> = tags.iter()
        .filter(|tag| Phase::from_tag(tag).is_some())
        .copied()
        .collect();

    // Validate exactly one phase tag
    if phase_tags.is_empty() {
        return Err(ParseError::MissingPhaseTag(path.to_path_buf()));
    }

    if phase_tags.len() > 1 {
        let phase_strings: Vec<String> = phase_tags.iter().map(|s| s.to_string()).collect();
        return Err(ParseError::MultiplePhaseTag(path.to_path_buf(), phase_strings));
    }

    let phase = Phase::from_tag(phase_tags[0]).unwrap(); // Safe because we validated above

    // Extract project tags (everything that's not #action, phase tag, or #priority)
    let project_tags: Vec<String> = tags.iter()
        .filter(|tag| {
            **tag != "#action"
            && **tag != "#priority"
            && Phase::from_tag(tag).is_none()
        })
        .map(|tag| {
            // Remove leading # for storage
            if tag.starts_with('#') {
                tag[1..].to_string()
            } else {
                tag.to_string()
            }
        })
        .collect();

    Ok((phase, priority, project_tags))
}

/// Extract the title from the first markdown heading
fn extract_title(lines: &[&str], path: &Path) -> ParseResult<String> {
    for line in lines {
        let trimmed = line.trim();

        // Look for markdown heading (starts with # followed by space)
        if trimmed.starts_with("# ") {
            let title = trimmed[2..].trim().to_string();
            if !title.is_empty() {
                return Ok(title);
            }
        }
    }

    Err(ParseError::MissingTitle(path.to_path_buf()))
}

/// Extract a section's content by heading name
///
/// Sections are delimited by markdown headings. This function finds the
/// heading matching `section_name` and returns all content until the next
/// heading of equal or higher level, or end of file.
fn extract_section(lines: &[&str], section_name: &str) -> Option<String> {
    let mut in_section = false;
    let mut section_content = Vec::new();
    let mut section_level = 0;

    for line in lines {
        let trimmed = line.trim();

        // Check if this is a heading
        if let Some(heading_info) = parse_heading(trimmed) {
            let (level, heading_text) = heading_info;

            if !in_section {
                // Check if this is the section we're looking for
                if heading_text == section_name {
                    in_section = true;
                    section_level = level;
                    continue; // Don't include the heading itself
                }
            } else {
                // We're in the section - check if we've hit another heading at same or higher level
                if level <= section_level {
                    break; // End of section
                }
            }
        }

        // Add line to section content if we're in the section
        if in_section {
            section_content.push(*line);
        }
    }

    if section_content.is_empty() {
        None
    } else {
        Some(section_content.join("\n").trim().to_string())
    }
}

/// Parse a markdown heading, returning (level, text) if it's a heading
///
/// Examples:
/// - "# Title" -> Some((1, "Title"))
/// - "## Subsection" -> Some((2, "Subsection"))
/// - "Not a heading" -> None
fn parse_heading(line: &str) -> Option<(usize, String)> {
    let trimmed = line.trim();

    if !trimmed.starts_with('#') {
        return None;
    }

    // Count the number of # characters
    let hash_count = trimmed.chars().take_while(|&c| c == '#').count();

    // Must have space after the hashes
    if trimmed.len() <= hash_count || !trimmed.chars().nth(hash_count).map_or(false, |c| c.is_whitespace()) {
        return None;
    }

    // Extract heading text
    let heading_text = trimmed[hash_count..].trim().to_string();

    Some((hash_count, heading_text))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    /// Helper to create a temporary test directory
    fn create_temp_dir(name: &str) -> PathBuf {
        let temp_base = std::env::temp_dir();
        let test_dir = temp_base.join(format!("action_lite_parser_test_{}", name));

        // Clean up if it exists from previous test
        if test_dir.exists() {
            fs::remove_dir_all(&test_dir).ok();
        }

        fs::create_dir_all(&test_dir).expect("Failed to create test directory");
        test_dir
    }

    /// Helper to create a file with content
    fn create_test_file(dir: &Path, name: &str, content: &str) -> PathBuf {
        let file_path = dir.join(name);
        let mut file = File::create(&file_path).expect("Failed to create test file");
        file.write_all(content.as_bytes()).expect("Failed to write test file");
        file_path
    }

    #[test]
    fn test_parse_valid_action() {
        let test_dir = create_temp_dir("valid_action");
        let content = r#"#action #design #action-lite #priority

# Test Action Title

Some content here.

# Statement of Inputs

This action depends on something.
"#;
        let file_path = create_test_file(&test_dir, "test.md", content);

        let result = parse_action(&file_path);
        assert!(result.is_ok(), "Should successfully parse valid action");

        let action = result.unwrap();
        assert_eq!(action.title, "Test Action Title");
        assert_eq!(action.phase, Phase::Design);
        assert!(action.priority);
        assert_eq!(action.project_tags, vec!["action-lite"]);
        assert!(action.statement_of_inputs.is_some());
        assert!(action.statement_of_inputs.unwrap().contains("depends on something"));

        // Cleanup
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_parse_without_priority() {
        let test_dir = create_temp_dir("no_priority");
        let content = r#"#action #implementation #rust

# Implementation Task
"#;
        let file_path = create_test_file(&test_dir, "test.md", content);

        let result = parse_action(&file_path);
        assert!(result.is_ok());

        let action = result.unwrap();
        assert_eq!(action.title, "Implementation Task");
        assert_eq!(action.phase, Phase::Implementation);
        assert!(!action.priority);
        assert_eq!(action.project_tags, vec!["rust"]);

        // Cleanup
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_parse_missing_action_tag() {
        let test_dir = create_temp_dir("missing_action");
        let content = r#"#design #priority

# Title
"#;
        let file_path = create_test_file(&test_dir, "test.md", content);

        let result = parse_action(&file_path);
        assert!(result.is_err());

        match result {
            Err(ParseError::MissingActionTag(_)) => {}
            _ => panic!("Expected MissingActionTag error"),
        }

        // Cleanup
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_parse_missing_phase_tag() {
        let test_dir = create_temp_dir("missing_phase");
        let content = r#"#action #priority

# Title
"#;
        let file_path = create_test_file(&test_dir, "test.md", content);

        let result = parse_action(&file_path);
        assert!(result.is_err());

        match result {
            Err(ParseError::MissingPhaseTag(_)) => {}
            _ => panic!("Expected MissingPhaseTag error"),
        }

        // Cleanup
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_parse_multiple_phase_tags() {
        let test_dir = create_temp_dir("multiple_phases");
        let content = r#"#action #design #implementation

# Title
"#;
        let file_path = create_test_file(&test_dir, "test.md", content);

        let result = parse_action(&file_path);
        assert!(result.is_err());

        match result {
            Err(ParseError::MultiplePhaseTag(_, phases)) => {
                assert_eq!(phases.len(), 2);
            }
            _ => panic!("Expected MultiplePhaseTag error"),
        }

        // Cleanup
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_parse_invalid_tag_line() {
        let test_dir = create_temp_dir("invalid_tags");
        let content = r#"This is not a tag line

# Title
"#;
        let file_path = create_test_file(&test_dir, "test.md", content);

        let result = parse_action(&file_path);
        assert!(result.is_err());

        match result {
            Err(ParseError::InvalidTagLine(_)) => {}
            _ => panic!("Expected InvalidTagLine error"),
        }

        // Cleanup
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_parse_missing_title() {
        let test_dir = create_temp_dir("missing_title");
        let content = r#"#action #design

Some content without a title heading.
"#;
        let file_path = create_test_file(&test_dir, "test.md", content);

        let result = parse_action(&file_path);
        assert!(result.is_err());

        match result {
            Err(ParseError::MissingTitle(_)) => {}
            _ => panic!("Expected MissingTitle error"),
        }

        // Cleanup
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_parse_empty_file() {
        let test_dir = create_temp_dir("empty");
        let content = "";
        let file_path = create_test_file(&test_dir, "test.md", content);

        let result = parse_action(&file_path);
        assert!(result.is_err());

        match result {
            Err(ParseError::EmptyFile(_)) => {}
            _ => panic!("Expected EmptyFile error"),
        }

        // Cleanup
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_parse_all_phases() {
        let phases = vec![
            ("#discovery", Phase::Discovery),
            ("#design", Phase::Design),
            ("#implementation", Phase::Implementation),
            ("#test", Phase::Test),
            ("#document", Phase::Document),
            ("#publish", Phase::Publish),
        ];

        for (tag, expected_phase) in phases {
            let test_dir = create_temp_dir(&format!("phase_{}", tag));
            let content = format!("#action {} #test-tag\n\n# Title\n", tag);
            let file_path = create_test_file(&test_dir, "test.md", &content);

            let result = parse_action(&file_path);
            assert!(result.is_ok());

            let action = result.unwrap();
            assert_eq!(action.phase, expected_phase);

            // Cleanup
            fs::remove_dir_all(&test_dir).ok();
        }
    }

    #[test]
    fn test_extract_section_present() {
        let test_dir = create_temp_dir("section_present");
        let content = r#"#action #design

# Title

# Statement of Inputs

This is the inputs section.
It has multiple lines.

# Another Section

This should not be included.
"#;
        let file_path = create_test_file(&test_dir, "test.md", content);

        let result = parse_action(&file_path);
        assert!(result.is_ok());

        let action = result.unwrap();
        assert!(action.statement_of_inputs.is_some());

        let inputs = action.statement_of_inputs.unwrap();
        assert!(inputs.contains("This is the inputs section"));
        assert!(inputs.contains("multiple lines"));
        assert!(!inputs.contains("Another Section"));

        // Cleanup
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_extract_section_missing() {
        let test_dir = create_temp_dir("section_missing");
        let content = r#"#action #design

# Title

# Some Other Section

Content here.
"#;
        let file_path = create_test_file(&test_dir, "test.md", content);

        let result = parse_action(&file_path);
        assert!(result.is_ok());

        let action = result.unwrap();
        assert!(action.statement_of_inputs.is_none());

        // Cleanup
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_parse_heading() {
        assert_eq!(parse_heading("# Title"), Some((1, "Title".to_string())));
        assert_eq!(parse_heading("## Subsection"), Some((2, "Subsection".to_string())));
        assert_eq!(parse_heading("### Deep"), Some((3, "Deep".to_string())));
        assert_eq!(parse_heading("# Title with spaces"), Some((1, "Title with spaces".to_string())));
        assert_eq!(parse_heading("  # Indented  "), Some((1, "Indented".to_string())));

        // Invalid headings
        assert_eq!(parse_heading("#NoSpace"), None);
        assert_eq!(parse_heading("Not a heading"), None);
        assert_eq!(parse_heading(""), None);
    }

    #[test]
    fn test_multiple_project_tags() {
        let test_dir = create_temp_dir("multi_tags");
        let content = r#"#action #design #rust #cli #priority #backend

# Title
"#;
        let file_path = create_test_file(&test_dir, "test.md", content);

        let result = parse_action(&file_path);
        assert!(result.is_ok());

        let action = result.unwrap();
        assert_eq!(action.project_tags.len(), 3);
        assert!(action.project_tags.contains(&"rust".to_string()));
        assert!(action.project_tags.contains(&"cli".to_string()));
        assert!(action.project_tags.contains(&"backend".to_string()));
        assert!(action.priority);

        // Cleanup
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_title_with_special_characters() {
        let test_dir = create_temp_dir("special_chars");
        let content = r#"#action #design

# Title with "quotes" and symbols: #$%!

Content
"#;
        let file_path = create_test_file(&test_dir, "test.md", content);

        let result = parse_action(&file_path);
        assert!(result.is_ok());

        let action = result.unwrap();
        assert_eq!(action.title, r#"Title with "quotes" and symbols: #$%!"#);

        // Cleanup
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_parse_all_actions_success() {
        let test_dir = create_temp_dir("parse_all");

        let content1 = "#action #design\n\n# Action One\n";
        let content2 = "#action #test #priority\n\n# Action Two\n";

        let file1 = create_test_file(&test_dir, "action1.md", content1);
        let file2 = create_test_file(&test_dir, "action2.md", content2);

        let paths = vec![file1, file2];
        let result = parse_all_actions(paths);

        assert!(result.is_ok());
        let actions = result.unwrap();
        assert_eq!(actions.len(), 2);

        // Cleanup
        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_parse_all_actions_fails_on_malformed() {
        let test_dir = create_temp_dir("parse_all_fail");

        let content1 = "#action #design\n\n# Valid Action\n";
        let content2 = "Not a valid action\n\n# Title\n"; // Missing tags

        let file1 = create_test_file(&test_dir, "action1.md", content1);
        let file2 = create_test_file(&test_dir, "action2.md", content2);

        let paths = vec![file1, file2];
        let result = parse_all_actions(paths);

        // Should fail on the malformed file
        assert!(result.is_err());

        // Cleanup
        fs::remove_dir_all(&test_dir).ok();
    }
}
