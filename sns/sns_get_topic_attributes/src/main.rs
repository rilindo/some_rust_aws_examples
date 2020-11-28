extern crate rusoto_core;
extern crate rusoto_sns;

extern crate clap;
use clap::{App, Arg};

use rusoto_core::Region;
use rusoto_sns::{GetTopicAttributesInput, GetTopicAttributesResponse, Sns, SnsClient};

async fn get_topic_attributes(
    client: &SnsClient,
    get_topic_attributes_request: GetTopicAttributesInput,
) -> GetTopicAttributesResponse {
    let resp = client
        .get_topic_attributes(get_topic_attributes_request)
        .await;
    return resp.unwrap();
}

fn get_topic_attributes_response(resp: GetTopicAttributesResponse) {
    match resp.attributes {
        Some(attributes) => {
            for (key, value) in attributes.iter() {
                println!("{}: {}", key, value);
            }
        }
        None => println!("Unable to get topic attributes"),
    }
}

fn main() {
    let matches = App::new("Example of a get topic attribute call using Rust")
        .version("1.0")
        .author("rilindo.foster@<rilindo.foster@monzell.com")
        .about("Get Topic Attributes")
        .arg(
            Arg::with_name("topic_arn")
                .short("t")
                .long("topic_arn")
                .help("Set Topic Name")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let client = SnsClient::new(Region::default());
    let topic_arn = matches.value_of("topic_arn").unwrap();

    let get_topic_attributes_request = GetTopicAttributesInput {
        topic_arn: topic_arn.clone().to_string(),
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let resp = rt.block_on(get_topic_attributes(&client, get_topic_attributes_request));

    get_topic_attributes_response(resp.clone());
}
