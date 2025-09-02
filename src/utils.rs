use anyhow::Result;
use std::path::Path;

/// Convert a title to a valid filename
pub fn title_to_filename(title: &str) -> String {
    title
        .chars()
        .map(|c| match c {
            ' ' => '_',
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '-',
            c if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' => c,
            _ => '_',
        })
        .collect::<String>()
        .to_lowercase()
}

/// Validate that a project name is valid
pub fn validate_project_name(name: &str) -> Result<()> {
    if name.is_empty() {
        anyhow::bail!("Project name cannot be empty");
    }
    
    if name.contains('/') || name.contains('\\') {
        anyhow::bail!("Project name cannot contain path separators");
    }
    
    if name.starts_with('.') {
        anyhow::bail!("Project name cannot start with a dot");
    }
    
    Ok(())
}

/// Validate that an action title is valid
pub fn validate_action_title(title: &str) -> Result<()> {
    if title.is_empty() {
        anyhow::bail!("Action title cannot be empty");
    }
    
    if title.len() > 100 {
        anyhow::bail!("Action title cannot be longer than 100 characters");
    }
    
    Ok(())
}

/// Check if a path is within another path (for security)
pub fn is_subpath<P: AsRef<Path>, Q: AsRef<Path>>(path: P, parent: Q) -> bool {
    let path = path.as_ref();
    let parent = parent.as_ref();
    
    if let (Ok(path), Ok(parent)) = (path.canonicalize(), parent.canonicalize()) {
        path.starts_with(parent)
    } else {
        false
    }
}

/// Format a relative path for display
pub fn format_relative_path<P: AsRef<Path>, Q: AsRef<Path>>(path: P, base: Q) -> String {
    let path = path.as_ref();
    let base = base.as_ref();
    
    if let Ok(relative) = path.strip_prefix(base) {
        relative.display().to_string()
    } else {
        path.display().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_title_to_filename() {
        assert_eq!(title_to_filename("Simple Title"), "simple_title");
        assert_eq!(title_to_filename("Complex: Title/With*Special?Chars"), "complex-_title-with-special-chars");
        assert_eq!(title_to_filename("Unicode: 你好"), "unicode-_你好");
    }
    
    #[test]
    fn test_validate_project_name() {
        assert!(validate_project_name("valid-project").is_ok());
        assert!(validate_project_name("ValidProject123").is_ok());
        assert!(validate_project_name("").is_err());
        assert!(validate_project_name("project/with/slash").is_err());
        assert!(validate_project_name(".hidden").is_err());
    }
    
    #[test]
    fn test_validate_action_title() {
        assert!(validate_action_title("Valid Title").is_ok());
        assert!(validate_action_title("").is_err());
        assert!(validate_action_title(&"x".repeat(101)).is_err());
    }
}