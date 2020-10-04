//! Quick and dirty script for generating HTML from Markdown file.
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use std::env;
use std::fs;

mod page;
use crate::page::{Page, Feature};
mod rss;
use crate::rss::{generate_feed};

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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Too few arguments!");
    }
    let command = &args[1];
    if command == "feed" {
        generate_feed();
    } else if command == "page" {
        if args.len() < 3 {
            panic!("Missing Markdown path argument!");
        }
        let md = &args[2];
        let page = Page::new(md);
        let html = generate_html(page);
        println!("{}", html);
    } else {
        panic!("Unknown command '{}'!", command);
    }
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
        String::from("")
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
        _ => String::from(""),
    };

    // Generate meta description
    let description = match page.config.description {
        Some(description) => format!("<meta name=\"description\" content=\"{}\">", description),
        _ => String::from(""),
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
            "description" => &description,
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
