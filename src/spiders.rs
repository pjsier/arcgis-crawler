use anyhow::{self, Result};
use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;
use url::Url;

use crate::nodes::{ArcgisResponse, ServerNode};

#[async_trait]
pub trait Spider: Send + Sync {
    fn start_urls(&self) -> Vec<String>;
    async fn scrape(&self, url: String) -> Result<(Vec<ServerNode>, Vec<String>)>;
    async fn process(&self, item: ServerNode) -> Result<()>;
}

pub struct ArcgisSpider {
    base_url: String,
    http_client: Client,
}

impl ArcgisSpider {
    pub fn new(base_url: String) -> Self {
        let http_timeout = Duration::from_secs(30);
        let http_client = Client::builder()
            .timeout(http_timeout)
            .user_agent(
                "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0",
            )
            .build()
            .expect("spiders/arcgis: Building HTTP client");
        Self {
            base_url,
            http_client,
        }
    }
}

#[async_trait]
impl Spider for ArcgisSpider {
    fn start_urls(&self) -> Vec<String> {
        vec![self.base_url.clone()]
    }

    async fn scrape(&self, url: String) -> Result<(Vec<ServerNode>, Vec<String>)> {
        let url = Url::parse_with_params(&url, &[("f", "pjson")])?;

        let base_segments_count: usize = Url::parse(&self.base_url)?
            .path_segments()
            .map_or(0, |v| v.filter(|s| s != &"").count());

        // Skip base segments in generating path for nodes
        let path_segments: Vec<String> = url.path_segments().map_or(vec![], |v| {
            v.skip(base_segments_count)
                .map(String::from)
                .collect::<Vec<String>>()
        });

        let res: ArcgisResponse = self
            .http_client
            .get(url.clone())
            .send()
            .await?
            .json()
            .await?;

        let mut urls: Vec<String> = res
            .folders
            .into_iter()
            .filter_map(|folder| url.clone().join(&folder).ok())
            .map(|u| u.to_string())
            .collect();

        let server_urls: Vec<String> = res
            .services
            .into_iter()
            .filter_map(|service| {
                url.clone()
                    .join(&format!("{}/{}", service.name, service.type_))
                    .ok()
            })
            .map(|u| u.to_string())
            .collect();

        urls.extend(server_urls);

        let nodes: Vec<ServerNode> = res
            .layers
            .into_iter()
            .map(|s| {
                let mut path = path_segments.clone();
                path.push(s.name);
                ServerNode(path)
            })
            .collect();

        Ok((nodes, urls))
    }

    async fn process(&self, item: ServerNode) -> Result<()> {
        println!("{:?}", item);
        Ok(())
    }
}
