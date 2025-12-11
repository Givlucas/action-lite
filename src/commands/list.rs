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
    let priority_actions: Vec<_> = actions.iter()
        .filter(|action| action.priority)
        .collect();

    // 5. Format and output (simple format for now - formatter module separate action)
    if priority_actions.is_empty() {
        println!("No priority actions found.");
    } else {
        println!("Priority Actions ({}):\n", priority_actions.len());
        for action in priority_actions {
            println!("  {} [{}]", action.title, action.phase);
            println!("    Path: {}", action.path.display());
            if !action.project_tags.is_empty() {
                println!("    Tags: {}", action.project_tags.join(", "));
            }
            println!();
        }
    }

    Ok(())
}
