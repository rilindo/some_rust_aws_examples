extern crate rusoto_core;
extern crate rusoto_sqs;

extern crate clap;
use clap::{App, Arg};

use rusoto_core::Region;
use rusoto_sqs::{
    GetQueueUrlRequest, GetQueueUrlResult, SendMessageRequest, SendMessageResult, Sqs, SqsClient,
};

async fn get_queue_url(client: &SqsClient, queue_name: &str) -> GetQueueUrlResult {
    let get_queue_url_req = GetQueueUrlRequest {
        queue_name: queue_name.to_string(),
        ..Default::default()
    };

    let resp = client.get_queue_url(get_queue_url_req).await;
    return resp.unwrap();
}

async fn send_message(
    client: &SqsClient,
    message_body: &str,
    queue_url: &GetQueueUrlResult,
) -> SendMessageResult {
    let create_message_req = SendMessageRequest {
        message_body: message_body.to_string(),
        queue_url: queue_url
            .queue_url
            .as_deref()
            .unwrap_or("default string")
            .to_string(),
        ..Default::default()
    };

    let resp = client.send_message(create_message_req).await;

    return resp.unwrap();
}

fn main() {
    let matches = App::new("Example of a get queue URL using Rust")
        .version("1.0")
        .author("rilindo.foster@<rilindo.foster@monzell.com")
        .about("get queue url")
        .arg(
            Arg::with_name("queue_name")
                .short("q")
                .long("queue_name")
                .help("Set queue name")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("message")
                .short("m")
                .long("message")
                .help("Set Message")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let client = SqsClient::new(Region::default());
    let queue_name = matches.value_of("queue_name").unwrap();
    let message = matches.value_of("message").unwrap();
    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let queue_url_resp = rt.block_on(get_queue_url(&client, &queue_name));

    let send_message_resp = rt.block_on(send_message(&client, &message, &queue_url_resp));

    println!(
        "Queue message '{}' send, resp: {:#?}",
        message.clone(),
        send_message_resp
    );
}
