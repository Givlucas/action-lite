// src/main.rs
//! action-lite CLI tool
//!
//! Command-line interface for visualizing and managing action-lite
//! methodology workflows.

use std::env;
use std::process::ExitCode;

mod commands;
mod scanner;
// Other modules will be added as they're implemented

use commands::Command;

fn main() -> ExitCode {
    // Parse command-line arguments
    let command = match parse_args() {
        Ok(args) => args.command,
        Err(error) => {
            eprintln!("Error: {}", error);
            eprintln!();
            commands::help::print_usage();
            return ExitCode::FAILURE;
        }
    };

    // Execute the command
    match execute_command(command) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {}", error);
            ExitCode::FAILURE
        }
    }
}

/// Parsed command-line arguments
#[derive(Debug)]
struct Args {
    command: Command,
}

/// Parse command-line arguments
fn parse_args() -> Result<Args, String> {
    let mut args = env::args().skip(1); // Skip program name

    // If no arguments, default to help
    let subcommand = match args.next() {
        Some(cmd) => cmd,
        None => return Ok(Args { command: Command::Help }),
    };

    // Parse the subcommand
    let command = match subcommand.to_lowercase().as_str() {
        "list" => Command::List,
        "graph" => Command::Graph,
        "help" | "-h" | "--help" => Command::Help,
        unknown => {
            return Err(format!("Unknown command: '{}'\n\nValid commands: list, graph, help", unknown));
        }
    };

    // Check for extra arguments (not supported in this version)
    if let Some(extra) = args.next() {
        return Err(format!("Unexpected argument: '{}'\n\nCommands take no additional arguments", extra));
    }

    Ok(Args { command })
}

/// Execute the parsed command
fn execute_command(command: Command) -> Result<(), String> {
    match command {
        Command::Help => {
            commands::help::print_help();
            Ok(())
        }
        Command::List => {
            commands::list::execute()
                .map_err(|e| format!("List command failed: {}", e))
        }
        Command::Graph => {
            commands::graph::execute()
                .map_err(|e| format!("Graph command failed: {}", e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_args_from_vec(args: Vec<String>) -> Result<Args, String> {
        let mut args_iter = args.into_iter().skip(1); // Skip program name

        // If no arguments, default to help
        let subcommand = match args_iter.next() {
            Some(cmd) => cmd,
            None => return Ok(Args { command: Command::Help }),
        };

        // Parse the subcommand
        let command = match subcommand.to_lowercase().as_str() {
            "list" => Command::List,
            "graph" => Command::Graph,
            "help" | "-h" | "--help" => Command::Help,
            unknown => {
                return Err(format!("Unknown command: '{}'\n\nValid commands: list, graph, help", unknown));
            }
        };

        // Check for extra arguments (not supported in this version)
        if let Some(extra) = args_iter.next() {
            return Err(format!("Unexpected argument: '{}'\n\nCommands take no additional arguments", extra));
        }

        Ok(Args { command })
    }

    #[test]
    fn test_parse_list_command() {
        let result = parse_args_from_vec(vec!["action-lite".to_string(), "list".to_string()]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().command, Command::List);
    }

    #[test]
    fn test_parse_graph_command() {
        let result = parse_args_from_vec(vec!["action-lite".to_string(), "graph".to_string()]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().command, Command::Graph);
    }

    #[test]
    fn test_parse_help_command_variants() {
        // Test "help"
        let result = parse_args_from_vec(vec!["action-lite".to_string(), "help".to_string()]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().command, Command::Help);

        // Test "-h"
        let result = parse_args_from_vec(vec!["action-lite".to_string(), "-h".to_string()]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().command, Command::Help);

        // Test "--help"
        let result = parse_args_from_vec(vec!["action-lite".to_string(), "--help".to_string()]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().command, Command::Help);
    }

    #[test]
    fn test_no_arguments_defaults_to_help() {
        let result = parse_args_from_vec(vec!["action-lite".to_string()]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().command, Command::Help);
    }

    #[test]
    fn test_unknown_command() {
        let result = parse_args_from_vec(vec!["action-lite".to_string(), "unknown".to_string()]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown command: 'unknown'"));
    }

    #[test]
    fn test_extra_arguments_rejected() {
        let result = parse_args_from_vec(vec!["action-lite".to_string(), "list".to_string(), "extra".to_string()]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unexpected argument: 'extra'"));
    }

    #[test]
    fn test_case_insensitivity() {
        // Test "LIST"
        let result = parse_args_from_vec(vec!["action-lite".to_string(), "LIST".to_string()]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().command, Command::List);

        // Test "GrApH"
        let result = parse_args_from_vec(vec!["action-lite".to_string(), "GrApH".to_string()]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().command, Command::Graph);

        // Test "HeLp"
        let result = parse_args_from_vec(vec!["action-lite".to_string(), "HeLp".to_string()]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().command, Command::Help);
    }
}
