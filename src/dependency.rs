//! Dependency resolution and graph building for action-lite workflow.
//!
//! This module parses dependency references from Statement of Inputs sections
//! and constructs a directed acyclic graph (DAG) representing the dependency
//! relationships between actions.
//!
//! The dependency resolution process follows a 4-stage algorithm:
//! 1. Link Extraction - Parse markdown and wiki links from Statement of Inputs
//! 2. Reference Resolution - Resolve file paths and titles to actual actions
//! 3. Graph Construction - Build the DAG structure with nodes and edges
//! 4. Cycle Detection - Validate that the graph is acyclic (no circular dependencies)

use crate::parser::Action;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Represents a directed acyclic graph of action dependencies
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    /// All nodes in the graph, keyed by action title
    pub nodes: HashMap<String, DependencyNode>,
    /// Root nodes (actions with no dependencies)
    pub roots: Vec<String>,
}

/// Represents an action node in the dependency graph
#[derive(Debug, Clone)]
pub struct DependencyNode {
    /// Reference to the original Action
    pub action: Action,
    /// Titles of actions this action depends on
    pub dependencies: Vec<String>,
    /// Titles of actions that depend on this action (reverse edges)
    pub dependents: Vec<String>,
}

/// Errors that can occur during dependency resolution
#[derive(Debug)]
pub enum DependencyError {
    /// A referenced action could not be found
    MissingDependency {
        referencing_action: String,
        missing_reference: String,
    },
    /// Circular dependency detected
    CircularDependency {
        cycle_path: Vec<String>,
    },
    /// Failed to resolve a link reference
    UnresolvableReference {
        action: String,
        reference: String,
        reason: String,
    },
}

impl std::fmt::Display for DependencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DependencyError::MissingDependency {
                referencing_action,
                missing_reference,
            } => {
                write!(
                    f,
                    "Missing dependency: action '{}' references '{}' which does not exist",
                    referencing_action, missing_reference
                )
            }
            DependencyError::CircularDependency { cycle_path } => {
                write!(
                    f,
                    "Circular dependency detected: {}",
                    cycle_path.join(" → ")
                )
            }
            DependencyError::UnresolvableReference {
                action,
                reference,
                reason,
            } => {
                write!(
                    f,
                    "Failed to resolve reference '{}' in action '{}': {}",
                    reference, action, reason
                )
            }
        }
    }
}

impl std::error::Error for DependencyError {}

/// Result type for dependency operations
pub type DependencyResult<T> = Result<T, DependencyError>;

/// Build a dependency graph from a collection of actions
///
/// # Arguments
/// * `actions` - Vector of parsed Action objects
///
/// # Returns
/// A DependencyGraph structure representing the action dependencies
///
/// # Errors
/// Returns DependencyError if:
/// - A referenced action cannot be found (MissingDependency)
/// - A circular dependency is detected (CircularDependency)
/// - A reference cannot be resolved (UnresolvableReference)
pub fn build_dependency_graph(actions: Vec<Action>) -> DependencyResult<DependencyGraph> {
    // Stage 1: Extract raw dependency links from all actions
    let raw_dependencies = extract_all_links(&actions);

    // Stage 2: Resolve references to actual action titles
    let resolved_dependencies = resolve_references(&actions, &raw_dependencies)?;

    // Stage 3: Construct the graph structure
    let graph = construct_graph(&actions, &resolved_dependencies);

    // Stage 4: Detect cycles in the graph
    detect_cycles(&graph)?;

    Ok(graph)
}

/// Stage 1: Extract all dependency links from Statement of Inputs sections
///
/// Returns a map of action title -> vector of raw link references
fn extract_all_links(actions: &[Action]) -> HashMap<String, Vec<String>> {
    let mut all_links = HashMap::new();

    // Regex patterns for matching links
    // Markdown link: [Link Text](path/to/file.md)
    let markdown_regex = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
    // Wiki link: [[Action Title]]
    let wiki_regex = Regex::new(r"\[\[([^\]]+)\]\]").unwrap();

    for action in actions {
        // Skip if no Statement of Inputs
        let inputs = match &action.statement_of_inputs {
            Some(content) => content,
            None => continue,
        };

        let mut links = Vec::new();

        // Extract markdown links
        for cap in markdown_regex.captures_iter(inputs) {
            if let Some(url) = cap.get(2) {
                let url_str = url.as_str();
                // Skip external links (http://, https://)
                if !url_str.starts_with("http://") && !url_str.starts_with("https://") {
                    links.push(url_str.to_string());
                }
            }
        }

        // Extract wiki links
        for cap in wiki_regex.captures_iter(inputs) {
            if let Some(title) = cap.get(1) {
                links.push(format!("[[{}]]", title.as_str()));
            }
        }

        all_links.insert(action.title.clone(), links);
    }

    all_links
}

/// Stage 2: Resolve raw link references to action titles
///
/// Takes raw references (file paths or wiki links) and resolves them
/// to actual action titles by matching against the action collection.
fn resolve_references(
    actions: &[Action],
    raw_dependencies: &HashMap<String, Vec<String>>,
) -> DependencyResult<HashMap<String, Vec<String>>> {
    // Build lookup maps for O(1) resolution
    let path_lookup = build_path_lookup(actions);
    let title_lookup = build_title_lookup(actions);

    let mut resolved = HashMap::new();

    for (action_title, raw_refs) in raw_dependencies {
        let mut resolved_refs = Vec::new();

        // Get the action to access its path for relative resolution
        let action = actions
            .iter()
            .find(|a| &a.title == action_title)
            .expect("Action should exist in collection");

        for raw_ref in raw_refs {
            if let Some(resolved_title) = resolve_single_reference(
                raw_ref,
                &action.path,
                &path_lookup,
                &title_lookup,
            ) {
                resolved_refs.push(resolved_title);
            } else {
                // Reference could not be resolved - missing dependency
                return Err(DependencyError::MissingDependency {
                    referencing_action: action_title.clone(),
                    missing_reference: raw_ref.clone(),
                });
            }
        }

        resolved.insert(action_title.clone(), resolved_refs);
    }

    Ok(resolved)
}

/// Build a lookup map from normalized file paths to actions
fn build_path_lookup(actions: &[Action]) -> HashMap<PathBuf, Action> {
    actions
        .iter()
        .map(|action| (action.path.clone(), action.clone()))
        .collect()
}

/// Build a lookup map from titles to actions
fn build_title_lookup(actions: &[Action]) -> HashMap<String, Action> {
    actions
        .iter()
        .map(|action| (action.title.clone(), action.clone()))
        .collect()
}

/// Resolve a single reference to an action title
///
/// Handles both markdown links (file paths) and wiki links (titles)
fn resolve_single_reference(
    reference: &str,
    referencing_action_path: &Path,
    path_lookup: &HashMap<PathBuf, Action>,
    title_lookup: &HashMap<String, Action>,
) -> Option<String> {
    // Check if it's a wiki link
    if reference.starts_with("[[") && reference.ends_with("]]") {
        // Extract title from [[Title]]
        let title = &reference[2..reference.len() - 2];
        return title_lookup.get(title).map(|action| action.title.clone());
    }

    // Otherwise, treat as file path (markdown link)
    resolve_file_path(reference, referencing_action_path, path_lookup)
}

/// Resolve a file path reference to an action title
///
/// Handles relative paths (./, ../) and normalizes paths for matching
fn resolve_file_path(
    file_path: &str,
    referencing_action_path: &Path,
    path_lookup: &HashMap<PathBuf, Action>,
) -> Option<String> {
    // Get the directory of the referencing action
    let base_dir = referencing_action_path.parent()?;

    // Resolve relative path
    let mut resolved_path = base_dir.join(file_path);

    // Strip .md extension if present for matching
    if resolved_path.extension().map_or(false, |ext| ext == "md") {
        resolved_path.set_extension("");
        resolved_path = resolved_path.with_extension("md");
    }

    // Try to canonicalize to resolve . and .. components
    // If canonicalize fails (path doesn't exist), try manual resolution
    let normalized_path = if let Ok(canonical) = resolved_path.canonicalize() {
        canonical
    } else {
        // Manual normalization: just resolve the path components
        normalize_path(&resolved_path)
    };

    // Look up in path map
    path_lookup
        .get(&normalized_path)
        .map(|action| action.title.clone())
}

/// Manually normalize a path by resolving . and .. components
///
/// This is a fallback when canonicalize() fails (e.g., path doesn't exist yet)
fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();

    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                // Pop the last component (go up one directory)
                components.pop();
            }
            std::path::Component::CurDir => {
                // Skip current directory markers
            }
            _ => {
                // Add normal components
                components.push(component);
            }
        }
    }

    components.iter().collect()
}

/// Stage 3: Construct the dependency graph structure
///
/// Builds the DAG with nodes, edges, and identifies root nodes
fn construct_graph(
    actions: &[Action],
    resolved_dependencies: &HashMap<String, Vec<String>>,
) -> DependencyGraph {
    let mut nodes = HashMap::new();

    // Create nodes for all actions
    for action in actions {
        let dependencies = resolved_dependencies
            .get(&action.title)
            .cloned()
            .unwrap_or_default();

        let node = DependencyNode {
            action: action.clone(),
            dependencies,
            dependents: Vec::new(), // Will be populated in next step
        };

        nodes.insert(action.title.clone(), node);
    }

    // Build reverse edges (dependents)
    let dependency_pairs: Vec<(String, String)> = nodes
        .iter()
        .flat_map(|(title, node)| {
            node.dependencies
                .iter()
                .map(move |dep| (dep.clone(), title.clone()))
        })
        .collect();

    for (dependency_title, dependent_title) in dependency_pairs {
        if let Some(node) = nodes.get_mut(&dependency_title) {
            node.dependents.push(dependent_title);
        }
    }

    // Identify root nodes (no dependencies)
    let roots = nodes
        .iter()
        .filter(|(_, node)| node.dependencies.is_empty())
        .map(|(title, _)| title.clone())
        .collect();

    DependencyGraph { nodes, roots }
}

/// Stage 4: Detect cycles in the dependency graph
///
/// Uses depth-first search with recursion stack tracking to detect cycles.
/// Returns error if any cycle is found.
fn detect_cycles(graph: &DependencyGraph) -> DependencyResult<()> {
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();

    for node_title in graph.nodes.keys() {
        if !visited.contains(node_title.as_str()) {
            let mut path = Vec::new();
            dfs_cycle_check(
                node_title,
                graph,
                &mut visited,
                &mut rec_stack,
                &mut path,
            )?;
        }
    }

    Ok(())
}

/// Recursive DFS helper for cycle detection
///
/// Maintains a recursion stack and path to detect back edges (cycles)
fn dfs_cycle_check(
    node_title: &str,
    graph: &DependencyGraph,
    visited: &mut HashSet<String>,
    rec_stack: &mut HashSet<String>,
    path: &mut Vec<String>,
) -> DependencyResult<()> {
    // Mark as visited and add to recursion stack
    visited.insert(node_title.to_string());
    rec_stack.insert(node_title.to_string());
    path.push(node_title.to_string());

    // Get the node
    let node = &graph.nodes[node_title];

    // Check all dependencies
    for dep in &node.dependencies {
        if rec_stack.contains(dep.as_str()) {
            // Found a cycle! Dependency is in our current path
            let cycle_start_idx = path.iter().position(|p| p == dep).unwrap();
            let mut cycle_path = path[cycle_start_idx..].to_vec();
            cycle_path.push(dep.clone()); // Complete the cycle
            return Err(DependencyError::CircularDependency { cycle_path });
        }

        if !visited.contains(dep.as_str()) {
            dfs_cycle_check(dep, graph, visited, rec_stack, path)?;
        }
    }

    // Remove from recursion stack and path
    rec_stack.remove(node_title);
    path.pop();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Phase;
    use std::fs::{self, File};
    use std::io::Write;

    /// Helper to create a temporary test directory
    fn create_temp_dir(name: &str) -> PathBuf {
        let temp_base = std::env::temp_dir();
        let test_dir = temp_base.join(format!("action_lite_dependency_test_{}", name));

        // Clean up if it exists from previous test
        if test_dir.exists() {
            fs::remove_dir_all(&test_dir).ok();
        }

        fs::create_dir_all(&test_dir).expect("Failed to create test directory");
        test_dir
    }

    /// Helper to create a test action
    fn create_test_action(
        path: PathBuf,
        title: &str,
        statement_of_inputs: Option<String>,
    ) -> Action {
        Action {
            path,
            title: title.to_string(),
            phase: Phase::Design,
            priority: false,
            project_tags: vec![],
            statement_of_inputs,
        }
    }

    /// Helper to create a file with content
    fn create_test_file(dir: &Path, name: &str, content: &str) -> PathBuf {
        let file_path = dir.join(name);
        let mut file = File::create(&file_path).expect("Failed to create test file");
        file.write_all(content.as_bytes())
            .expect("Failed to write test file");
        file_path
    }

    #[test]
    fn test_extract_markdown_links() {
        let test_dir = create_temp_dir("extract_markdown");
        let path = create_test_file(&test_dir, "test.md", "");

        let action = create_test_action(
            path,
            "Test Action",
            Some("Depends on [Action A](./action-a.md) and [Action B](../parent/action-b.md)".to_string()),
        );

        let links = extract_all_links(&[action]);
        assert_eq!(links.len(), 1);
        assert!(links.contains_key("Test Action"));

        let refs = &links["Test Action"];
        assert_eq!(refs.len(), 2);
        assert!(refs.contains(&"./action-a.md".to_string()));
        assert!(refs.contains(&"../parent/action-b.md".to_string()));

        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_extract_wiki_links() {
        let test_dir = create_temp_dir("extract_wiki");
        let path = create_test_file(&test_dir, "test.md", "");

        let action = create_test_action(
            path,
            "Test Action",
            Some("Depends on [[Action A]] and [[Action B]]".to_string()),
        );

        let links = extract_all_links(&[action]);
        let refs = &links["Test Action"];
        assert_eq!(refs.len(), 2);
        assert!(refs.contains(&"[[Action A]]".to_string()));
        assert!(refs.contains(&"[[Action B]]".to_string()));

        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_extract_mixed_links() {
        let test_dir = create_temp_dir("extract_mixed");
        let path = create_test_file(&test_dir, "test.md", "");

        let action = create_test_action(
            path,
            "Test Action",
            Some("Depends on [File Link](./action-a.md) and [[Wiki Link]]".to_string()),
        );

        let links = extract_all_links(&[action]);
        let refs = &links["Test Action"];
        assert_eq!(refs.len(), 2);
        assert!(refs.contains(&"./action-a.md".to_string()));
        assert!(refs.contains(&"[[Wiki Link]]".to_string()));

        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_extract_skips_external_links() {
        let test_dir = create_temp_dir("extract_external");
        let path = create_test_file(&test_dir, "test.md", "");

        let action = create_test_action(
            path,
            "Test Action",
            Some("See [External](https://example.com) and [Local](./action.md)".to_string()),
        );

        let links = extract_all_links(&[action]);
        let refs = &links["Test Action"];
        assert_eq!(refs.len(), 1);
        assert!(refs.contains(&"./action.md".to_string()));

        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_extract_empty_inputs() {
        let test_dir = create_temp_dir("extract_empty");
        let path = create_test_file(&test_dir, "test.md", "");

        let action = create_test_action(path, "Test Action", None);

        let links = extract_all_links(&[action]);
        assert!(!links.contains_key("Test Action"));

        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_resolve_wiki_link() {
        let test_dir = create_temp_dir("resolve_wiki");
        let path1 = create_test_file(&test_dir, "action-a.md", "");
        let path2 = create_test_file(&test_dir, "action-b.md", "");

        let action_a = create_test_action(
            path1,
            "Action A",
            Some("Depends on [[Action B]]".to_string()),
        );
        let action_b = create_test_action(path2, "Action B", None);

        let actions = vec![action_a, action_b];
        let raw_deps = extract_all_links(&actions);
        let resolved = resolve_references(&actions, &raw_deps).unwrap();

        assert_eq!(resolved["Action A"], vec!["Action B"]);

        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_resolve_file_path_same_dir() {
        let test_dir = create_temp_dir("resolve_file_same");
        let path1 = create_test_file(&test_dir, "action-a.md", "");
        let path2 = create_test_file(&test_dir, "action-b.md", "");

        let action_a = create_test_action(
            path1,
            "action-a",
            Some("Depends on [B](./action-b.md)".to_string()),
        );
        let action_b = create_test_action(path2, "action-b", None);

        let actions = vec![action_a, action_b];
        let raw_deps = extract_all_links(&actions);
        let resolved = resolve_references(&actions, &raw_deps).unwrap();

        assert_eq!(resolved["action-a"], vec!["action-b"]);

        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_missing_dependency_error() {
        let test_dir = create_temp_dir("missing_dep");
        let path = create_test_file(&test_dir, "action-a.md", "");

        let action = create_test_action(
            path,
            "Action A",
            Some("Depends on [[Missing Action]]".to_string()),
        );

        let actions = vec![action];
        let raw_deps = extract_all_links(&actions);
        let result = resolve_references(&actions, &raw_deps);

        assert!(result.is_err());
        match result {
            Err(DependencyError::MissingDependency {
                referencing_action,
                missing_reference,
            }) => {
                assert_eq!(referencing_action, "Action A");
                assert_eq!(missing_reference, "[[Missing Action]]");
            }
            _ => panic!("Expected MissingDependency error"),
        }

        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_construct_simple_graph() {
        let test_dir = create_temp_dir("construct_simple");
        let path1 = create_test_file(&test_dir, "action-a.md", "");
        let path2 = create_test_file(&test_dir, "action-b.md", "");

        let action_a = create_test_action(
            path1,
            "Action A",
            Some("Depends on [[Action B]]".to_string()),
        );
        let action_b = create_test_action(path2, "Action B", None);

        let actions = vec![action_a, action_b];
        let mut resolved = HashMap::new();
        resolved.insert("Action A".to_string(), vec!["Action B".to_string()]);
        resolved.insert("Action B".to_string(), vec![]);

        let graph = construct_graph(&actions, &resolved);

        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.roots, vec!["Action B"]);
        assert_eq!(graph.nodes["Action A"].dependencies, vec!["Action B"]);
        assert_eq!(graph.nodes["Action B"].dependents, vec!["Action A"]);

        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_construct_multiple_parents() {
        let test_dir = create_temp_dir("construct_multi_parent");
        let path1 = create_test_file(&test_dir, "a.md", "");
        let path2 = create_test_file(&test_dir, "b.md", "");
        let path3 = create_test_file(&test_dir, "c.md", "");

        let action_a = create_test_action(path1, "A", Some("Depends on [[C]]".to_string()));
        let action_b = create_test_action(path2, "B", Some("Depends on [[C]]".to_string()));
        let action_c = create_test_action(path3, "C", None);

        let actions = vec![action_a, action_b, action_c];
        let mut resolved = HashMap::new();
        resolved.insert("A".to_string(), vec!["C".to_string()]);
        resolved.insert("B".to_string(), vec!["C".to_string()]);
        resolved.insert("C".to_string(), vec![]);

        let graph = construct_graph(&actions, &resolved);

        assert_eq!(graph.nodes["C"].dependents.len(), 2);
        assert!(graph.nodes["C"].dependents.contains(&"A".to_string()));
        assert!(graph.nodes["C"].dependents.contains(&"B".to_string()));

        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_detect_simple_cycle() {
        let test_dir = create_temp_dir("cycle_simple");
        let path1 = create_test_file(&test_dir, "a.md", "");
        let path2 = create_test_file(&test_dir, "b.md", "");

        let action_a = create_test_action(path1, "A", Some("[[B]]".to_string()));
        let action_b = create_test_action(path2, "B", Some("[[A]]".to_string()));

        let actions = vec![action_a, action_b];
        let result = build_dependency_graph(actions);

        assert!(result.is_err());
        match result {
            Err(DependencyError::CircularDependency { cycle_path }) => {
                assert!(cycle_path.len() >= 2);
                // Cycle should be A -> B -> A or B -> A -> B
                assert!(
                    (cycle_path[0] == "A" && cycle_path[1] == "B")
                        || (cycle_path[0] == "B" && cycle_path[1] == "A")
                );
            }
            _ => panic!("Expected CircularDependency error"),
        }

        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_detect_complex_cycle() {
        let test_dir = create_temp_dir("cycle_complex");
        let path1 = create_test_file(&test_dir, "a.md", "");
        let path2 = create_test_file(&test_dir, "b.md", "");
        let path3 = create_test_file(&test_dir, "c.md", "");
        let path4 = create_test_file(&test_dir, "d.md", "");

        let action_a = create_test_action(path1, "A", Some("[[B]]".to_string()));
        let action_b = create_test_action(path2, "B", Some("[[C]]".to_string()));
        let action_c = create_test_action(path3, "C", Some("[[D]]".to_string()));
        let action_d = create_test_action(path4, "D", Some("[[B]]".to_string())); // Creates cycle

        let actions = vec![action_a, action_b, action_c, action_d];
        let result = build_dependency_graph(actions);

        assert!(result.is_err());
        match result {
            Err(DependencyError::CircularDependency { cycle_path }) => {
                assert!(cycle_path.len() >= 2);
            }
            _ => panic!("Expected CircularDependency error"),
        }

        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_no_cycle_in_valid_dag() {
        let test_dir = create_temp_dir("no_cycle");
        let path1 = create_test_file(&test_dir, "a.md", "");
        let path2 = create_test_file(&test_dir, "b.md", "");
        let path3 = create_test_file(&test_dir, "c.md", "");

        // Linear: A -> B -> C
        let action_a = create_test_action(path1, "A", Some("[[B]]".to_string()));
        let action_b = create_test_action(path2, "B", Some("[[C]]".to_string()));
        let action_c = create_test_action(path3, "C", None);

        let actions = vec![action_a, action_b, action_c];
        let result = build_dependency_graph(actions);

        assert!(result.is_ok());

        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_self_reference_cycle() {
        let test_dir = create_temp_dir("self_cycle");
        let path = create_test_file(&test_dir, "a.md", "");

        let action = create_test_action(path, "A", Some("[[A]]".to_string()));

        let result = build_dependency_graph(vec![action]);

        assert!(result.is_err());
        match result {
            Err(DependencyError::CircularDependency { .. }) => {}
            _ => panic!("Expected CircularDependency error"),
        }

        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_build_full_graph_integration() {
        let test_dir = create_temp_dir("full_integration");
        let path1 = create_test_file(&test_dir, "root.md", "");
        let path2 = create_test_file(&test_dir, "child1.md", "");
        let path3 = create_test_file(&test_dir, "child2.md", "");
        let path4 = create_test_file(&test_dir, "grandchild.md", "");

        // Root -> Child1 -> Grandchild
        //      -> Child2 -> Grandchild (diamond pattern)
        let root = create_test_action(
            path1,
            "Root",
            Some("[[Child1]] and [[Child2]]".to_string()),
        );
        let child1 = create_test_action(path2, "Child1", Some("[[Grandchild]]".to_string()));
        let child2 = create_test_action(path3, "Child2", Some("[[Grandchild]]".to_string()));
        let grandchild = create_test_action(path4, "Grandchild", None);

        let actions = vec![root, child1, child2, grandchild];
        let result = build_dependency_graph(actions);

        assert!(result.is_ok());
        let graph = result.unwrap();

        assert_eq!(graph.nodes.len(), 4);
        assert_eq!(graph.roots, vec!["Grandchild"]);
        assert_eq!(graph.nodes["Root"].dependencies.len(), 2);
        assert_eq!(graph.nodes["Grandchild"].dependents.len(), 2);

        fs::remove_dir_all(&test_dir).ok();
    }

    #[test]
    fn test_orphan_nodes() {
        let test_dir = create_temp_dir("orphan");
        let path1 = create_test_file(&test_dir, "orphan.md", "");
        let path2 = create_test_file(&test_dir, "connected.md", "");

        let orphan = create_test_action(path1, "Orphan", None);
        let connected = create_test_action(path2, "Connected", Some("[[Orphan]]".to_string()));

        let actions = vec![orphan, connected];
        let result = build_dependency_graph(actions);

        assert!(result.is_ok());
        let graph = result.unwrap();

        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.roots, vec!["Orphan"]);
        assert_eq!(graph.nodes["Orphan"].dependents, vec!["Connected"]);

        fs::remove_dir_all(&test_dir).ok();
    }
}
