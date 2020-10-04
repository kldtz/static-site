use std::fs;
use std::path::Path;
use glob::glob;
use rss::{Item, ItemBuilder, ChannelBuilder, GuidBuilder};

use crate::page::{Config, find_config};


pub fn generate_feed() {  
    let mut configs: Vec<(Config, String)> = Vec::new();
    for entry in glob("private/content/posts/**/index.md").expect("Failed to read Markdown index files")  {
        let path = entry.unwrap();
        let sub_url = path.to_str().unwrap().split('/').collect::<Vec<&str>>()[3];
        let config: Config = read_config(&path);
        configs.push((config, sub_url.to_string()));
    }
    // sort by date in decreasing order
    configs.sort_by(|c2, c1| c1.0.date.cmp(&c2.0.date));

    let mut items: Vec<Item> = Vec::new();
    for (c, sub_url) in configs  {
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

fn read_config(path: &Path) -> Config {
    let content = fs::read_to_string(path).unwrap();
    let (start, end) = find_config(&content);
    let yaml = &content[start..end];
    let config: Config = serde_yaml::from_str(&yaml).unwrap();
    config
}