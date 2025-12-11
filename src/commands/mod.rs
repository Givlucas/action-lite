// src/commands/mod.rs
//! Command definitions and execution

pub mod help;
pub mod list;
pub mod graph;

use std::fmt;

/// Available commands
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Command {
    List,
    Graph,
    Help,
}

/// Result type for command operations
pub type CommandResult = Result<(), CommandError>;

/// Errors during command execution
#[derive(Debug)]
pub enum CommandError {
    /// File system scanning error
    ScanError(crate::scanner::ScanError),
    /// Action parsing error
    #[allow(dead_code)]
    ParseError(String),
    /// Graph building error
    #[allow(dead_code)]
    GraphError(String),
    /// Actions directory not found
    NoActionsDirectory,
    /// Generic execution error
    #[allow(dead_code)]
    ExecutionError(String),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::ScanError(e) => write!(f, "Scan error: {:?}", e),
            CommandError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CommandError::GraphError(msg) => write!(f, "Graph error: {}", msg),
            CommandError::NoActionsDirectory => write!(f, "No 'actions/' directory found in current location"),
            CommandError::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
        }
    }
}

impl std::error::Error for CommandError {}
