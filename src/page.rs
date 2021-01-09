use chrono::{DateTime, Utc};
use glob::glob;
use serde::Deserialize;
use std::fs;
use std::path::Path;

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
#[derive(Deserialize, Debug, PartialEq)]
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

pub fn collect_sorted_configs() -> Vec<(Config, String)> {
    let mut configs: Vec<(Config, String)> = Vec::new();
    for entry in
        glob("private/content/posts/**/index.md").expect("Failed to read Markdown index files")
    {
        let path = entry.unwrap();
        let sub_url = path.to_str().unwrap().split('/').collect::<Vec<&str>>()[3];
        let config: Config = read_config(&path);
        configs.push((config, sub_url.to_string()));
    }
    // sort by date in decreasing order
    configs.sort_by(|c2, c1| c1.0.date.cmp(&c2.0.date));
    configs
}

fn read_config(path: &Path) -> Config {
    let content = fs::read_to_string(path).unwrap();
    let (start, end) = find_config(&content);
    let yaml = &content[start..end];
    let config: Config = serde_yaml::from_str(&yaml).unwrap();
    config
}
