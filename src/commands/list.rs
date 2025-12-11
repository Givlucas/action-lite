// src/commands/list.rs
//! List command implementation

use super::{CommandResult, CommandError};
use crate::scanner;
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

    // 3. Parse action files (requires parser module - separate action)
    // let actions = parse_all_actions(action_files)?;

    // 4. Filter for priority (requires parser module)
    // let priority_actions = filter_priority(actions);

    // 5. Format and output (requires formatter module - separate action)
    // formatters::list::format_and_print(priority_actions);

    // Placeholder for now
    println!("List command - implementation pending parser and formatter modules");
    println!("Found {} action files", action_files.len());

    Ok(())
}
