extern crate rusoto_core;
extern crate rusoto_sns;

extern crate clap;
use clap::{App, Arg};

use rusoto_core::Region;
use rusoto_sns::{CreateTopicInput, CreateTopicResponse, Sns, SnsClient};

async fn create_topic(
    client: &SnsClient,
    create_topic_request: CreateTopicInput,
) -> CreateTopicResponse {
    let resp = client.create_topic(create_topic_request).await;
    return resp.unwrap();
}

fn create_topics_response(resp: CreateTopicResponse) {
    match resp.topic_arn {
        Some(topic_arn) => println!("SNS Topic Created: {}", topic_arn),
        None => println!("Unable to create SNS Topic"),
    }
}

fn main() {
    let matches = App::new("Example of a create topic call using Rust")
        .version("1.0")
        .author("rilindo.foster@<rilindo.foster@monzell.com")
        .about("Create Topic Command")
        .arg(
            Arg::with_name("topic_name")
                .short("t")
                .long("topic_name")
                .help("Set Topic Name")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let client = SnsClient::new(Region::default());
    let topic_name = matches.value_of("topic_name").unwrap();

    let create_topic_request = CreateTopicInput {
        name: topic_name.clone().to_string(),
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let resp = rt.block_on(create_topic(&client, create_topic_request));

    create_topics_response(resp.clone());
}
