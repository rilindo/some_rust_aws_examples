extern crate rusoto_core;
extern crate rusoto_ses;

use rusoto_core::Region;
use rusoto_ses::{Ses,
    SesClient,
    SendEmailRequest,
};

#[tokio::main]
async fn main() {

    // First try with ses. Pretty messy, but it works. I will look into
    // refactoring this on the next round.

    // See https://docs.rs/rusoto_core/0.40.0/rusoto_core/region/enum.Region.html#default
    // to under how the defaults work for regions.

    let client = SesClient::new(Region::default());

    let my_destination = "<jdoe@example.com>".to_string();
    let my_reply_to_addresses = ["webmaster@example.org".to_string()];
    let my_source = "webmaster@example.org".to_string();

    let my_subject = rusoto_ses::Content {
        data: String::from("This is a test message"),
        ..Default::default()
    };

    let my_content = rusoto_ses::Content {
        data: String::from("test 1 2 3"),
        ..Default::default()
    };

    let my_body = rusoto_ses::Body {
        text: Some(my_content),
        ..Default::default()
    };

    let my_message = rusoto_ses::Message {
        body: my_body,
        subject: my_subject,
    };

    let my_destinations = rusoto_ses::Destination {
        to_addresses: Some(vec![my_destination]),
        ..Default::default()
    };

    let send_email_req = SendEmailRequest {
        destination: my_destinations,
        message: my_message,
        source: my_source.clone(),
        reply_to_addresses: Some(my_reply_to_addresses.to_vec()),
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
