// create a new client instance reqwest
// and make a request to the server
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::de::DeserializeOwned;

pub struct NotionClient {
    client: reqwest::Client,
    base_url: String,
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

        Self {
            client: Client::builder().default_headers(headers).build().unwrap(),
            base_url: "https://api.notion.com/v1".to_string(),
        }
    }

    fn build_url(&self, endpoint: &str) -> String {
        format!("{}{}", self.base_url, endpoint)
    }

    pub async fn get(&self, endpoint: &str) -> Result<String, reqwest::Error> {
        let url = self.build_url(endpoint);
        let res = self.client.get(url).send().await?;
        res.text().await
    }

    pub async fn post<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: Option<String>,
    ) -> Result<T, reqwest::Error> {
        let url = self.build_url(endpoint);
        if let Some(payload) = body {
            let res = self.client.post(url).body(payload).send().await?;
            return res.json::<T>().await;
        }

        let res = self.client.post(url).send().await?;
        res.json::<T>().await
    }
}
