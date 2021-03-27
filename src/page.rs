use chrono::{DateTime, Utc};
use glob::glob;
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use serde::Deserialize;
use std::path::Path;
use std::{error, fs};

// Type alias for result with custom errors determined at runtime (heap)
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

lazy_static! {
    // Image element with SVG source that should be inlined
    static ref IMG: Regex = RegexBuilder::new(r#"<img data-inline="true" src="(.+?)".*?/>"#)
        .dot_matches_new_line(true)
        .build()
        .unwrap();
    // SVG element
    static ref SVG: Regex = RegexBuilder::new(r"<svg.*?</svg>")
        .dot_matches_new_line(true)
        .build()
        .unwrap();
    // HTML id attribute
    static ref ID: Regex = Regex::new(r#"id=".*?""#).unwrap();
}

/// Page read from YAML header and Markdown content.
pub struct Page {
    pub config: Config,
    pub content: String,
}

impl Page {
    pub fn new(path: &str) -> Result<Page> {
        let content = fs::read_to_string(path)?;
        // Extract and parse yaml header
        let (start, end) = find_config(&content);
        let yaml = &content[start..end];
        let config: Config = serde_yaml::from_str(&yaml)?;
        // Extract markdown content
        let raw_content = (&content[end + 3..]).to_string();
        let content = preprocess_markdown(&raw_content)?;
        Ok(Page { config, content })
    }
}

/// Inlines SVGs for proper colors in dark mode.
fn preprocess_markdown(raw_content: &str) -> Result<String> {
    let mut content = String::new();
    let mut last_offset = 0;
    for cap in IMG.captures_iter(&raw_content) {
        let svg = fs::read_to_string(format!(
            "private/static{}",
            cap.get(1).map_or("", |m| m.as_str())
        ))?;
        let svg = ID.replace_all(&svg, "");
        let svg_match = SVG.find(&svg).unwrap();
        let full_match = cap.get(0).unwrap();
        content.push_str(&raw_content[last_offset..full_match.start()]);
        content.push_str(&svg[svg_match.range()]);
        last_offset = full_match.end();
    }
    content.push_str(&raw_content[last_offset..]);
    Ok(content)
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
        let sub_url = path
            .strip_prefix("private/content/posts/")
            .unwrap()
            .parent()
            .unwrap();
        let config: Config = read_config(&path);
        configs.push((config, sub_url.display().to_string()));
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
