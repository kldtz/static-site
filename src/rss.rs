use rss::{ChannelBuilder, GuidBuilder, Item, ItemBuilder};

use crate::page::{collect_sorted_configs, Config};

pub fn generate_feed() {
    let configs: Vec<(Config, String)> = collect_sorted_configs();

    let mut items: Vec<Item> = Vec::new();
    for (c, sub_url) in configs {
        let url = format!("https://proceed-to-decode.com/posts/{}", sub_url);
        let item: Item = ItemBuilder::default()
            .title(c.title)
            .description(c.description.unwrap())
            .link(url.clone())
            .guid(GuidBuilder::default().value(url).build().unwrap())
            .pub_date(c.date.to_rfc2822())
            .build()
            .unwrap();
        items.push(item);
    }

    let channel = ChannelBuilder::default()
        .title("Proceed to Decode")
        .link("https://proceed-to-decode.com/")
        .description("Latest posts")
        .items(items)
        .build()
        .unwrap();

    let string = channel.to_string();
    println!("{}", string);
}
