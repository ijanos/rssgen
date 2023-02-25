use clap::Parser;
use futures::future::join_all;
use log::{debug, error, info, warn};
use rusoto_core::Region;
use rusoto_credential::StaticProvider;
use rusoto_s3::S3Client;

use std::future::Future;
use std::pin::Pin;
mod plugin;
use plugin::*;

/// RSS feed generator
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Config {
    /// S3 endpoint
    #[arg(long, env("RSSGEN_S3_ENDPOINT"))]
    s3_endpoint: String,

    /// S3 Bucket name
    #[arg(long, env("RSSGEN_S3_BUCKET"), default_value = "rss")]
    bucket: String,

    /// Skip upload
    #[arg(long)]
    skip_upload: bool,

    /// S3 ACCESS_KEY_ID
    #[arg(long, env("RSSGEN_ACCESS_KEY_ID"))]
    access_key_id: String,

    /// S3 Access key secret
    #[arg(long, env("RSSGEN_SECRET_ACCESS_KEY"))]
    access_key_secret: String,
}

async fn run(skip_upload: bool, p: impl Future<Output = RSSGenPlugin>, s3_client: &S3Client) {
    let p = p.await;
    debug!("{}", p.pretty_string());
    if !skip_upload {
        if p.items.is_empty() {
            warn!("{} skpping upload, no items", p.filename);
        } else {
            info!("{} upload starting, {} items", p.filename, p.items.len());
            match p.upload_to_s3(s3_client, "rss").await {
                Ok(()) => info!("{} upload finished", p.filename),
                Err(e) => error!("{} upload error: {}", p.filename, e),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let config = Config::parse();
    env_logger::init();
    info!("starting up");

    let region = Region::Custom {
        name: "auto".to_owned(),
        endpoint: config.s3_endpoint,
    };

    let creds = StaticProvider::new_minimal(config.access_key_id, config.access_key_secret);
    let http = rusoto_core::HttpClient::new().expect("Could not initalize HTTP client");
    let s3_client = S3Client::new_with(http, creds, region);
    let plugins: Vec<Pin<Box<dyn Future<Output = RSSGenPlugin>>>> = vec![
        Pin::from(Box::new(nepszava::getplugin())),
        Pin::from(Box::new(lobsters::getplugin())),
    ];

    join_all(
        plugins
            .into_iter()
            .map(|p| run(config.skip_upload, p, &s3_client)),
    )
    .await;

    info!("finishing");
}
