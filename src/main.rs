//! Custom static site generator. Turns Markdown into HTML.
use std::env;
use std::fmt::Display;
use std::path::Path;

use anyhow::{bail, Context, Result};
use askama::Template;
use pulldown_cmark::{html, Options, Parser};

use crate::config::{Config, read_config};
use crate::index::generate_index_content;
use crate::page::{Feature, Page};
use crate::rss::generate_feed;

mod config;
mod page;
mod rss;
mod index;

#[derive(Debug)]
pub struct SSGError(String);

impl std::error::Error for SSGError {}

impl Display for SSGError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Too few arguments!");
    }
    let config = read_config().context("Invalid config: website.yaml!")?;
    println!("{}", generate_output(config, &args)?);
    Ok(())
}

fn generate_output(config: Config, args: &[String]) -> Result<String> {
    let command = &args[1];
    match command.as_str() {
        "feed" => generate_feed(&config.url, &config.title)
            .context("Could not generate feed!"),
        "index" => generate_index_page("private/content/index.md")
            .context("Could not generate index page!"),
        "page" => {
            if args.len() < 3 {
                eprintln!("Missing Markdown path argument!");
                std::process::exit(1);
            }
            generate_content_page(&args[2])
                .with_context(|| format!("Could not generate page {:?}", &args[2]))
        }
        _ => bail!(SSGError(format!("Unknown command '{}'!", command)))
    }
}

fn generate_index_page(index_path: &str) -> Result<String> {
    let path = Path::new(index_path);
    let mut index = Page::new(path)?;
    index.content = generate_index_content()?;
    generate_html(index)
}

fn generate_content_page(content_path: &str) -> Result<String> {
    let md = Path::new(content_path);
    let page = Page::new(md)?;
    generate_html(page)
}

#[derive(Template)]
#[template(path = "default.html", escape = "none")]
struct DefaultTemplate<'a> {
    title: &'a str,
    language: &'a str,
    date: &'a str,
    description: &'a Option<String>,
    mathjax: bool,
    highlight: bool,
    scripts: Vec<String>,
    link: Vec<String>,
    content: &'a str,
}

impl DefaultTemplate<'_> {
    fn render(page: Page, content: &str) -> Result<String, askama::Error> {
        let features = page.config.features.unwrap_or_default();
        DefaultTemplate {
            title: &page.config.title,
            language: &page.config.language.unwrap_or_else(|| "en".to_string()),
            date: &page.config.date.format("%b %e, %Y").to_string(),
            description: &page.config.description,
            mathjax: features.iter().any(|f| f == &Feature::MathJax),
            highlight: features.iter().any(|f| f == &Feature::Highlight),
            scripts: page.config.scripts.unwrap_or_default(),
            link: page.config.link.unwrap_or_default(),
            content,
        }
            .render()
    }
}

#[derive(Template)]
#[template(path = "top.html", escape = "none")]
struct TopTemplate<'a> {
    title: &'a str,
    description: &'a Option<String>,
    link: Vec<String>,
    content: &'a str,
}

impl TopTemplate<'_> {
    fn render(page: Page, content: &str) -> Result<String, askama::Error> {
        TopTemplate {
            title: &page.config.title,
            description: &page.config.description,
            link: page.config.link.unwrap_or_default(),
            content,
        }
            .render()
    }
}

/// Generates HTML from Page struct.
fn generate_html(page: Page) -> Result<String> {
    // Convert markdown to HTML
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&page.content, options);
    let mut content = String::new();
    html::push_html(&mut content, parser);

    Ok(match &page.config.template {
        Some(template) => match &template[..] {
            "top" => TopTemplate::render(page, &content)?,
            unknown => bail!(SSGError(format!("Unknown template {}", unknown))),
        },
        _ => DefaultTemplate::render(page, &content)?,
    })
}

