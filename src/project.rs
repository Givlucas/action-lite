use crate::Action;
use crate::Status;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    actions: HashMap<String, Action>,
}

impl Project {
    pub fn new<P: AsRef<Path>>(name: String, project_path: P) -> Result<Self> {
        let path = project_path.as_ref().to_path_buf();

        // Create project directory if it doesn't exist
        if !path.exists() {
            fs::create_dir_all(&path).with_context(|| {
                format!("Failed to create project directory: {}", path.display())
            })?;
        }

        Ok(Self {
            name,
            path,
            actions: HashMap::new(),
        })
    }

    pub fn load<P: AsRef<Path>>(project_path: P) -> Result<Self> {
        let path = project_path.as_ref().to_path_buf();
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mut project = Self {
            name,
            path: path.clone(),
            actions: HashMap::new(),
        };

        project.load_actions()?;
        Ok(project)
    }

    fn load_actions(&mut self) -> Result<()> {
        if !self.path.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                match Action::from_file(&path) {
                    Ok(action) => {
                        self.actions.insert(action.title().to_string(), action);
                    }
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to load action from {}: {}",
                            path.display(),
                            e
                        );
                    }
                }
            }
        }

        Ok(())
    }

    pub fn create_action(&mut self, title: &str, priority: bool) -> Result<Action> {
        let filename = format!("{}.md", title.replace(" ", "_").to_lowercase());
        let file_path = self.path.join(&filename);

        if file_path.exists() {
            anyhow::bail!(
                "Action '{}' already exists in project '{}'",
                title,
                self.name
            );
        }

        let mut action = Action::new(self.name.clone(), title.to_string(), priority);
        action.file_path = file_path;

        // Create initial content from template
        action.notes = Some("General notes on the task".to_string());
        action.statement_of_action = Some("The task to be performed, more in depth than title, may include why the action is needed".to_string());
        action.statement_of_inputs =
            Some("A list of .md links to other markdown files".to_string());

        action.save()?;
        self.actions.insert(title.to_string(), action.clone());

        Ok(action)
    }

    pub fn get_action(&self, title: &str) -> Option<&Action> {
        self.actions.get(title)
    }

    pub fn get_action_mut(&mut self, title: &str) -> Option<&mut Action> {
        self.actions.get_mut(title)
    }

    pub fn update_action_status(&mut self, title: &str, status: Status) -> Result<()> {
        let action = self
            .actions
            .get_mut(title)
            .with_context(|| format!("Action '{}' not found in project '{}'", title, self.name))?;

        action.set_status(status);
        action.save()?;
        Ok(())
    }

    pub fn set_action_priority(&mut self, title: &str, priority: bool) -> Result<()> {
        let action = self
            .actions
            .get_mut(title)
            .with_context(|| format!("Action '{}' not found in project '{}'", title, self.name))?;

        action.set_priority(priority);
        action.save()?;
        Ok(())
    }

    pub fn list_actions(&self) -> Vec<&Action> {
        self.actions.values().collect()
    }

    pub fn actions_by_status(&self, status: &Status) -> Vec<&Action> {
        self.actions
            .values()
            .filter(|action| action.status() == status)
            .collect()
    }

    pub fn priority_actions(&self) -> Vec<&Action> {
        self.actions
            .values()
            .filter(|action| action.is_priority())
            .collect()
    }

    pub fn create_meta_graph(&self, action_title: &str) -> Result<PathBuf> {
        let action = self.actions.get(action_title).with_context(|| {
            format!(
                "Action '{}' not found in project '{}'",
                action_title, self.name
            )
        })?;

        let meta_graph_path = action.meta_graph_path();

        if !meta_graph_path.exists() {
            fs::create_dir_all(&meta_graph_path).with_context(|| {
                format!(
                    "Failed to create meta-graph directory: {}",
                    meta_graph_path.display()
                )
            })?;
        }

        Ok(meta_graph_path)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
