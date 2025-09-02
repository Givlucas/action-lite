use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashMap;

pub struct MarkdownParser<'a> {
    content: &'a str,
}

impl<'a> MarkdownParser<'a> {
    pub fn new(content: &'a str) -> Self {
        Self { content }
    }
    
    pub fn parse(&self) -> Result<(HashMap<String, String>, HashMap<String, String>)> {
        let mut metadata = HashMap::new();
        let mut sections = HashMap::new();
        
        let lines: Vec<&str> = self.content.lines().collect();
        let mut i = 0;
        
        // Parse frontmatter if present
        if i < lines.len() && lines[i].trim() == "---" {
            i += 1;
            while i < lines.len() && lines[i].trim() != "---" {
                let line = lines[i].trim();
                if let Some((key, value)) = line.split_once(':') {
                    metadata.insert(
                        key.trim().to_string(), 
                        value.trim().to_string()
                    );
                }
                i += 1;
            }
            if i < lines.len() {
                i += 1; // Skip closing ---
            }
        }
        
        // Parse sections
        let mut current_section: Option<String> = None;
        let mut section_content = String::new();
        
        while i < lines.len() {
            let line = lines[i];
            
            // Check for section headers (## Section Name)
            if line.starts_with("## ") {
                // Save previous section if exists
                if let Some(section_name) = current_section.take() {
                    sections.insert(section_name, section_content.trim().to_string());
                    section_content.clear();
                }
                
                // Start new section
                current_section = Some(line[3..].trim().to_string());
            } else if current_section.is_some() {
                // Add line to current section
                section_content.push_str(line);
                section_content.push('\n');
            }
            
            i += 1;
        }
        
        // Save last section
        if let Some(section_name) = current_section {
            sections.insert(section_name, section_content.trim().to_string());
        }
        
        Ok((metadata, sections))
    }
    
    pub fn extract_tags(&self) -> Vec<String> {
        let tag_regex = Regex::new(r"#([a-zA-Z0-9_-]+)").unwrap();
        tag_regex
            .captures_iter(self.content)
            .map(|cap| cap[1].to_string())
            .collect()
    }
    
    pub fn extract_links(&self) -> Vec<String> {
        let link_regex = Regex::new(r"\[([^\]]+)\]\(([^)]+\.md)\)").unwrap();
        link_regex
            .captures_iter(self.content)
            .map(|cap| cap[2].to_string())
            .collect()
    }
}