//! Generation of RSS feed.
use rss::{ChannelBuilder, GuidBuilder, Item, ItemBuilder};

use crate::page::{collect_sorted_configs, PageConfig};
use crate::SsgResult;

pub fn generate_feed(url: &str, title: &str) -> SsgResult<String> {
    let configs: Vec<(PageConfig, String)> = collect_sorted_configs()?;

    let mut items: Vec<Item> = Vec::new();
    for (i, (c, sub_url)) in configs.iter().enumerate() {
        if i > 9 {
            break;
        }
        let url = format!("{}/posts/{}", url, sub_url);
        let item: Item = ItemBuilder::default()
            .title(c.title.to_string())
            .description(c.description.clone())
            .link(url.clone())
            .guid(GuidBuilder::default().value(url).build()?)
            .pub_date(c.date.to_rfc2822())
            .build()?;
        items.push(item);
    }

    let channel = ChannelBuilder::default()
        .title(title)
        .link(url)
        .description("Latest posts")
        .items(items)
        .build()?;

    Ok(channel.to_string())
}
