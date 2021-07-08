//! Custom static site generator. Turns Markdown into HTML.
use askama::Template;
use pulldown_cmark::{html, Options, Parser};
use std::env;
use std::error;
use std::path::Path;

// Type alias for result with custom errors determined at runtime (heap)
pub type SsgResult<T> = std::result::Result<T, Box<dyn error::Error>>;

mod config;
use crate::config::{read_config};
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
    let config = read_config().expect("Invalid config: website.yaml!");
    let command = &args[1];
    if command == "feed" {
        print_result(generate_feed(&config.url, &config.title), "RSS feed");
    } else if command == "index" {
        let result = generate_index_page("private/content/index.md");
        print_result(result, "index page");
    } else if command == "page" {
        if args.len() < 3 {
            eprintln!("Missing Markdown path argument!");
            std::process::exit(1);
        }
        let result = generate_content_page(&args[2]);
        print_result(result, &args[2]);
    } else {
        panic!("Unknown command '{}'!", command);
    }
}

fn print_result(result: SsgResult<String>, context: &str) {
    match result {
        Ok(html) => println!("{}", html),
        Err(e) => {
            eprintln!("Error while generating {}:\n{:?}", context, e);
            std::process::exit(1)
        }
    }
}

fn generate_index_page(index_path: &str) -> SsgResult<String> {
    let path = Path::new(index_path);
    let mut index = Page::new(&path)?;
    index.content = generate_index_content()?;
    generate_html(index)
}

fn generate_content_page(content_path: &str) -> SsgResult<String> {
    let md = Path::new(content_path);
    let page = Page::new(&md)?;
    generate_html(page)
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
fn generate_html(page: Page) -> SsgResult<String> {
    // Convert markdown to HTML
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&page.content, options);
    let mut content = String::new();
    html::push_html(&mut content, parser);

    let res = match &page.config.template {
        Some(template) => match &template[..] {
            "default" => render_default(page, &content)?,
            "top" => TopTemplate {
                title: &page.config.title,
                description: &page.config.description,
                link: page.config.link.unwrap_or(Vec::new()),
                content: &content,
            }
            .render()?,
            unknown => panic!("Unknown template {}", unknown),
        },
        None => render_default(page, &content)?,
    };
    Ok(res)
}

fn render_default(page: Page, content: &String) -> Result<String, askama::Error> {
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
}
