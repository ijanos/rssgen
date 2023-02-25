use rss::{Category, Item as RSSItem};
use serde::Deserialize;

use super::RSSGenPlugin;
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
            link: Some(lobster.comments_url),
            description: Some(format!("{}\n{}", lobster.url, lobster.description)),
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

// https://lobste.rs/t/linux,video,rust,security.json?page=2
pub async fn getplugin() -> RSSGenPlugin {
    let body = ureq::get("https://lobste.rs/t/linux,video,rust,security,zig.json")
        .call()
        .expect("Couldn't GET artictles")
        .into_json::<Vec<Lobster>>()
        .expect("Couldn't convert response to JSON");

    let rssitems = body
        .into_iter()
        .filter(|l| l.score > 5)
        .map(RSSItem::from)
        .collect::<Vec<RSSItem>>();

    RSSGenPlugin {
        filename: "lobsters.rss".to_owned(),
        title: "Lobsters".to_owned(),
        description: "Feed for lobsters topic".to_owned(),
        site_url: "https://lobste.rs".to_owned(),
        items: rssitems,
    }
}
