use crate::Status;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct ActionTemplate {
    pub id: Uuid,
    pub title: String,
    pub project: String,
    pub status: Status,
    pub priority: bool,
    pub created_at: DateTime<Utc>,
}

impl ActionTemplate {
    pub fn new(project: String, title: String, priority: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            project,
            status: Status::Discovery,
            priority,
            created_at: Utc::now(),
        }
    }
    
    pub fn to_markdown(&self) -> String {
        let mut content = String::new();
        
        // Frontmatter
        content.push_str("---\n");
        content.push_str(&format!("id: {}\n", self.id));
        content.push_str(&format!("created_at: {}\n", self.created_at.to_rfc3339()));
        content.push_str(&format!("updated_at: {}\n", self.created_at.to_rfc3339()));
        content.push_str("---\n\n");
        
        // Title and tags
        content.push_str(&format!("# {}\n\n", self.title));
        content.push_str(&format!("#project #action #{} #{}", 
            self.status.to_string(),
            self.project.replace(" ", "-").to_lowercase()
        ));
        
        if self.priority {
            content.push_str(" #priority");
        }
        content.push_str("\n\n");
        
        // Template sections based on discovery status
        content.push_str("## Notes\n\n");
        content.push_str("General notes on the task\n\n");
        
        content.push_str("## Statement of Action\n\n");
        content.push_str("The task to be performed, more in depth than title, may include why the action is needed\n\n");
        
        content.push_str("## Statement of Inputs\n\n");
        content.push_str("A list of .md links to other markdown files:\n\n");
        content.push_str("- [Related Action](../other-project/related-action.md)\n\n");
        
        // Only include design section template if not in discovery
        if self.status != Status::Discovery {
            content.push_str("## Statement of Design\n\n");
            content.push_str("### Output\n\n");
            content.push_str("The action produced by the design\n\n");
            content.push_str("### Design\n\n");
            content.push_str("A detailed design for how to proceed\n\n");
        }
        
        // Only include impact analysis if in document stage or later
        if matches!(self.status, Status::Document | Status::Publish | Status::Published) {
            content.push_str("## Analysis of Impact\n\n");
            content.push_str("Analysis of the impact and outcomes of this action\n\n");
        }
        
        content
    }
}

pub fn get_workspace_readme() -> &'static str {
    include_str!("../templates/workspace_readme.md")
}

pub fn get_project_readme(project_name: &str) -> String {
    format!(
        "# {}\n\nThis is a project directory in the Action Lite workspace.\n\n## Actions\n\nActions for this project are stored as markdown files in this directory.\n\n## Meta-graphs\n\nMeta-graph directories (if any) are stored alongside their corresponding action files.\n",
        project_name
    )
}