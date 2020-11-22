extern crate rusoto_core;
extern crate rusoto_ses;

extern crate clap;
use clap::{Arg, App};

use rusoto_core::Region;
use rusoto_ses::{Ses,
    SesClient,
    SendEmailRequest,
    SendEmailResponse,
    Body,
    Message,
    Destination,
    Content,
};

struct EmailMessage {
    destinations: Destination,
    message: Message,
}

fn message(destination: &str, subject: &str, content: &str) -> EmailMessage {

    let message = EmailMessage {
        destinations: Destination {
            to_addresses: Some(vec![destination.to_string()]),
            ..Default::default()
        },
        message: Message {
            body: Body {
                text: Some(
                    Content {
                        data: String::from(content),
                        ..Default::default()
                    }
                ),
                ..Default::default()
            },
            subject: Content {
                data: String::from(subject),
                ..Default::default()
            },
        },
    };

    message
}

async fn send_email(client: &SesClient, send_email_request: SendEmailRequest) -> SendEmailResponse {

    let resp = client.send_email(send_email_request).await;
    return resp.unwrap();

}

fn process_send_email(resp: SendEmailResponse) {

    println!("Message ID: {}", resp.message_id )

}

fn main() {

    let client = SesClient::new(Region::default());

    let matches = App::new("Example Send SES Message by Rust")
                            .version("1.0")
                            .author("rilindo.foster@<rilindo.foster@monzell.com")
                            .about("Send SES")
                            .arg(Arg::with_name("SOURCE")
                               .help("Source Email")
                               .required(true)
                               .index(1))
                           .arg(Arg::with_name("DESTINATION")
                              .help("Destination Email")
                              .required(true)
                              .index(2))
                          .arg(Arg::with_name("SUBJECT")
                             .help("SUBJECT")
                             .required(true)
                             .index(3))
                          .arg(Arg::with_name("CONTENT")
                             .help("CONTENT")
                             .required(true)
                             .index(4))
                              .get_matches();

    let source = matches.value_of("SOURCE").unwrap();
    let destination = matches.value_of("DESTINATION").unwrap();
    let subject = matches.value_of("SUBJECT").unwrap();
    let content = matches.value_of("CONTENT").unwrap();

    let message = message(destination, subject, content);

    let send_email_request = SendEmailRequest {
        destination: message.destinations,
        message: message.message,
        source: source.to_string(),
        reply_to_addresses: Some(vec![source.to_string()]),
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let resp = rt.block_on(send_email(&client, send_email_request));

    process_send_email(resp.clone());

}
