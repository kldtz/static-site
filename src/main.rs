//! Custom static site generator. Turns Markdown into HTML.
use pulldown_cmark::{html, Options, Parser};
use std::env;
use std::path::Path;

use askama::Template;

mod page;
use crate::page::{Feature, Page};
mod rss;
use crate::rss::generate_feed;
mod index;
use crate::index::generate_index_content;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Too few arguments!");
    }
    let command = &args[1];
    if command == "feed" {
        generate_feed();
    } else if command == "index" {
        let path = Path::new("private/content/index.md");
        let mut index = Page::new(&path).unwrap();
        index.content = generate_index_content();
        let html = generate_html(index);
        println!("{}", html);
    } else if command == "page" {
        if args.len() < 3 {
            panic!("Missing Markdown path argument!");
        }
        let md = Path::new(&args[2]);
        let page = Page::new(&md).unwrap();
        let html = generate_html(page);
        println!("{}", html);
    } else {
        panic!("Unknown command '{}'!", command);
    }
}

#[derive(Template)]
#[template(path = "default.html", escape = "none")]
struct DefaultTemplate<'a> {
    title: &'a str,
    date: &'a str,
    description: &'a Option<String>,
    mathjax: bool,
    highlight: bool,
    scripts: Vec<String>,
    link: Vec<String>,
    content: &'a str,
}

#[derive(Template)]
#[template(path = "top.html", escape = "none")]
struct TopTemplate<'a> {
    title: &'a str,
    description: &'a Option<String>,
    link: Vec<String>,
    content: &'a str,
}

/// Generates HTML from Page struct.
fn generate_html(page: Page) -> String {
    // Convert markdown to HTML
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&page.content, options);
    let mut content = String::new();
    html::push_html(&mut content, parser);

    match &page.config.template {
        Some(template) => match &template[..] {
            "default" => render_default(page, &content),
            "top" => TopTemplate {
                title: &page.config.title,
                description: &page.config.description,
                link: page.config.link.unwrap_or(Vec::new()),
                content: &content,
            }
            .render()
            .unwrap(),
            unknown => panic!("Unknown template {}", unknown),
        },
        None => render_default(page, &content)
    }
}

fn render_default(page: Page, content: &String) -> String {
    let features = page.config.features.unwrap_or(Vec::new());
    DefaultTemplate {
        title: &page.config.title,
        date: &page.config.date.format("%b %e, %Y").to_string(),
        description: &page.config.description,
        mathjax: features.iter().any(|f| f == &Feature::MathJax),
        highlight: features.iter().any(|f| f == &Feature::Highlight),
        scripts: page.config.scripts.unwrap_or(Vec::new()),
        link: page.config.link.unwrap_or(Vec::new()),
        content: &content,
    }
    .render()
    .unwrap()
}