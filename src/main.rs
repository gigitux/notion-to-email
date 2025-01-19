use dotenv::dotenv;
use notion_to_email::{
    client::{body::PropertyValue, notion_client::NotionClient},
    email::send_email::{send_email, EmailContent},
    scraper::utils::parse_link,
    utils::generate_random_index,
};

use std::{collections::HashMap, env};

struct EnvVars {
    api_key: String,
    database_id: String,
    username: String,
    password: String,
    email_from: String,
    email_to: String,
    smtp_server: String,
}

fn get_env_vars() -> EnvVars {
    let api_key = env::var("NOTION_API_KEY").expect("NOTION_API_KEY must be set");
    let database_id = env::var("NOTION_DATABASE_ID").expect("NOTION_DATABASE_ID must be set");
    let username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
    let email_from = env::var("EMAIL_FROM").expect("EMAIL_FROM must be set");
    let email_to = env::var("EMAIL_TO").expect("EMAIL_TO must be set");
    let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER must be set");

    EnvVars {
        api_key,
        database_id,
        username,
        password,
        email_from,
        email_to,
        smtp_server,
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let EnvVars {
        api_key,
        database_id,
        username,
        password,
        email_from,
        email_to,
        smtp_server,
    } = get_env_vars();

    let notion_client = NotionClient::new(&api_key);

    let database_response = notion_client
        .database
        .get(database_id.as_str())
        .await
        .unwrap();

    let number_of_links = database_response.results.len();
    let random_index = generate_random_index(number_of_links);
    let notion_object = &database_response.results[random_index];
    let link = &notion_object.properties.url.url;
    let article = parse_link(link).await;

    let email_content = EmailContent {
        email_from,
        email_to,
        subject: article.title.unwrap_or("No title found".to_string()),
        body: article.html.unwrap_or("No content found".to_string()),
        smtp_server,
    };

    send_email(&username, &password, email_content).expect("Failed to send email");

    let mut properties = HashMap::new();
    properties.insert(
        "Read Status".to_string(),
        PropertyValue::Checkbox { checkbox: true },
    );

    notion_client
        .page
        .update(&notion_object.id, &database_id, properties)
        .await
        .expect("Failed to update page");
}
