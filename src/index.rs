use crate::page::collect_sorted_configs;
use chrono::Datelike;

pub fn generate_index_content() -> String {
    let configs = collect_sorted_configs();
    let mut html: Vec<String> = Vec::new();

    let mut year = 3000;
    for (config, sub_url) in configs {
        let date = config.date.format("%b %d");
        // year
        if config.date.year() < year {
            year = config.date.year();
            let year_h = format!("<h2 class=\"year\">{}</h2>", year);
            html.push(year_h);
        }
        // item
        let item = format!("<a class=\"post\" href=\"/posts/{}\">", sub_url);
        html.push(item);
        // title
        let title = format!("<span class=\"title\">{}</span>", config.title);
        html.push(title);
        // month, day
        let date = date.to_string();
        let date_div = format!("<span class=\"date\">{}</span>", date);
        html.push(date_div);
        html.push("</a>".to_string());
    }
    html.join("\n")
}