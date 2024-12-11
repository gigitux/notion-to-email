use article_scraper::{Article, ArticleScraper};
use dotenv::dotenv;
use notion_to_email::{
    client::{database_response::DatabaseResponse, notion_client::NotionClient},
    utils::generate_random_index,
};
use reqwest::Client;
use url::Url;

use std::env;

struct EnvVars {
    api_key: String,
    database_id: String,
}

fn get_env_vars() -> EnvVars {
    let api_key = env::var("NOTION_API_KEY").expect("NOTION_API_KEY must be set");
    let database_id = env::var("NOTION_DATABASE_ID").expect("NOTION_DATABASE_ID must be set");

    EnvVars {
        api_key,
        database_id,
    }
}

async fn parse_link(link: &str) -> Article {
    let scraper = ArticleScraper::new(None);
    let url = Url::parse(link).expect("Failed to parse URL");
    let client = Client::new();
    scraper
        .await
        .parse(&url, false, &client, None)
        .await
        .expect("Failed to parse article")
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let EnvVars {
        api_key,
        database_id,
    } = get_env_vars();

    let notion_client = NotionClient::new(&api_key);

    let endpoint = format!("/databases/{}/query", database_id);

    let database_response = notion_client
        .post::<DatabaseResponse>(&endpoint, None)
        .await
        .expect("Failed to get database response");

    let number_of_links = database_response.results.len();

    let random_index = generate_random_index(number_of_links);

    let link = &database_response.results[random_index].properties.url.url;

    let article = parse_link(link).await;
}
