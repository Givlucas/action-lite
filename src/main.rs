use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "action-lite")]
#[command(version = "0.1.0")]
#[command(about = "A file-based task tracking system using acyclic directed meta graphs")]
#[command(long_about = "Action Lite helps you manage tasks and their dependencies using markdown files.\n\
                         Each action is tracked through states: discovery, design, implementation, test, document, publish, published.\n\
                         \n\
                         Example usage:\n  \
                         action-lite list\n  \
                         action-lite graph\n  \
                         action-lite priority")]
struct Cli {
    #[arg(short, long, default_value = ".", help = "Path to the actions directory")]
    path: PathBuf,

    #[arg(long, global = true, help = "Show published actions (hidden by default)")]
    show_published: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "List all actions and their status")]
    List,

    #[command(about = "List priority actions")]
    Priority,

    #[command(about = "List actions by status")]
    Status {
        #[arg(help = "Status to filter by: discovery, design, implementation, test, document, publish, published")]
        status: String,
    },

    #[command(about = "Create a new empty action")]
    New {
        #[arg(help = "Name of the new action")]
        name: String,

        #[arg(short, long, help = "Project name")]
        project: String,
    },

    #[command(about = "Graph the actions in terminal")]
    Graph,

    #[command(about = "Move an action and update all references")]
    Move {
        #[arg(help = "Source path (relative to action directory)")]
        from: String,

        #[arg(help = "Destination path (relative to action directory)")]
        to: String,
    },
}

#[derive(Debug, Clone)]
struct Action {
    path: PathBuf,
    name: String,
    status: String,
    project: String,
    is_priority: bool,
    inputs: Vec<String>,
}

impl Action {
    fn from_file(path: &Path, base_path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .context(format!("Failed to read file: {}", path.display()))?;

        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Extract tags (including hyphens in tag names)
        let tag_regex = Regex::new(r"#([\w-]+)").unwrap();
        let tags: Vec<String> = tag_regex
            .captures_iter(&content)
            .map(|cap| cap[1].to_string())
            .collect();

        // Determine status (looking for state tags)
        let valid_states = ["discovery", "design", "implementation", "test", "document", "publish", "published"];
        let status = tags
            .iter()
            .find(|t| valid_states.contains(&t.as_str()))
            .cloned()
            .unwrap_or_else(|| "discovery".to_string());

        // Check for priority
        let is_priority = tags.contains(&"priority".to_string());

        // Find project tag (anything that's not action, priority, or a status)
        let project = tags
            .iter()
            .find(|t| {
                t.as_str() != "action"
                    && t.as_str() != "priority"
                    && !valid_states.contains(&t.as_str())
            })
            .cloned()
            .unwrap_or_else(|| "default".to_string());

        // Extract inputs from Statement of Inputs section
        let inputs = Self::extract_inputs(&content, base_path);

        Ok(Action {
            path: path.to_path_buf(),
            name,
            status,
            project,
            is_priority,
            inputs,
        })
    }

    fn extract_inputs(content: &str, base_path: &Path) -> Vec<String> {
        let mut inputs = Vec::new();

        // Look for Statement of inputs section (case insensitive, handles both "inputs" and "specifications")
        let section_regex = Regex::new(r"(?mi)^# Statement of (inputs|specifications)\s*$").unwrap();
        // Match wiki-links: [[Page Name]] or [[page-name.md]]
        let wiki_link_regex = Regex::new(r"\[\[([^\]]+)\]\]").unwrap();
        // Also support markdown links: [text](path) or [text](path.md)
        let md_link_regex = Regex::new(r"\[([^\]]+)\]\(([^\)]+)\)").unwrap();

        if let Some(section_match) = section_regex.find(content) {
            let section_start = section_match.end();

            // Find the next section or end of file
            let next_section_regex = Regex::new(r"(?m)^# ").unwrap();
            let section_end = next_section_regex
                .find_at(content, section_start + 1)
                .map(|m| m.start())
                .unwrap_or(content.len());

            let section_content = &content[section_start..section_end];

            // Extract wiki-links first
            for cap in wiki_link_regex.captures_iter(section_content) {
                if let Some(link) = cap.get(1) {
                    let link_str = link.as_str().trim();
                    // Normalize: ensure .md extension is present
                    let normalized = if link_str.ends_with(".md") {
                        link_str.to_string()
                    } else {
                        format!("{}.md", link_str)
                    };
                    inputs.push(normalized);
                }
            }

            // Also extract markdown-style links
            for cap in md_link_regex.captures_iter(section_content) {
                if let Some(link) = cap.get(2) {
                    let link_str = link.as_str().trim();
                    // Normalize: ensure .md extension is present
                    let normalized = if link_str.ends_with(".md") {
                        link_str.to_string()
                    } else {
                        format!("{}.md", link_str)
                    };
                    inputs.push(normalized);
                }
            }
        }

        inputs
    }

    fn display(&self, show_path: bool) {
        let status_colored = match self.status.as_str() {
            "discovery" => self.status.blue(),
            "design" => self.status.cyan(),
            "implementation" => self.status.yellow(),
            "test" => self.status.magenta(),
            "document" => self.status.green(),
            "publish" => self.status.bright_green(),
            "published" => self.status.bright_blue(),
            _ => self.status.white(),
        };

        let priority_marker = if self.is_priority {
            " [PRIORITY]".red().bold()
        } else {
            "".normal()
        };

        let project_colored = format!("[{}]", self.project).bright_black();

        if show_path {
            println!(
                "{} {} {}{}",
                status_colored.bold(),
                self.name.bright_white().bold(),
                project_colored,
                priority_marker
            );
            println!("  {}", self.path.display().to_string().bright_black());
        } else {
            println!(
                "{} {} {}{}",
                status_colored.bold(),
                self.name.bright_white().bold(),
                project_colored,
                priority_marker
            );
        }
    }
}

fn find_actions(base_path: &Path) -> Result<Vec<Action>> {
    let mut actions = Vec::new();

    for entry in WalkDir::new(base_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            // Check if file contains #action tag
            if let Ok(content) = fs::read_to_string(path) {
                if content.contains("#action") {
                    if let Ok(action) = Action::from_file(path, base_path) {
                        actions.push(action);
                    }
                }
            }
        }
    }

    // Sort actions by name for consistent output
    actions.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(actions)
}

fn list_actions(base_path: &Path, show_published: bool) -> Result<()> {
    let mut actions = find_actions(base_path)?;

    // Filter out published actions unless flag is set
    if !show_published {
        actions.retain(|a| a.status != "published");
    }

    if actions.is_empty() {
        if show_published {
            println!("{}", "No actions found.".yellow());
        } else {
            println!("{}", "No active actions found. Use --show-published to see published actions.".yellow());
        }
        return Ok(());
    }

    println!("{}", "Actions:".bright_white().bold().underline());
    println!();

    for action in &actions {
        action.display(true);
        println!();
    }

    println!("{}", format!("Total: {} actions", actions.len()).bright_black());

    Ok(())
}

fn list_priority(base_path: &Path, show_published: bool) -> Result<()> {
    let actions = find_actions(base_path)?;
    let priority_actions: Vec<_> = actions
        .iter()
        .filter(|a| {
            a.is_priority && (show_published || a.status != "published")
        })
        .collect();

    if priority_actions.is_empty() {
        if show_published {
            println!("{}", "No priority actions found.".yellow());
        } else {
            println!("{}", "No active priority actions found. Use --show-published to see published priorities.".yellow());
        }
        return Ok(());
    }

    println!("{}", "Priority Actions:".red().bold().underline());
    println!();

    for action in &priority_actions {
        action.display(true);
        println!();
    }

    println!("{}", format!("Total: {} priority actions", priority_actions.len()).bright_black());

    Ok(())
}

fn list_by_status(base_path: &Path, status: &str) -> Result<()> {
    // Validate status
    let valid_states = ["discovery", "design", "implementation", "test", "document", "publish", "published"];
    if !valid_states.contains(&status) {
        anyhow::bail!(
            "Invalid status '{}'. Valid statuses are: {}",
            status,
            valid_states.join(", ")
        );
    }

    let actions = find_actions(base_path)?;
    let filtered_actions: Vec<_> = actions.iter().filter(|a| a.status == status).collect();

    if filtered_actions.is_empty() {
        println!("{}", format!("No actions found with status '{}'.", status).yellow());
        return Ok(());
    }

    println!("{}", format!("Actions with status '{}':", status).bright_white().bold().underline());
    println!();

    for action in &filtered_actions {
        action.display(true);
        println!();
    }

    println!("{}", format!("Total: {} actions", filtered_actions.len()).bright_black());

    Ok(())
}

fn create_action(base_path: &Path, name: &str, project: &str) -> Result<()> {
    // Validate action name (no path separators)
    if name.contains('/') || name.contains('\\') {
        anyhow::bail!("Action name cannot contain path separators. Use 'move' command to organize actions in subdirectories.");
    }

    if name.is_empty() {
        anyhow::bail!("Action name cannot be empty");
    }

    // Validate project name (must be a valid tag - alphanumeric, hyphens, underscores)
    if project.is_empty() {
        anyhow::bail!("Project name cannot be empty");
    }

    let valid_project_regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    if !valid_project_regex.is_match(project) {
        anyhow::bail!("Project name must contain only letters, numbers, hyphens, and underscores");
    }

    let file_name = format!("{}.md", name);
    let file_path = base_path.join(&file_name);

    if file_path.exists() {
        anyhow::bail!("Action '{}' already exists at {}", name, file_path.display());
    }

    let template = format!(
        r#"# {}

#action #discovery #{}

# Notes

# Statement of Action

# Statement of Inputs

# Statement of Design

## Output

### Design

# Analysis of Impact
"#,
        name, project
    );

    fs::write(&file_path, template)
        .context(format!("Failed to create action file: {}", file_path.display()))?;

    println!("{}", format!("Created action: {}", file_path.display()).green());

    Ok(())
}

fn graph_actions(base_path: &Path, show_published: bool) -> Result<()> {
    let mut actions = find_actions(base_path)?;

    // Filter out published actions unless flag is set
    if !show_published {
        actions.retain(|a| a.status != "published");
    }

    if actions.is_empty() {
        if show_published {
            println!("{}", "No actions found.".yellow());
        } else {
            println!("{}", "No active actions found. Use --show-published to see published actions.".yellow());
        }
        return Ok(());
    }

    println!("{}", "Action Dependency Graph:".bright_white().bold().underline());
    println!();

    // Build a map of action names to actions
    let mut action_map: HashMap<String, &Action> = HashMap::new();
    for action in &actions {
        action_map.insert(action.name.clone(), action);
    }

    // Track which actions we've already displayed
    let mut displayed = HashSet::new();

    // Find root actions (those with no inputs or whose inputs are not in the action set)
    let mut roots = Vec::new();
    for action in &actions {
        let has_internal_inputs = action.inputs.iter().any(|input| {
            let input_name = Path::new(input)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            action_map.contains_key(input_name)
        });

        if !has_internal_inputs {
            roots.push(action);
        }
    }

    // Display root actions first
    for root in &roots {
        display_action_tree(root, &action_map, &mut displayed, 0);
    }

    // Display any remaining actions that weren't reached from roots
    for action in &actions {
        if !displayed.contains(&action.name) {
            display_action_tree(action, &action_map, &mut displayed, 0);
        }
    }

    Ok(())
}

fn display_action_tree(
    action: &Action,
    action_map: &HashMap<String, &Action>,
    displayed: &mut HashSet<String>,
    depth: usize,
) {
    if displayed.contains(&action.name) {
        // Show reference to already displayed action
        let indent = "  ".repeat(depth);
        println!(
            "{}{}  {} {} (see above)",
            indent,
            "→".cyan(),
            action.name.bright_white(),
            format!("[{}]", action.status).bright_black()
        );
        return;
    }

    displayed.insert(action.name.clone());

    let indent = "  ".repeat(depth);
    let connector = if depth == 0 { "●" } else { "→" };

    let status_colored = match action.status.as_str() {
        "discovery" => action.status.blue(),
        "design" => action.status.cyan(),
        "implementation" => action.status.yellow(),
        "test" => action.status.magenta(),
        "document" => action.status.green(),
        "publish" => action.status.bright_green(),
        "published" => action.status.bright_blue(),
        _ => action.status.white(),
    };

    let priority_marker = if action.is_priority {
        " [PRIORITY]".red()
    } else {
        "".normal()
    };

    println!(
        "{}{}  {} {} {}{}",
        indent,
        connector.cyan(),
        action.name.bright_white().bold(),
        format!("[{}]", status_colored).bright_black(),
        format!("[{}]", action.project).bright_black(),
        priority_marker
    );

    // Display dependencies
    for input in &action.inputs {
        let input_name = Path::new(input)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        if let Some(&dep_action) = action_map.get(input_name) {
            display_action_tree(dep_action, action_map, displayed, depth + 1);
        } else {
            // External dependency
            let dep_indent = "  ".repeat(depth + 1);
            println!(
                "{}{}  {} {}",
                dep_indent,
                "→".bright_black(),
                input_name.bright_black(),
                "[external]".bright_black()
            );
        }
    }
}

fn move_action(base_path: &Path, from: &str, to: &str) -> Result<()> {
    let from_path = base_path.join(from);
    let to_path = base_path.join(to);

    if !from_path.exists() {
        anyhow::bail!("Source action '{}' does not exist", from_path.display());
    }

    if to_path.exists() {
        anyhow::bail!("Destination '{}' already exists", to_path.display());
    }

    // Create parent directory if it doesn't exist
    if let Some(parent) = to_path.parent() {
        fs::create_dir_all(parent)
            .context("Failed to create destination directory")?;
    }

    // Move the file
    fs::rename(&from_path, &to_path)
        .context("Failed to move action file")?;

    println!("{}", format!("Moved: {} -> {}", from, to).green());

    // Check if there's a directory with the same name (for meta-graph)
    let from_dir = from_path.with_extension("");
    let to_dir = to_path.with_extension("");

    if from_dir.exists() && from_dir.is_dir() {
        fs::rename(&from_dir, &to_dir)
            .context("Failed to move meta-graph directory")?;
        println!("{}", format!("Moved meta-graph directory: {} -> {}",
            from_dir.display(), to_dir.display()).green());
    }

    // Update all references in other actions
    let actions = find_actions(base_path)?;
    let mut updated_count = 0;

    // Prepare patterns for both markdown and wiki-link formats
    let from_without_ext = from.strip_suffix(".md").unwrap_or(from);
    let to_without_ext = to.strip_suffix(".md").unwrap_or(to);

    // Wiki-link patterns: [[from.md]] and [[from]]
    let wiki_pattern_with_ext = format!(r"\[\[{}\]\]", regex::escape(from));
    let wiki_regex_with_ext = Regex::new(&wiki_pattern_with_ext)
        .context("Failed to create wiki-link replacement regex")?;

    let wiki_pattern_without_ext = format!(r"\[\[{}\]\]", regex::escape(from_without_ext));
    let wiki_regex_without_ext = Regex::new(&wiki_pattern_without_ext)
        .context("Failed to create wiki-link replacement regex")?;

    // Markdown link patterns: [text](from.md) and [text](from)
    let md_pattern_with_ext = format!(r"\[([^\]]+)\]\({}\)", regex::escape(from));
    let md_regex_with_ext = Regex::new(&md_pattern_with_ext)
        .context("Failed to create markdown replacement regex")?;

    let md_pattern_without_ext = format!(r"\[([^\]]+)\]\({}\)", regex::escape(from_without_ext));
    let md_regex_without_ext = Regex::new(&md_pattern_without_ext)
        .context("Failed to create markdown replacement regex")?;

    for action in actions {
        let content = fs::read_to_string(&action.path)?;
        let mut new_content = content.clone();
        let mut modified = false;

        // Replace wiki-links with .md extension: [[from.md]] -> [[to.md]]
        if wiki_regex_with_ext.is_match(&new_content) {
            new_content = wiki_regex_with_ext.replace_all(&new_content, format!("[[{}]]", to)).to_string();
            modified = true;
        }

        // Replace wiki-links without .md extension: [[from]] -> [[to]]
        if from != from_without_ext && wiki_regex_without_ext.is_match(&new_content) {
            new_content = wiki_regex_without_ext.replace_all(&new_content, format!("[[{}]]", to_without_ext)).to_string();
            modified = true;
        }

        // Replace markdown links with .md extension: [text](from.md) -> [text](to.md)
        if md_regex_with_ext.is_match(&new_content) {
            new_content = md_regex_with_ext.replace_all(&new_content, |caps: &regex::Captures| {
                format!("[{}]({})", &caps[1], to)
            }).to_string();
            modified = true;
        }

        // Replace markdown links without .md extension: [text](from) -> [text](to)
        if from != from_without_ext && md_regex_without_ext.is_match(&new_content) {
            new_content = md_regex_without_ext.replace_all(&new_content, |caps: &regex::Captures| {
                format!("[{}]({})", &caps[1], to_without_ext)
            }).to_string();
            modified = true;
        }

        if modified && new_content != content {
            fs::write(&action.path, new_content)?;
            updated_count += 1;
            println!("{}", format!("Updated references in: {}", action.path.display()).cyan());
        }
    }

    if updated_count > 0 {
        println!("{}", format!("Updated {} action(s)", updated_count).bright_green());
    } else {
        println!("{}", "No references to update".bright_black());
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Check if path exists
    if !cli.path.exists() {
        anyhow::bail!("Path '{}' does not exist", cli.path.display());
    }

    if !cli.path.is_dir() {
        anyhow::bail!("Path '{}' is not a directory", cli.path.display());
    }

    let base_path = cli.path.canonicalize()
        .context("Failed to resolve path")?;

    match cli.command {
        Commands::List => list_actions(&base_path, cli.show_published)?,
        Commands::Priority => list_priority(&base_path, cli.show_published)?,
        Commands::Status { status } => list_by_status(&base_path, &status)?,
        Commands::New { name, project } => create_action(&base_path, &name, &project)?,
        Commands::Graph => graph_actions(&base_path, cli.show_published)?,
        Commands::Move { from, to } => move_action(&base_path, &from, &to)?,
    }

    Ok(())
}
