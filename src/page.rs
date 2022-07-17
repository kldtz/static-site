//! Generation of single content page.
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use glob::glob;
use lazy_static::lazy_static;
use regex::{CaptureMatches, Captures, Regex, RegexBuilder};
use serde::Deserialize;

use crate::SSGError;

lazy_static! {
    // Image element with SVG source that should be inlined
    static ref IMG: Regex = RegexBuilder::new(r#"<img data-inline="(true|false)" src="(.+?)".*?/>"#)
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
    // Latex math blocks
    static ref MATH: Regex = RegexBuilder::new(r#"\$([^$]+?)\$|\$\$([^$]+?)\$\$"#)
        .dot_matches_new_line(true)
        .build()
        .unwrap();
    static ref KATEX_OPTS_INLINE: katex::Opts = katex::Opts::builder()
        .display_mode(false)
        .build()
        .unwrap();
    static ref KATEX_OPTS_BLOCK: katex::Opts = katex::Opts::builder()
        .display_mode(true)
        .build()
        .unwrap();
}

/// Page read from YAML header and Markdown content.
pub struct Page {
    pub config: PageConfig,
    pub content: String,
}

impl Page {
    pub fn new(path: &Path) -> Result<Page> {
        let content = fs::read_to_string(path)?;
        // Extract and parse yaml header
        let (start, end) = find_config(&content)
            .ok_or_else(|| SSGError(format!("{:?} is missing YAML header.", path)))?;
        let yaml = &content[start..end];
        let config: PageConfig = serde_yaml::from_str(yaml)?;
        // Extract markdown content
        let raw_content = (&content[end + 3..]).to_string();
        // Inline SVGs
        let captures = IMG.captures_iter(&raw_content);
        let static_dir = Path::new("private/static");
        let compute_replacement = |cap| replace_svg(cap, static_dir, path);
        let mut content = replace_all(captures, &raw_content, compute_replacement)?;
        if let Some(features) = &config.features {
            if features.contains(&Feature::Katex) {
                // Render Katex
                let captures = MATH.captures_iter(&raw_content);
                content = replace_all(captures, &content, replace_katex)?;
            }
        }
        Ok(Page { config, content })
    }
}

fn replace_katex(cap: Captures) -> Result<String> {
    Ok(if cap.get(1).is_some() {
        let markup = cap.get(1).unwrap().as_str();
        let opts = katex::Opts::builder()
            .display_mode(false)
            .build()
            .unwrap();
        katex::render_with_opts(markup, opts)?
    } else {
        let markup = cap.get(2).unwrap().as_str();
        let opts = katex::Opts::builder()
            .display_mode(true)
            .build()
            .unwrap();
        katex::render_with_opts(markup, opts)?
    })
}

fn replace_svg(cap: Captures, static_dir: &Path, path: &Path) -> Result<String> {
    let replace_ids = cap.get(1).unwrap();
    let src = cap.get(2).unwrap();
    // construct SVG path specified in src attribute
    let src_str = src.as_str();
    let page_dir = path.parent()
        .ok_or_else(|| SSGError(format!("Path argument {:?} has no parent!", path)))?;
    let svg_path = if src_str.starts_with('/') {
        static_dir.join(src_str)
    } else {
        page_dir.join(src_str)
    };
    // read SVG file
    let svg = fs::read_to_string(svg_path)?;
    // delete the IDs, they might not be unique after inlining
    let svg = if replace_ids.as_str() == "true" {
        ID.replace_all(&svg, "").to_string()
    } else {
        svg
    };
    // we only inline the SVG (assume there is one per file)
    let svg_match = SVG
        .find(&svg)
        .ok_or_else(|| SSGError(format!("{} does not contain any SVG element.", src_str)))?;
    Ok(String::from(&svg[svg_match.range()]))
}

fn replace_all<'a>(
    captures: CaptureMatches<'_, 'a>,
    raw_content: &'a str,
    compute_replacement: impl Fn(Captures<'a>) -> Result<String>,
) -> Result<String> {
    let mut content = String::new();
    let mut last_offset = 0;
    for cap in captures {
        let full_match = cap.get(0).unwrap();
        content.push_str(&raw_content[last_offset..full_match.start()]);
        let replacement = compute_replacement(cap)?;
        content.push_str(&replacement);
        last_offset = full_match.end();
    }
    content.push_str(&raw_content[last_offset..]);
    Ok(content)
}

/// Metadata for page generation.
#[derive(Deserialize, Debug)]
pub struct PageConfig {
    pub title: String,
    pub date: DateTime<Utc>,
    pub description: Option<String>,
    pub template: Option<String>,
    pub features: Option<Vec<Feature>>,
    pub scripts: Option<Vec<String>>,
    pub link: Option<Vec<String>>,
    pub language: Option<String>,
}

/// Optional features used by the page.
#[derive(Deserialize, Debug, PartialEq)]
pub enum Feature {
    Katex,
    MathJax,
    Highlight,
}

pub fn find_config(content: &str) -> Option<(usize, usize)> {
    let mut iter = content.match_indices("---");
    let (start, _) = iter.next()?;
    let (end, _) = iter.next()?;
    Some((start + 3, end))
}

pub fn collect_sorted_configs() -> Result<Vec<(PageConfig, String)>> {
    let mut configs: Vec<(PageConfig, String)> = Vec::new();
    for entry in
    glob("private/content/posts/**/index.md").context("Failed to read Markdown index files")?
    {
        let path = entry?;
        let sub_url = path
            .strip_prefix("private/content/posts/")?
            .parent()
            .ok_or_else(|| SSGError(format!("Error constructing relative path for {:?}", path)))?;
        let config: PageConfig = read_config(&path)?;
        configs.push((config, sub_url.display().to_string()));
    }
    // sort by date in decreasing order
    configs.sort_by(|c2, c1| c1.0.date.cmp(&c2.0.date));
    Ok(configs)
}

fn read_config(path: &Path) -> Result<PageConfig> {
    let content = fs::read_to_string(path)?;
    let (start, end) =
        find_config(&content)
            .ok_or_else(|| SSGError(format!("{:?} is missing YAML header.", path)))?;
    let yaml = &content[start..end];
    let config: PageConfig = serde_yaml::from_str(yaml)?;
    Ok(config)
}
