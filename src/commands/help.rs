// src/commands/help.rs
//! Help text and usage information

/// Print full help text
pub fn print_help() {
    println!("action-lite {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("DESCRIPTION:");
    println!("  Command-line tool for visualizing and managing action-lite methodology");
    println!("  workflows. Helps developers understand action dependencies and priorities.");
    println!();
    println!("USAGE:");
    println!("  action-lite <COMMAND>");
    println!();
    println!("COMMANDS:");
    println!("  list     Display all priority actions (tagged with #priority)");
    println!("  graph    Display dependency graph of all actions");
    println!("  help     Display this help information");
    println!();
    println!("EXAMPLES:");
    println!("  action-lite list          # Show all priority actions");
    println!("  action-lite graph         # Show full dependency graph");
    println!();
    println!("NOTES:");
    println!("  - Must be run from a directory containing an 'actions/' folder");
    println!("  - Action files must follow the action-lite markdown format");
    println!("  - All output is written to stdout");
}

/// Print brief usage information (for errors)
pub fn print_usage() {
    println!("Usage: action-lite <COMMAND>");
    println!();
    println!("Commands: list, graph, help");
    println!();
    println!("Run 'action-lite help' for more information.");
}
