use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

use super::{database::database_endpoint::DatabaseEndpoint, page::page_endpoint::PageEndpoint};

pub struct NotionClient {
    pub database: DatabaseEndpoint,
    pub page: PageEndpoint,
}

impl NotionClient {
    pub fn new(token: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
        headers.insert(
            "Notion-Version",
            HeaderValue::from_str("2022-06-28").unwrap(),
        );

        let base_url = "https://api.notion.com/v1";
        let client = Client::builder().default_headers(headers).build().unwrap();
        let database = DatabaseEndpoint::new(client.clone(), base_url);
        let page = PageEndpoint::new(client.clone(), base_url);

        Self { database, page }
    }
}
