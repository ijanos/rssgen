use chrono::prelude::*;
use rss::Item as RSSItem;
use serde::Deserialize;

use super::{RSSGenPlugin, RSSGenPluginResult};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct NepszavaArticle {
    title: String,
    link: String,
    public_date: String,
    lead: String,
}

impl From<NepszavaArticle> for RSSItem {
    fn from(article: NepszavaArticle) -> Self {
        RSSItem {
            title: Some(article.title),
            link: Some(format!("https://nepszava.hu/{}", article.link)),
            description: Some(article.lead),
            author: Some("Nádasdy Ádám".to_owned()),
            pub_date: Some(
                NaiveDateTime::parse_from_str(&article.public_date, "%Y.%m.%d. %H:%M")
                    .expect("Error parsing nepszava's date time")
                    .and_utc()
                    .to_rfc2822(),
            ),
            ..Default::default()
        }
    }
}

pub async fn getplugin() -> RSSGenPluginResult {
    let body = reqwest::get("https://nepszava.hu/json/list.json?type_path=szerzo&data_path=nadasdy-adam")
        .await?
        .json::<HashMap<String, Vec<NepszavaArticle>>>()
        .await?;

    let rssitems = body
        .into_values()
        .flatten()
        .map(RSSItem::from)
        .collect::<Vec<RSSItem>>();

    Ok(RSSGenPlugin {
        filename: "nadasdy.rss".to_owned(),
        title: "Nadasdy Adam cikkei".to_owned(),
        description: "Nepszava Nadasdy Adam RSS feed".to_owned(),
        site_url: "https://nepszava.hu".to_owned(),
        items: rssitems,
    })
}
