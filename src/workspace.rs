use crate::{Action, Project, Status};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
pub struct Workspace {
    pub root: PathBuf,
    projects: HashMap<String, Project>,
}

impl Workspace {
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Self> {
        let root = path.as_ref().to_path_buf();
        
        // Create workspace directory if it doesn't exist
        if !root.exists() {
            fs::create_dir_all(&root)
                .with_context(|| format!("Failed to create workspace directory: {}", root.display()))?;
        }
        
        // Create .action-lite marker file
        let marker_file = root.join(".action-lite");
        fs::write(&marker_file, "# Action Lite Workspace\n")
            .with_context(|| "Failed to create workspace marker file")?;
        
        // Create README
        let readme_path = root.join("README.md");
        if !readme_path.exists() {
            let readme_content = include_str!("../templates/workspace_readme.md");
            fs::write(&readme_path, readme_content)
                .with_context(|| "Failed to create workspace README")?;
        }
        
        Ok(Self {
            root,
            projects: HashMap::new(),
        })
    }
    
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let root = path.as_ref().to_path_buf();
        
        // Check if this is a valid workspace
        let marker_file = root.join(".action-lite");
        if !marker_file.exists() {
            anyhow::bail!(
                "Not an Action Lite workspace. Run 'action init' to initialize a workspace in: {}", 
                root.display()
            );
        }
        
        let mut workspace = Self {
            root: root.clone(),
            projects: HashMap::new(),
        };
        
        workspace.load_projects()?;
        Ok(workspace)
    }
    
    fn load_projects(&mut self) -> Result<()> {
        for entry in fs::read_dir(&self.root)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() && !path.file_name().unwrap().to_str().unwrap().starts_with('.') {
                let project_name = path.file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                match Project::load(&path) {
                    Ok(project) => {
                        self.projects.insert(project_name, project);
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to load project from {}: {}", path.display(), e);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    pub fn create_action(&mut self, project_name: &str, title: &str, priority: bool) -> Result<Action> {
        // Get or create project
        let project = if let Some(project) = self.projects.get_mut(project_name) {
            project
        } else {
            let project_path = self.root.join(project_name);
            let project = Project::new(project_name.to_string(), &project_path)?;
            self.projects.insert(project_name.to_string(), project);
            self.projects.get_mut(project_name).unwrap()
        };
        
        project.create_action(title, priority)
    }
    
    pub fn get_action(&self, project_name: &str, title: &str) -> Result<&Action> {
        let project = self.projects.get(project_name)
            .with_context(|| format!("Project '{}' not found", project_name))?;
        
        project.get_action(title)
            .with_context(|| format!("Action '{}' not found in project '{}'", title, project_name))
    }
    
    pub fn update_action_status(&mut self, project_name: &str, title: &str, status: Status) -> Result<()> {
        let project = self.projects.get_mut(project_name)
            .with_context(|| format!("Project '{}' not found", project_name))?;
        
        project.update_action_status(title, status)
    }
    
    pub fn set_action_priority(&mut self, project_name: &str, title: &str, priority: bool) -> Result<()> {
        let project = self.projects.get_mut(project_name)
            .with_context(|| format!("Project '{}' not found", project_name))?;
        
        project.set_action_priority(title, priority)
    }
    
    pub fn list_actions(
        &self, 
        project_filter: Option<&str>, 
        status_filter: Option<&str>, 
        priority_only: bool
    ) -> Result<Vec<&Action>> {
        let mut actions = Vec::new();
        
        for (project_name, project) in &self.projects {
            // Filter by project if specified
            if let Some(filter) = project_filter {
                if project_name != filter {
                    continue;
                }
            }
            
            for action in project.list_actions() {
                // Filter by status if specified
                if let Some(status_str) = status_filter {
                    let status = Status::from_str(status_str)?;
                    if action.status() != &status {
                        continue;
                    }
                }
                
                // Filter by priority if specified
                if priority_only && !action.is_priority() {
                    continue;
                }
                
                actions.push(action);
            }
        }
        
        // Sort by project, then by title
        actions.sort_by(|a, b| {
            a.project().cmp(b.project())
                .then_with(|| a.title().cmp(b.title()))
        });
        
        Ok(actions)
    }
    
    pub fn edit_action(&self, project_name: &str, title: &str) -> Result<()> {
        let action = self.get_action(project_name, title)?;
        let file_path = &action.file_path;
        
        // Try to find an editor
        let editor = std::env::var("EDITOR")
            .or_else(|_| std::env::var("VISUAL"))
            .unwrap_or_else(|_| {
                if cfg!(target_os = "windows") {
                    "notepad".to_string()
                } else {
                    "nano".to_string()
                }
            });
        
        let status = Command::new(&editor)
            .arg(file_path)
            .status()
            .with_context(|| format!("Failed to open editor: {}", editor))?;
        
        if !status.success() {
            anyhow::bail!("Editor exited with non-zero status");
        }
        
        Ok(())
    }
    
    pub fn create_meta_graph(&self, project_name: &str, title: &str) -> Result<PathBuf> {
        let project = self.projects.get(project_name)
            .with_context(|| format!("Project '{}' not found", project_name))?;
        
        project.create_meta_graph(title)
    }
    
    pub fn validate(&self) -> Result<()> {
        // Check workspace marker
        let marker_file = self.root.join(".action-lite");
        if !marker_file.exists() {
            anyhow::bail!("Missing .action-lite marker file");
        }
        
        // Validate each project
        for (project_name, project) in &self.projects {
            if !project.path().exists() {
                anyhow::bail!("Project directory does not exist: {}", project.path().display());
            }
            
            // Validate actions in project
            for action in project.list_actions() {
                if !action.file_path.exists() {
                    anyhow::bail!("Action file does not exist: {}", action.file_path.display());
                }
                
                // Validate required outputs for current status
                let required_outputs = action.status().required_outputs();
                for output in required_outputs {
                    match output {
                        "Notes" if action.notes().is_none() => {
                            anyhow::bail!("Action {}/{} missing required Notes section", project_name, action.title());
                        }
                        "Statement of Action" if action.statement_of_action().is_none() => {
                            anyhow::bail!("Action {}/{} missing required Statement of Action section", project_name, action.title());
                        }
                        "Statement of Inputs" if action.statement_of_inputs().is_none() => {
                            anyhow::bail!("Action {}/{} missing required Statement of Inputs section", project_name, action.title());
                        }
                        "Statement of Design" if action.statement_of_design().is_none() => {
                            anyhow::bail!("Action {}/{} missing required Statement of Design section", project_name, action.title());
                        }
                        "Analysis of Impact" if action.analysis_of_impact().is_none() => {
                            anyhow::bail!("Action {}/{} missing required Analysis of Impact section", project_name, action.title());
                        }
                        _ => {}
                    }
                }
            }
        }
        
        Ok(())
    }
    
    pub fn projects(&self) -> &HashMap<String, Project> {
        &self.projects
    }
    
    pub fn get_project(&self, name: &str) -> Option<&Project> {
        self.projects.get(name)
    }
    
    pub fn root(&self) -> &Path {
        &self.root
    }
}