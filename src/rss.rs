//! Generation of RSS feed.
use anyhow::Result;
use askama::Template;

use crate::page::{collect_sorted_configs, PageConfig};

struct ChannelItem<'a> {
    title: &'a str,
    description: &'a Option<String>,
    sub_url: &'a str,
    date: String,
}

#[derive(Template)]
#[template(path = "rss_template.xml", escape = "none")]
struct RssTemplate<'a> {
    title: &'a str,
    url: &'a str,
    items: Vec<ChannelItem<'a>>,
}

pub fn generate_feed(url: &str, title: &str) -> Result<String> {
    let configs: Vec<(PageConfig, String)> = collect_sorted_configs()?;

    let mut items: Vec<ChannelItem> = Vec::new();
    for (i, (c, sub_url)) in configs.iter().enumerate() {
        if i > 9 {
            break;
        }
        let item = ChannelItem {
            title: &c.title,
            description: &c.description,
            sub_url,
            date: c.date.to_rfc2822(),
        };
        items.push(item);
    }
    let rss = RssTemplate { title, url, items }.render()?;
    Ok(rss)
}
