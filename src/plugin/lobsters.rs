use rss::{Category, Item as RSSItem};
use serde::Deserialize;

use super::{RSSGenPlugin, RSSGenPluginResult};
use chrono::prelude::*;

#[derive(Deserialize, Debug)]
struct Lobster {
    title: String,
    url: String,
    comments_url: String,
    description: String,
    score: u32,
    created_at: String,
    tags: Vec<String>,
}

impl From<Lobster> for RSSItem {
    fn from(lobster: Lobster) -> Self {
        RSSItem {
            title: Some(lobster.title),
            link: Some(lobster.comments_url.clone()),
            description: Some(format!("<p>Article URL: {}</p><p>Comment URL: {}</p><p>score: {}</p><p>{}</p>", lobster.url, lobster.comments_url, lobster.score, lobster.description)),
            author: Some("Lobster".to_owned()),
            categories: lobster.tags.into_iter().map(category_from_string).collect(),
            pub_date: Some(
                DateTime::parse_from_rfc3339(&lobster.created_at)
                    .unwrap()
                    .to_rfc2822(),
            ),
            ..Default::default()
        }
    }
}

fn category_from_string(from: String) -> Category {
    Category {
        name: from,
        domain: None,
    }
}

pub async fn getplugin() -> RSSGenPluginResult {
    let body = ureq::get("https://lobste.rs/t/linux,video,rust,security,zig.json")
        .call()?
        .into_json::<Vec<Lobster>>()?;

    let rssitems = body
        .into_iter()
        .filter(|l| l.score > 5)
        .map(RSSItem::from)
        .collect::<Vec<RSSItem>>();

    Ok(RSSGenPlugin {
        filename: "lobsters.rss".to_owned(),
        title: "Lobsters".to_owned(),
        description: "Feed for lobsters topic".to_owned(),
        site_url: "https://lobste.rs".to_owned(),
        items: rssitems,
    })
}
