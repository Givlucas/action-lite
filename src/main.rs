use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

use action_lite::{Workspace, Status};

#[derive(Parser)]
#[command(name = "action")]
#[command(about = "Action Lite - A file-based task tracking system")]
#[command(version = "0.1.0")]
struct Cli {
    /// Action Lite workspace directory
    #[arg(short, long, value_name = "DIR")]
    workspace: Option<PathBuf>,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Action Lite workspace
    Init {
        /// Directory to initialize (defaults to current directory)
        path: Option<PathBuf>,
    },
    /// Create a new action
    New {
        /// Project name
        project: String,
        /// Action title
        title: String,
        /// Set priority flag
        #[arg(short, long)]
        priority: bool,
    },
    /// List actions
    List {
        /// Filter by project
        #[arg(short, long)]
        project: Option<String>,
        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,
        /// Show only priority actions
        #[arg(long)]
        priority: bool,
    },
    /// Show action details
    Show {
        /// Project name
        project: String,
        /// Action title
        title: String,
    },
    /// Update action status
    Status {
        /// Project name
        project: String,
        /// Action title
        title: String,
        /// New status
        status: String,
    },
    /// Set or unset priority flag
    Priority {
        /// Project name
        project: String,
        /// Action title
        title: String,
        /// Set priority (true) or remove priority (false)
        #[arg(short, long)]
        set: bool,
    },
    /// Edit an action in your default editor
    Edit {
        /// Project name
        project: String,
        /// Action title
        title: String,
    },
    /// Create a meta-graph for an action
    MetaGraph {
        /// Project name
        project: String,
        /// Action title
        title: String,
    },
    /// Validate workspace structure and files
    Validate,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let workspace_path = cli.workspace.unwrap_or_else(|| {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    });
    
    match cli.command {
        Commands::Init { path } => {
            let init_path = path.unwrap_or(workspace_path);
            let _workspace = Workspace::init(&init_path)?;
            println!("{} Action Lite workspace initialized at: {}", 
                "✓".green(), 
                init_path.display().to_string().cyan()
            );
            Ok(())
        }
        
        Commands::New { project, title, priority } => {
            let mut workspace = Workspace::load(&workspace_path)?;
            let _action = workspace.create_action(&project, &title, priority)?;
            println!("{} Created action: {} in project {}", 
                "✓".green(), 
                title.cyan(), 
                project.yellow()
            );
            if priority {
                println!("  {} Priority action", "!".red());
            }
            Ok(())
        }
        
        Commands::List { project, status, priority } => {
            let workspace = Workspace::load(&workspace_path)?;
            let actions = workspace.list_actions(project.as_deref(), status.as_deref(), priority)?;
            
            if actions.is_empty() {
                println!("{} No actions found matching criteria", "ℹ".blue());
                return Ok(());
            }
            
            for action in actions {
                let priority_marker = if action.is_priority() { "!" } else { " " };
                let status_color = match action.status() {
                    Status::Discovery => "yellow",
                    Status::Design => "blue", 
                    Status::Implement => "magenta",
                    Status::Test => "cyan",
                    Status::Document => "green",
                    Status::Publish => "bright_green",
                    Status::Published => "bright_blue",
                };
                
                println!("{} [{}] {}/{} - {}", 
                    priority_marker.red(),
                    action.status().to_string().color(status_color),
                    action.project().yellow(),
                    action.title().cyan(),
                    action.statement_of_action().map_or("No description", |s| s.as_str()).dimmed()
                );
            }
            Ok(())
        }
        
        Commands::Show { project, title } => {
            let workspace = Workspace::load(&workspace_path)?;
            let action = workspace.get_action(&project, &title)?;
            
            println!("\n{} {}/{}", "Action:".bold(), project.yellow(), title.cyan());
            println!("{} {}", "Status:".bold(), action.status().to_string().green());
            if action.is_priority() {
                println!("{} {}", "Priority:".bold(), "HIGH".red());
            }
            
            if let Some(notes) = action.notes() {
                println!("\n{}:\n{}", "Notes".bold(), notes);
            }
            
            if let Some(statement) = action.statement_of_action() {
                println!("\n{}:\n{}", "Statement of Action".bold(), statement);
            }
            
            if let Some(inputs) = action.statement_of_inputs() {
                println!("\n{}:\n{}", "Statement of Inputs".bold(), inputs);
            }
            
            if let Some(design) = action.statement_of_design() {
                println!("\n{}:\n{}", "Statement of Design".bold(), design);
            }
            
            if let Some(impact) = action.analysis_of_impact() {
                println!("\n{}:\n{}", "Analysis of Impact".bold(), impact);
            }
            
            Ok(())
        }
        
        Commands::Status { project, title, status } => {
            let mut workspace = Workspace::load(&workspace_path)?;
            let new_status = Status::from_str(&status)?;
            workspace.update_action_status(&project, &title, new_status)?;
            println!("{} Updated status of {}/{} to {}", 
                "✓".green(), 
                project.yellow(), 
                title.cyan(), 
                status.green()
            );
            Ok(())
        }
        
        Commands::Priority { project, title, set } => {
            let mut workspace = Workspace::load(&workspace_path)?;
            workspace.set_action_priority(&project, &title, set)?;
            let action_desc = format!("{}/{}", project.yellow(), title.cyan());
            if set {
                println!("{} Set priority for {}", "✓".green(), action_desc);
            } else {
                println!("{} Removed priority from {}", "✓".green(), action_desc);
            }
            Ok(())
        }
        
        Commands::Edit { project, title } => {
            let workspace = Workspace::load(&workspace_path)?;
            workspace.edit_action(&project, &title)?;
            Ok(())
        }
        
        Commands::MetaGraph { project, title } => {
            let workspace = Workspace::load(&workspace_path)?;
            let _meta_graph_path = workspace.create_meta_graph(&project, &title)?;
            println!("{} Created meta-graph directory for {}/{}", 
                "✓".green(), 
                project.yellow(), 
                title.cyan()
            );
            Ok(())
        }
        
        Commands::Validate => {
            let workspace = Workspace::load(&workspace_path)?;
            match workspace.validate() {
                Ok(()) => {
                    println!("{} Workspace validation passed", "✓".green());
                }
                Err(e) => {
                    println!("{} Workspace validation failed: {}", "✗".red(), e);
                    std::process::exit(1);
                }
            }
            Ok(())
        }
    }
}