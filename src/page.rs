use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::fs;

/// Page read from YAML header and Markdown content.
pub struct Page {
    pub config: Config,
    pub content: String,
}

impl Page {
    pub fn new(path: &str) -> Page {
        let content = fs::read_to_string(path).unwrap();

        // Extract and parse yaml header
        let (start, end) = find_config(&content);
        let yaml = &content[start..end];
        let config: Config = serde_yaml::from_str(&yaml).unwrap();
        // Extract markdown content
        let content = (&content[end + 3..]).to_string();

        Page { config, content }
    }

    pub fn template_name(&self) -> &str {
        match &self.config.template {
            Some(name) => name,
            _ => "default",
        }
    }
}

/// Metadata for page generation.
#[derive(Deserialize, Debug)]
pub struct Config {
    pub title: String,
    pub date: DateTime<Utc>,
    pub description: Option<String>,
    pub template: Option<String>,
    pub features: Option<Vec<Feature>>,
    pub scripts: Option<Vec<String>>,
    pub link: Option<Vec<String>>,
}

/// Optional features used by the page.
#[derive(Deserialize, Debug)]
pub enum Feature {
    MathJax,
    Highlight,
}

pub fn find_config(content: &str) -> (usize, usize) {
    let mut iter = content.match_indices("---");
    let (start, _) = iter.next().unwrap();
    let (end, _) = iter.next().unwrap();
    (start + 3, end)
}