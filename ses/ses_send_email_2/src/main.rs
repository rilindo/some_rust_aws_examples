extern crate rusoto_core;
extern crate rusoto_ses;

use rusoto_core::Region;
use rusoto_ses::{Ses,
    SesClient,
    SendEmailRequest,
};

struct EmailConfig {
    destination: String,
    source: String,
    reply_to_addresses: Vec<String>,
}

struct EmailContent {
    subject: rusoto_ses::Content,
    content: rusoto_ses::Content,
}

#[tokio::main]
async fn main() {

    // Second try with ses. looking better. That said, I'll
    // rewrite it to break it out to use functions.

    // See https://docs.rs/rusoto_core/0.40.0/rusoto_core/region/enum.Region.html#default
    // to under how the defaults work for regions.

    let client = SesClient::new(Region::default());

    let email_config = EmailConfig {
        destination: "jdoe@example.com".to_string(),
        source: "webmaster@example.com".to_string(),
        reply_to_addresses: vec!["webmaster@example.com".to_string()]
    };

    let content = EmailContent {
        subject: rusoto_ses::Content {
            data: String::from("This is a test message"),
            ..Default::default()
        },
        content: rusoto_ses::Content {
            data: String::from("test 1 2 3"),
            ..Default::default()
        },
    };

    let body = rusoto_ses::Body {
        text: Some(content.content),
        ..Default::default()
    };

    let my_message = rusoto_ses::Message {
        body: body,
        subject: content.subject,
    };

    let my_destinations = rusoto_ses::Destination {
        to_addresses: Some(vec![email_config.destination]),
        ..Default::default()
    };

    let send_email_req = SendEmailRequest {
        destination: my_destinations,
        message: my_message,
        source: email_config.source,
        reply_to_addresses: Some(email_config.reply_to_addresses),
        ..Default::default()
    };

    match client.send_email(send_email_req).await {
        Ok(output) => {
            println!("Message ID: {}", output.message_id )
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
