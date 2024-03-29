//! Generation of index page listing all posts in reverse chronological order.
use anyhow::Result;
use chrono::Datelike;

use crate::page::collect_sorted_configs;

pub fn generate_index_content() -> Result<String> {
    let configs = collect_sorted_configs()?;
    let mut html = String::new();
    let mut year = configs[0].0.date.year();
    html.push_str(&format!("<fieldset class=\"year\"><legend>{}</legend>", year));
    for (config, sub_url) in configs {
        let date = config.date.format("%b %d");
        // year
        if config.date.year() < year {
            year = config.date.year();
            html.push_str("</fieldset>\n");
            html.push_str(&format!("<fieldset class=\"year\"><legend>{}</legend>", year));
        }
        // item
        html.push_str(&format!("<a class=\"post\" href=\"/posts/{}\">", sub_url));
        // title
        html.push_str(&format!("<span class=\"title\">{}</span>", config.title));
        // month, day
        let date = date.to_string();
        html.push_str(&format!("<span class=\"date\">{}</span>", date));
        html.push_str("</a>");
    }
    html.push_str("</fieldset>");
    Ok(html)
}