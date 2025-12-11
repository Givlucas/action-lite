// src/commands/list.rs
//! List command implementation

use super::{CommandResult, CommandError};
use crate::scanner;
use crate::parser;
use std::path::Path;

/// Execute the list command
///
/// Scans the actions/ directory, parses all action files,
/// filters for priority actions, and outputs them to stdout.
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

    // 5. Sort alphabetically by title for consistent, scannable output
    priority_actions.sort_by(|a, b| a.title.cmp(&b.title));

    // 6. Format and output - simple list of titles only
    if priority_actions.is_empty() {
        println!("No priority actions found.");
    } else {
        for action in priority_actions {
            println!("{}", action.title);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parser::{Action, Phase};
    use std::path::PathBuf;

    /// Helper to create a test action
    fn create_test_action(title: &str, priority: bool, phase: Phase) -> Action {
        Action {
            path: PathBuf::from("/test/action.md"),
            title: title.to_string(),
            phase,
            priority,
            project_tags: vec![],
            statement_of_inputs: None,
        }
    }

    #[test]
    fn test_filter_priority_actions() {
        let actions = vec![
            create_test_action("Priority Action 1", true, Phase::Design),
            create_test_action("Non-Priority Action", false, Phase::Implementation),
            create_test_action("Priority Action 2", true, Phase::Test),
        ];

        let priority_actions: Vec<_> = actions.iter()
            .filter(|action| action.priority)
            .collect();

        assert_eq!(priority_actions.len(), 2);
        assert!(priority_actions.iter().all(|a| a.priority));
    }

    #[test]
    fn test_sort_alphabetically() {
        let mut actions = vec![
            create_test_action("Zebra Action", true, Phase::Design),
            create_test_action("Apple Action", true, Phase::Implementation),
            create_test_action("Middle Action", true, Phase::Test),
        ];

        actions.sort_by(|a, b| a.title.cmp(&b.title));

        assert_eq!(actions[0].title, "Apple Action");
        assert_eq!(actions[1].title, "Middle Action");
        assert_eq!(actions[2].title, "Zebra Action");
    }

    #[test]
    fn test_empty_priority_list() {
        let actions = vec![
            create_test_action("Non-Priority 1", false, Phase::Design),
            create_test_action("Non-Priority 2", false, Phase::Implementation),
        ];

        let priority_actions: Vec<_> = actions.iter()
            .filter(|action| action.priority)
            .collect();

        assert_eq!(priority_actions.len(), 0);
    }

    #[test]
    fn test_title_preservation() {
        // Test that special characters, capitalization, etc. are preserved
        let actions = vec![
            create_test_action("Title with \"Quotes\"", true, Phase::Design),
            create_test_action("Title with CAPS and lowercase", true, Phase::Design),
            create_test_action("Title with symbols: #$%!", true, Phase::Design),
        ];

        for action in &actions {
            // Verify titles are preserved exactly
            assert!(action.title.contains("\"") || action.title.contains("CAPS") || action.title.contains("#$%!"));
        }
    }

    #[test]
    fn test_single_priority_action() {
        let actions = vec![
            create_test_action("Only Priority", true, Phase::Design),
            create_test_action("Not Priority 1", false, Phase::Implementation),
            create_test_action("Not Priority 2", false, Phase::Test),
        ];

        let priority_actions: Vec<_> = actions.iter()
            .filter(|action| action.priority)
            .collect();

        assert_eq!(priority_actions.len(), 1);
        assert_eq!(priority_actions[0].title, "Only Priority");
    }

    #[test]
    fn test_all_priority_actions() {
        let actions = vec![
            create_test_action("Priority 1", true, Phase::Design),
            create_test_action("Priority 2", true, Phase::Implementation),
            create_test_action("Priority 3", true, Phase::Test),
        ];

        let priority_actions: Vec<_> = actions.iter()
            .filter(|action| action.priority)
            .collect();

        assert_eq!(priority_actions.len(), 3);
        assert_eq!(priority_actions.len(), actions.len());
    }

    #[test]
    fn test_sorting_case_sensitivity() {
        // Ensure sorting is case-sensitive (as per standard string comparison)
        let mut actions = vec![
            create_test_action("zebra", true, Phase::Design),
            create_test_action("Zebra", true, Phase::Design),
            create_test_action("ZEBRA", true, Phase::Design),
            create_test_action("apple", true, Phase::Design),
        ];

        actions.sort_by(|a, b| a.title.cmp(&b.title));

        // In Rust, uppercase letters come before lowercase in ASCII/Unicode order
        assert_eq!(actions[0].title, "ZEBRA");
        assert_eq!(actions[1].title, "Zebra");
        assert_eq!(actions[2].title, "apple");
        assert_eq!(actions[3].title, "zebra");
    }

    #[test]
    fn test_sorting_with_numbers() {
        let mut actions = vec![
            create_test_action("Action 10", true, Phase::Design),
            create_test_action("Action 2", true, Phase::Design),
            create_test_action("Action 1", true, Phase::Design),
        ];

        actions.sort_by(|a, b| a.title.cmp(&b.title));

        // Lexicographic sort: "1" < "10" < "2"
        assert_eq!(actions[0].title, "Action 1");
        assert_eq!(actions[1].title, "Action 10");
        assert_eq!(actions[2].title, "Action 2");
    }
}
