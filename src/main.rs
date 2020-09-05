//! Quick and dirty script for generating HTML from Markdown file.

use chrono::{DateTime, Utc};
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use serde::Deserialize;
use std::env;
use std::fs;


macro_rules! needs_feature {
    ($page: expr, $feature: pat) => {
        match &$page.config.features {
            Some(features) => features.iter().any(|s| match s {
                $feature => true,
                _ => false,
            }),
            _ => false,
        }
    }
}

/// Config read from YAML header and Markdown content.
struct Page {
    config: Config,
    content: String,
}

impl Page {
    fn new(path: &str) -> Page {
        let content = fs::read_to_string(path).unwrap();

        // Extract and parse yaml header
        let mut iter = content.match_indices("---");
        let (start, _) = iter.next().unwrap();
        let (end, _) = iter.next().unwrap();

        let yaml = &content[start + 3..end];
        let config: Config = serde_yaml::from_str(&yaml).unwrap();
        // Extract markdown content
        let content = (&content[end + 3..]).to_string();

        Page { config, content }
    }
}

/// Metadata for page generation.
#[derive(Deserialize, Debug)]
struct Config {
    title: String,
    date: DateTime<Utc>,
    template: Option<String>,
    features: Option<Vec<Feature>>,
    scripts: Option<Vec<String>>,
    link: Option<Vec<String>>,
}

/// Optional features used by the page.
#[derive(Deserialize, Debug)]
enum Feature {
    MathJax,
    Highlight,
}

impl Page {
    fn template_name(&self) -> &str {
        match &self.config.template {
            Some(name) => name,
            _ => "default",
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Wrong number of arguments!");
    }

    let md = &args[1];
    let page = Page::new(md);
    let html = generate_html(page);
    println!("{}", html);
}

/// Generates HTML from Page struct.
fn generate_html(page: Page) -> String {
    // Read specified template (or default)
    let template = fs::read_to_string(format!("private/templates/{}.html", page.template_name())).unwrap();

    // Get formatted date
    let date = page.config.date.format("%b %e, %Y");
    let date = date.to_string();

    // Load MathJax snippet if requested
    let mathjax = if needs_feature!(&page, Feature::MathJax) {
        fs::read_to_string("private/snips/mathjax.html").unwrap()
    } else {
        "".to_string()
    };

    // Load highlight snippet if requested
    let highlight = if needs_feature!(&page, Feature::Highlight) {
        fs::read_to_string("private/snips/highlight.html").unwrap()
    } else {
        String::from("")
    };

    // Generate script elements
    let scripts = match page.config.scripts {
        Some(scripts) => scripts.iter().map(|src| format!("<script src=\"{}\"></script>", src)).collect::<Vec<String>>().join("\n"),
        _ => String::from("")
    };

    // Generate link elements
    let link = match page.config.link {
        Some(link) => link.iter().map(|l| format!("<link rel=\"stylesheet\" href=\"{}\">", l)).collect::<Vec<String>>().join("\n"),
        _ => "".to_string(),
    };

    // Convert markdown to HTML
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&page.content, options);
    let mut content = String::new();
    html::push_html(&mut content, parser);

    // Replace variables in template
    let re = Regex::new(r"\{\{(.+?)\}\}").unwrap();
    let mut parts: Vec<&str> = Vec::new();
    let mut offset: usize = 0;
    for m in re.find_iter(&template) {
        if m.start() > offset {
            parts.push(&template[offset..m.start()]);
        }
        let match_str = m.as_str();

        let replacement = match &match_str[2..match_str.len() - 2] {
            "title" => &page.config.title,
            "date" => &date,
            "content" => &content,
            "highlight" => &highlight,
            "mathjax" => &mathjax,
            "scripts" => &scripts,
            "link" => &link,
            _ => "",
        };
        parts.push(replacement);
        offset = m.end();
    }
    if offset < template.len() {
        parts.push(&template[offset..]);
    }

    parts.join("")
}
