use reqwest::Client;

use super::model::DatabaseResponse;

pub struct DatabaseEndpoint {
    client: Client,
    url: String,
}

impl DatabaseEndpoint {
    pub fn new(client: Client, base_url: &str) -> Self {
        let url = base_url.to_string() + "/databases";

        Self { client, url }
    }

    pub async fn get(&self, database_id: &str) -> Result<DatabaseResponse, reqwest::Error> {
        let endpoint = format!("{}/{}/query", self.url, database_id);
        self.client
            .post(endpoint.as_str())
            .send()
            .await?
            .json()
            .await
    }
}
