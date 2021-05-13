//! Generation of RSS feed.
use rss::{ChannelBuilder, GuidBuilder, Item, ItemBuilder};

use crate::page::{collect_sorted_configs, Config, SsgResult};

pub fn generate_feed() -> SsgResult<String> {
    let configs: Vec<(Config, String)> = collect_sorted_configs()?;

    let mut items: Vec<Item> = Vec::new();
    for (i, (c, sub_url)) in configs.iter().enumerate() {
        if i > 9 {
            break;
        }
        let url = format!("https://proceed-to-decode.com/posts/{}", sub_url);
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
        .title("Proceed to Decode")
        .link("https://proceed-to-decode.com/")
        .description("Latest posts")
        .items(items)
        .build()?;

    Ok(channel.to_string())
}
