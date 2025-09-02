use crate::status::Status;
use crate::parser::MarkdownParser;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: Uuid,
    pub title: String,
    pub project: String,
    pub status: Status,
    pub priority: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub file_path: PathBuf,
    
    // Content sections
    pub notes: Option<String>,
    pub statement_of_action: Option<String>,
    pub statement_of_inputs: Option<String>,
    pub statement_of_design: Option<String>,
    pub analysis_of_impact: Option<String>,
}

impl Action {
    pub fn new(project: String, title: String, priority: bool) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            project,
            status: Status::default(),
            priority,
            created_at: now,
            updated_at: now,
            file_path: PathBuf::new(),
            notes: None,
            statement_of_action: None,
            statement_of_inputs: None,
            statement_of_design: None,
            analysis_of_impact: None,
        }
    }
    
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self> {
        let content = fs::read_to_string(&file_path)
            .with_context(|| format!("Failed to read action file: {}", file_path.as_ref().display()))?;
            
        let parser = MarkdownParser::new(&content);
        let (metadata, sections) = parser.parse()?;
        
        let title = file_path.as_ref()
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("untitled")
            .to_string();
            
        let project = file_path.as_ref()
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        Ok(Self {
            id: metadata.get("id")
                .and_then(|s| Uuid::parse_str(s).ok())
                .unwrap_or_else(Uuid::new_v4),
            title,
            project,
            status: metadata.get("status")
                .and_then(|s| Status::from_str(s).ok())
                .unwrap_or_default(),
            priority: metadata.get("priority")
                .map(|s| s.to_lowercase() == "true")
                .unwrap_or(false),
            created_at: metadata.get("created_at")
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now),
            updated_at: metadata.get("updated_at")
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now),
            file_path: file_path.as_ref().to_path_buf(),
            notes: sections.get("Notes").cloned(),
            statement_of_action: sections.get("Statement of Action").cloned(),
            statement_of_inputs: sections.get("Statement of Inputs").cloned(),
            statement_of_design: sections.get("Statement of Design").cloned(),
            analysis_of_impact: sections.get("Analysis of Impact").cloned(),
        })
    }
    
    pub fn save(&mut self) -> Result<()> {
        self.updated_at = Utc::now();
        let content = self.to_markdown();
        fs::write(&self.file_path, content)
            .with_context(|| format!("Failed to write action file: {}", self.file_path.display()))?;
        Ok(())
    }
    
    pub fn to_markdown(&self) -> String {
        let mut content = String::new();
        
        // Metadata
        content.push_str(&format!("---\n"));
        content.push_str(&format!("id: {}\n", self.id));
        content.push_str(&format!("created_at: {}\n", self.created_at.to_rfc3339()));
        content.push_str(&format!("updated_at: {}\n", self.updated_at.to_rfc3339()));
        content.push_str(&format!("---\n\n"));
        
        // Title and tags
        content.push_str(&format!("# {}\n\n", self.title));
        content.push_str(&format!("#project #{} #{}", 
            self.project.replace(" ", "-").to_lowercase(),
            self.status.to_string()
        ));
        
        if self.priority {
            content.push_str(" #priority");
        }
        content.push_str("\n\n");
        
        // Sections
        if let Some(notes) = &self.notes {
            content.push_str("## Notes\n\n");
            content.push_str(notes);
            content.push_str("\n\n");
        }
        
        if let Some(statement) = &self.statement_of_action {
            content.push_str("## Statement of Action\n\n");
            content.push_str(statement);
            content.push_str("\n\n");
        }
        
        if let Some(inputs) = &self.statement_of_inputs {
            content.push_str("## Statement of Inputs\n\n");
            content.push_str(inputs);
            content.push_str("\n\n");
        }
        
        if let Some(design) = &self.statement_of_design {
            content.push_str("## Statement of Design\n\n");
            content.push_str(design);
            content.push_str("\n\n");
        }
        
        if let Some(impact) = &self.analysis_of_impact {
            content.push_str("## Analysis of Impact\n\n");
            content.push_str(impact);
            content.push_str("\n\n");
        }
        
        content
    }
    
    // Getters
    pub fn title(&self) -> &str { &self.title }
    pub fn project(&self) -> &str { &self.project }
    pub fn status(&self) -> &Status { &self.status }
    pub fn is_priority(&self) -> bool { self.priority }
    pub fn notes(&self) -> Option<&String> { self.notes.as_ref() }
    pub fn statement_of_action(&self) -> Option<&String> { self.statement_of_action.as_ref() }
    pub fn statement_of_inputs(&self) -> Option<&String> { self.statement_of_inputs.as_ref() }
    pub fn statement_of_design(&self) -> Option<&String> { self.statement_of_design.as_ref() }
    pub fn analysis_of_impact(&self) -> Option<&String> { self.analysis_of_impact.as_ref() }
    
    // Setters
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
        self.updated_at = Utc::now();
    }
    
    pub fn set_priority(&mut self, priority: bool) {
        self.priority = priority;
        self.updated_at = Utc::now();
    }
    
    pub fn update_section(&mut self, section: &str, content: Option<String>) {
        match section.to_lowercase().as_str() {
            "notes" => self.notes = content,
            "statement of action" => self.statement_of_action = content,
            "statement of inputs" => self.statement_of_inputs = content,
            "statement of design" => self.statement_of_design = content,
            "analysis of impact" => self.analysis_of_impact = content,
            _ => {}
        }
        self.updated_at = Utc::now();
    }
    
    pub fn has_meta_graph(&self) -> bool {
        let meta_graph_path = self.file_path.with_extension("");
        meta_graph_path.exists() && meta_graph_path.is_dir()
    }
    
    pub fn meta_graph_path(&self) -> PathBuf {
        self.file_path.with_extension("")
    }
}