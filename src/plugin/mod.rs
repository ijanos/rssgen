pub mod lobsters;
pub mod nepszava;

use rss::{Channel, ChannelBuilder, Item as RSSItem};
use rusoto_s3::{PutObjectRequest, S3Client, S3};

pub type RSSGenPluginResult = Result<RSSGenPlugin, Box<dyn std::error::Error>>;

pub struct RSSGenPlugin {
    title: String,
    description: String,
    pub filename: String,
    site_url: String,
    pub items: Vec<RSSItem>,
}

impl RSSGenPlugin {
    fn gen_feed(&self) -> Channel {
        ChannelBuilder::default()
            .title(self.title.clone())
            .link(self.site_url.clone())
            .description(self.description.clone())
            .items(self.items.clone())
            .ttl(Some("40".to_owned()))
            .build()
    }

    pub async fn upload_to_s3(
        &self,
        s3_client: &S3Client,
        bucket: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        s3_client
            .put_object(PutObjectRequest {
                bucket: bucket.to_owned(),
                key: self.filename.to_owned(),
                body: Some(self.gen_feed().to_string().as_bytes().to_owned().into()),
                content_type: Some("application/rss+xml".to_owned()),
                ..Default::default()
            })
            .await?;
        Ok(())
    }

    pub fn pretty_string(&self) -> String {
        let mut buf = Vec::<u8>::new();

        self.gen_feed().pretty_write_to(&mut buf, b' ', 2).unwrap();
        String::from_utf8_lossy(&buf).into_owned()
    }
}
