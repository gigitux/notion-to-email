use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::{Code, Response};
use lettre::transport::smtp::Error;
use lettre::{Message, SmtpTransport};

pub struct EmailContent {
    pub subject: String,
    pub body: String,
    pub email_from: String,
    pub email_to: String,
    pub smtp_server: String,
}

pub fn send_email(
    username: &str,
    password: &str,
    email_content: EmailContent,
) -> Result<Response, Error> {
    let EmailContent {
        subject,
        body,
        email_from,
        email_to,
        smtp_server,
    } = email_content;
    let from_address = format!("Notion To Email Bot <{}>", email_from);

    #[allow(unused_variables)] // This variable is used only in production build
    let email = Message::builder()
        .from(from_address.parse().unwrap())
        .to(email_to.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(body)
        .unwrap();

    let creds = Credentials::new(username.to_string(), password.to_string());

    // Open a remote connection to gmail
    #[allow(unused_variables)] // This variable is used only in production build
    let mailer = SmtpTransport::relay(&smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    #[cfg(debug_assertions)]
    {
        println!("Email is sent");
        Ok(Response::new(
            Code::new(
                lettre::transport::smtp::response::Severity::PositiveCompletion,
                lettre::transport::smtp::response::Category::Information,
                lettre::transport::smtp::response::Detail::Eight,
            ),
            vec!["Email sent".to_string()],
        ))
    }

    #[cfg(not(debug_assertions))]
    {
        mailer.send(&email)
    }
}
