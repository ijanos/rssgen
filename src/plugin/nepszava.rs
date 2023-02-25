use chrono::prelude::*;
use rss::Item as RSSItem;
use serde::Deserialize;

use super::RSSGenPlugin;
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
                Utc.datetime_from_str(&article.public_date, "%Y.%m.%d. %H:%M")
                    .unwrap()
                    .to_rfc2822(),
            ),
            ..Default::default()
        }
    }
}

pub async fn getplugin() -> RSSGenPlugin {
    let body =
        ureq::get("https://nepszava.hu/json/list.json?type_path=szerzo&data_path=nadasdy-adam")
            .call()
            .expect("Couldn't GET artictles")
            .into_json::<HashMap<String, Vec<NepszavaArticle>>>()
            .expect("Couldn't convert response to JSON");

    let rssitems = body
        .into_values()
        .flatten()
        .into_iter()
        .map(RSSItem::from)
        .collect::<Vec<RSSItem>>();

    RSSGenPlugin {
        filename: "nadasdy.rss".to_owned(),
        title: "Nadasdy Adam cikkei".to_owned(),
        description: "Nepszava Nadasdy Adam RSS feed".to_owned(),
        site_url: "https://nepszava.hu".to_owned(),
        items: rssitems,
    }
}
