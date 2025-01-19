use article_scraper::{Article, ArticleScraper};
use reqwest::Client;
use url::Url;

pub async fn parse_link(link: &str) -> Article {
    let scraper = ArticleScraper::new(None);
    let url = Url::parse(link).expect("Failed to parse URL");
    let client = Client::new();
    scraper
        .await
        .parse(&url, false, &client, None)
        .await
        .expect("Failed to parse article")
}
