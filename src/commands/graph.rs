// src/commands/graph.rs
//! Graph command implementation

use super::{CommandResult, CommandError};
use crate::scanner;
use crate::parser;
use crate::dependency;
use crate::visualizer;
use std::path::Path;

/// Execute the graph command
///
/// Scans the actions/ directory, parses all action files,
/// builds the dependency graph, and outputs visualization to stdout.
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
    let actions: Result<Vec<_>, _> = action_files
        .iter()
        .map(|path| parser::parse_action(path))
        .collect();
    let actions = actions.map_err(CommandError::ParseError)?;

    // 4. Build dependency graph
    let graph = dependency::build_dependency_graph(actions)
        .map_err(CommandError::DependencyError)?;

    // 5. Visualize and output
    visualizer::render_graph(graph);

    Ok(())
}
