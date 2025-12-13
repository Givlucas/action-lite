//! Graph visualization module for rendering dependency DAGs as unicode trees.
//!
//! This module transforms DependencyGraph structures into human-readable
//! unicode tree representations. It handles DAG visualization where each
//! node appears exactly once, with reference pointers for subsequent occurrences.

use crate::dependency::DependencyGraph;
use std::collections::HashSet;

/// Visualizes a dependency graph using unicode box-drawing characters
pub struct GraphVisualizer {
    /// The dependency graph to visualize
    graph: DependencyGraph,
    /// Tracks which nodes have been rendered in-place
    rendered_nodes: HashSet<String>,
}

/// Context passed during tree traversal to manage indentation
struct RenderContext {
    /// Prefix state for each ancestor level (true = continuation │, false = empty space)
    prefix: Vec<bool>,
    /// Whether this node is the last child at its level
    is_last_child: bool,
}

impl RenderContext {
    /// Create a new root-level context
    fn new() -> Self {
        RenderContext {
            prefix: Vec::new(),
            is_last_child: false,
        }
    }

    /// Create a child context by pushing a new level
    fn push_level(&self, is_last: bool) -> Self {
        let mut new_prefix = self.prefix.clone();
        // Add continuation state for current level
        // If current node is last child, don't draw vertical line in its column
        new_prefix.push(!self.is_last_child);

        RenderContext {
            prefix: new_prefix,
            is_last_child: is_last,
        }
    }
}

impl GraphVisualizer {
    /// Create a new graph visualizer
    pub fn new(graph: DependencyGraph) -> Self {
        GraphVisualizer {
            graph,
            rendered_nodes: HashSet::new(),
        }
    }

    /// Render the graph to stdout
    pub fn render(&mut self) {
        // Handle empty graph
        if self.graph.nodes.is_empty() {
            return;
        }

        // Find top-level actions (nodes with no dependents - nothing depends on them)
        let top_level: Vec<String> = self
            .graph
            .nodes
            .iter()
            .filter(|(_, node)| node.dependents.is_empty())
            .map(|(title, _)| title.clone())
            .collect();

        // Render each top-level action and its subtree
        for (i, action_title) in top_level.iter().enumerate() {
            // Add blank line between top-level actions (but not before first)
            if i > 0 {
                println!();
            }
            self.render_node(action_title, &RenderContext::new(), true);
        }

        // Handle orphan nodes (nodes with no dependencies and no dependents that haven't been rendered)
        let orphans: Vec<String> = self
            .graph
            .nodes
            .iter()
            .filter(|(title, node)| {
                node.dependencies.is_empty()
                    && node.dependents.is_empty()
                    && !self.rendered_nodes.contains(*title)
            })
            .map(|(title, _)| title.clone())
            .collect();

        if !orphans.is_empty() {
            println!(); // Blank line before orphans
            for orphan_title in orphans {
                println!("{} (orphan)", orphan_title);
            }
        }
    }

    /// Recursively render a node and its dependencies
    fn render_node(&mut self, node_title: &str, context: &RenderContext, is_root: bool) {
        // Check if this node has already been rendered in-place
        if self.rendered_nodes.contains(node_title) {
            // Render as reference
            self.print_with_indent(context, &format!("→ {}", node_title));
            return;
        }

        // Mark as rendered
        self.rendered_nodes.insert(node_title.to_string());

        // Get the node data (title and dependencies) - clone to release borrow
        let (title, dependencies) = match self.graph.nodes.get(node_title) {
            Some(n) => (n.action.title.clone(), n.dependencies.clone()),
            None => {
                // Defensive: should never happen if graph is valid
                eprintln!("Warning: node '{}' not found in graph", node_title);
                return;
            }
        };

        // Render the node title
        if is_root {
            println!("{}", title);
        } else {
            self.print_with_indent(context, &title);
        }

        // Render dependencies (children in tree view)
        if dependencies.is_empty() {
            return;
        }

        for (i, dep_title) in dependencies.iter().enumerate() {
            let is_last = i == dependencies.len() - 1;

            // Create new context for child
            let child_context = context.push_level(is_last);

            // Recursively render dependency
            self.render_node(dep_title, &child_context, false);
        }
    }

    /// Print text with proper indentation and tree connectors
    fn print_with_indent(&self, context: &RenderContext, text: &str) {
        // Build prefix string from context
        let mut prefix = String::new();
        for &has_continuation in &context.prefix {
            if has_continuation {
                prefix.push_str("│   ");
            } else {
                prefix.push_str("    ");
            }
        }

        // Add tree connector
        let connector = if context.is_last_child {
            "└── "
        } else {
            "├── "
        };

        println!("{}{}{}", prefix, connector, text);
    }
}

/// Main entry point: render a dependency graph to stdout
pub fn render_graph(graph: DependencyGraph) {
    let mut visualizer = GraphVisualizer::new(graph);
    visualizer.render();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dependency::{DependencyNode, DependencyGraph};
    use crate::parser::{Action, Phase};
    use std::collections::HashMap;
    use std::path::PathBuf;

    /// Helper to create a test action
    fn create_action(title: &str) -> Action {
        Action {
            path: PathBuf::from(format!("/test/{}.md", title)),
            title: title.to_string(),
            phase: Phase::Design,
            priority: false,
            project_tags: vec![],
            statement_of_inputs: None,
        }
    }

    /// Helper to create a dependency node
    fn create_node(title: &str, dependencies: Vec<&str>) -> DependencyNode {
        DependencyNode {
            action: create_action(title),
            dependencies: dependencies.iter().map(|s| s.to_string()).collect(),
            dependents: Vec::new(), // Will be populated by graph construction
        }
    }

    /// Helper to build a graph with reverse edges populated
    fn build_graph(nodes: Vec<(&str, Vec<&str>)>) -> DependencyGraph {
        let mut node_map = HashMap::new();

        // Create nodes
        for (title, deps) in &nodes {
            let node = create_node(title, deps.clone());
            node_map.insert(title.to_string(), node);
        }

        // Build reverse edges (dependents)
        let dependency_pairs: Vec<(String, String)> = node_map
            .iter()
            .flat_map(|(title, node)| {
                node.dependencies
                    .iter()
                    .map(move |dep| (dep.clone(), title.clone()))
            })
            .collect();

        for (dependency_title, dependent_title) in dependency_pairs {
            if let Some(node) = node_map.get_mut(&dependency_title) {
                node.dependents.push(dependent_title);
            }
        }

        // Identify roots
        let roots: Vec<String> = node_map
            .iter()
            .filter(|(_, node)| node.dependencies.is_empty())
            .map(|(title, _)| title.clone())
            .collect();

        DependencyGraph {
            nodes: node_map,
            roots,
        }
    }

    #[test]
    fn test_empty_graph() {
        let graph = DependencyGraph {
            nodes: HashMap::new(),
            roots: Vec::new(),
        };

        let mut visualizer = GraphVisualizer::new(graph);
        visualizer.render();
        // Should not panic, should output nothing
    }

    #[test]
    fn test_single_node_no_dependencies() {
        let nodes = vec![("A", vec![])];
        let graph = build_graph(nodes);

        let mut visualizer = GraphVisualizer::new(graph);
        visualizer.render();
        // Should render just "A" with no children
        assert!(visualizer.rendered_nodes.contains("A"));
    }

    #[test]
    fn test_linear_dependency_chain() {
        // A -> B -> C
        let nodes = vec![
            ("A", vec!["B"]),
            ("B", vec!["C"]),
            ("C", vec![]),
        ];
        let graph = build_graph(nodes);

        let mut visualizer = GraphVisualizer::new(graph);
        visualizer.render();

        // All nodes should be rendered
        assert!(visualizer.rendered_nodes.contains("A"));
        assert!(visualizer.rendered_nodes.contains("B"));
        assert!(visualizer.rendered_nodes.contains("C"));
    }

    #[test]
    fn test_branching_dependencies() {
        // A -> B
        //   -> C
        let nodes = vec![
            ("A", vec!["B", "C"]),
            ("B", vec![]),
            ("C", vec![]),
        ];
        let graph = build_graph(nodes);

        let mut visualizer = GraphVisualizer::new(graph);
        visualizer.render();

        assert!(visualizer.rendered_nodes.contains("A"));
        assert!(visualizer.rendered_nodes.contains("B"));
        assert!(visualizer.rendered_nodes.contains("C"));
    }

    #[test]
    fn test_diamond_pattern() {
        // A -> B -> D
        //   -> C -> D
        // D should appear in-place under first parent (B), reference under second (C)
        let nodes = vec![
            ("A", vec!["B", "C"]),
            ("B", vec!["D"]),
            ("C", vec!["D"]),
            ("D", vec![]),
        ];
        let graph = build_graph(nodes);

        let mut visualizer = GraphVisualizer::new(graph);
        visualizer.render();

        // D should be rendered in-place exactly once
        assert!(visualizer.rendered_nodes.contains("D"));
        // All nodes should be accounted for
        assert_eq!(visualizer.rendered_nodes.len(), 4);
    }

    #[test]
    fn test_multiple_parents() {
        // A -> C
        // B -> C
        // C should appear in-place once
        let nodes = vec![
            ("A", vec!["C"]),
            ("B", vec!["C"]),
            ("C", vec![]),
        ];
        let graph = build_graph(nodes);

        let mut visualizer = GraphVisualizer::new(graph);
        visualizer.render();

        // C should be rendered exactly once
        assert!(visualizer.rendered_nodes.contains("C"));
        assert_eq!(visualizer.rendered_nodes.len(), 3);
    }

    #[test]
    fn test_multiple_roots() {
        // A -> B
        // C -> D (separate tree)
        let nodes = vec![
            ("A", vec!["B"]),
            ("B", vec![]),
            ("C", vec!["D"]),
            ("D", vec![]),
        ];
        let graph = build_graph(nodes);

        let mut visualizer = GraphVisualizer::new(graph);
        visualizer.render();

        // All nodes should be rendered
        assert_eq!(visualizer.rendered_nodes.len(), 4);
    }

    #[test]
    fn test_orphan_nodes() {
        // Orphan: no dependencies, no dependents
        let nodes = vec![
            ("Connected", vec!["Dependency"]),
            ("Dependency", vec![]),
            ("Orphan", vec![]),
        ];
        let mut graph = build_graph(nodes);

        // Manually ensure "Orphan" has no dependents (it shouldn't anyway)
        if let Some(orphan) = graph.nodes.get_mut("Orphan") {
            orphan.dependents.clear();
        }

        let mut visualizer = GraphVisualizer::new(graph);
        visualizer.render();

        // Orphan should be rendered
        assert!(visualizer.rendered_nodes.contains("Orphan"));
    }

    #[test]
    fn test_deeply_nested() {
        // 5 levels deep: A -> B -> C -> D -> E
        let nodes = vec![
            ("A", vec!["B"]),
            ("B", vec!["C"]),
            ("C", vec!["D"]),
            ("D", vec!["E"]),
            ("E", vec![]),
        ];
        let graph = build_graph(nodes);

        let mut visualizer = GraphVisualizer::new(graph);
        visualizer.render();

        assert_eq!(visualizer.rendered_nodes.len(), 5);
    }

    #[test]
    fn test_wide_graph() {
        // A has many children
        let nodes = vec![
            ("A", vec!["B", "C", "D", "E", "F"]),
            ("B", vec![]),
            ("C", vec![]),
            ("D", vec![]),
            ("E", vec![]),
            ("F", vec![]),
        ];
        let graph = build_graph(nodes);

        let mut visualizer = GraphVisualizer::new(graph);
        visualizer.render();

        assert_eq!(visualizer.rendered_nodes.len(), 6);
    }

    #[test]
    fn test_complex_dag() {
        // More complex structure with multiple shared dependencies
        //     A
        //    / \
        //   B   C
        //   |\ /|
        //   | X |
        //   |/ \|
        //   D   E
        //    \ /
        //     F
        let nodes = vec![
            ("A", vec!["B", "C"]),
            ("B", vec!["D", "E"]),
            ("C", vec!["D", "E"]),
            ("D", vec!["F"]),
            ("E", vec!["F"]),
            ("F", vec![]),
        ];
        let graph = build_graph(nodes);

        let mut visualizer = GraphVisualizer::new(graph);
        visualizer.render();

        // Each node should be rendered exactly once
        assert_eq!(visualizer.rendered_nodes.len(), 6);
        assert!(visualizer.rendered_nodes.contains("F"));
        assert!(visualizer.rendered_nodes.contains("D"));
        assert!(visualizer.rendered_nodes.contains("E"));
    }

    #[test]
    fn test_render_context_push_level() {
        let ctx = RenderContext::new();
        assert!(ctx.prefix.is_empty());
        assert!(!ctx.is_last_child);

        let child_ctx = ctx.push_level(false);
        assert_eq!(child_ctx.prefix.len(), 1);
        assert!(!child_ctx.is_last_child);

        let grandchild_ctx = child_ctx.push_level(true);
        assert_eq!(grandchild_ctx.prefix.len(), 2);
        assert!(grandchild_ctx.is_last_child);
    }

    #[test]
    fn test_reference_pointer_format() {
        // Create a small graph to test reference rendering
        // A -> B -> C
        //   -> C (C should be reference here)
        let nodes = vec![
            ("A", vec!["B", "C"]),
            ("B", vec!["C"]),
            ("C", vec![]),
        ];
        let graph = build_graph(nodes);

        let mut visualizer = GraphVisualizer::new(graph);
        visualizer.render();

        // C should appear once in-place and once as reference
        // (We can't easily test stdout output, but we can verify internal state)
        assert!(visualizer.rendered_nodes.contains("C"));
    }
}
