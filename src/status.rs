use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Discovery,
    Design,
    Implement,
    Test,
    Document,
    Publish,
    Published,
}

#[derive(Error, Debug)]
pub enum StatusError {
    #[error("Invalid status: {0}. Valid statuses are: discovery, design, implement, test, document, publish, published")]
    InvalidStatus(String),
}

impl Status {
    pub fn from_str(s: &str) -> Result<Self, StatusError> {
        match s.to_lowercase().as_str() {
            "discovery" => Ok(Status::Discovery),
            "design" => Ok(Status::Design),
            "implement" => Ok(Status::Implement),
            "test" => Ok(Status::Test),
            "document" => Ok(Status::Document),
            "publish" => Ok(Status::Publish),
            "published" => Ok(Status::Published),
            _ => Err(StatusError::InvalidStatus(s.to_string())),
        }
    }
    
    pub fn as_tag(&self) -> String {
        format!("#{}", self.to_string().to_lowercase())
    }
    
    pub fn next_status(&self) -> Option<Status> {
        match self {
            Status::Discovery => Some(Status::Design),
            Status::Design => Some(Status::Implement),
            Status::Implement => Some(Status::Test),
            Status::Test => Some(Status::Document),
            Status::Document => Some(Status::Publish),
            Status::Publish => Some(Status::Published),
            Status::Published => None,
        }
    }
    
    pub fn previous_status(&self) -> Option<Status> {
        match self {
            Status::Discovery => None,
            Status::Design => Some(Status::Discovery),
            Status::Implement => Some(Status::Design),
            Status::Test => Some(Status::Implement),
            Status::Document => Some(Status::Test),
            Status::Publish => Some(Status::Document),
            Status::Published => Some(Status::Publish),
        }
    }
    
    pub fn required_outputs(&self) -> Vec<&'static str> {
        match self {
            Status::Discovery => vec![
                "Title", 
                "Notes", 
                "Statement of Action", 
                "Statement of Inputs", 
                "Statement of Specifications"
            ],
            Status::Design => vec!["Statement of Design"],
            Status::Implement => vec![],
            Status::Test => vec![],
            Status::Document => vec!["Analysis of Impact"],
            Status::Publish => vec![],
            Status::Published => vec![],
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Discovery => write!(f, "discovery"),
            Status::Design => write!(f, "design"),
            Status::Implement => write!(f, "implement"),
            Status::Test => write!(f, "test"),
            Status::Document => write!(f, "document"),
            Status::Publish => write!(f, "publish"),
            Status::Published => write!(f, "published"),
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::Discovery
    }
}