use std::collections::HashMap;

use reqwest::Client;
use serde::Serialize;
use serde_derive::Serialize;

pub struct PageEndpoint {
    client: Client,
    url: String,
}

#[derive(Serialize)]
struct UpdatePageRequest<T> {
    parent: Parent,
    properties: HashMap<String, T>,
}

#[derive(Serialize)]
struct Parent {
    database_id: String,
}

impl PageEndpoint {
    pub fn new(client: Client, base_url: &str) -> Self {
        let url = base_url.to_string() + "/pages";

        Self { client, url }
    }

    pub async fn update<T: Serialize>(
        &self,
        page_id: &str,
        database_id: &str,
        properties: HashMap<String, T>,
    ) -> Result<(), reqwest::Error> {
        let endpoint = format!("{}/{}", self.url, page_id);
        let body = UpdatePageRequest {
            parent: Parent {
                database_id: database_id.to_string(),
            },
            properties,
        };

        self.client
            .patch(endpoint.as_str())
            .json(&body)
            .send()
            .await?;
        Ok(())
    }
}
