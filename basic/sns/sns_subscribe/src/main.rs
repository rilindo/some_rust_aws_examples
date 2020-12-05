extern crate rusoto_core;
extern crate rusoto_sns;

extern crate clap;
use clap::{App, Arg};

use rusoto_core::Region;
use rusoto_sns::{Sns, SnsClient, SubscribeInput, SubscribeResponse};

async fn subscribe_input(
    client: &SnsClient,
    subscribe_request: SubscribeInput,
) -> SubscribeResponse {
    let resp = client.subscribe(subscribe_request).await;
    return resp.unwrap();
}

fn subscribe_response(resp: SubscribeResponse) {
    match resp.subscription_arn {
        Some(subscription_arn) => {
            println!("subscription status: {}", subscription_arn)
        }
        None => println!("Unable to subscribe endpoint"),
    }
}

fn main() {
    let matches = App::new("Example of a sns subscribe call using Rust")
        .version("1.0")
        .author("rilindo.foster@<rilindo.foster@monzell.com")
        .about("Subscribe Topic")
        .arg(
            Arg::with_name("topic_arn")
                .short("t")
                .long("topic_arn")
                .help("Set Topic Name")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("endpoint")
                .short("e")
                .long("endpoint")
                .help("Set Endpoint")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("protocol")
                .short("p")
                .long("protocol")
                .help("Set protocol")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let client = SnsClient::new(Region::default());
    let topic_arn = matches.value_of("topic_arn").unwrap();
    let endpoint = matches.value_of("endpoint").unwrap();
    let protocol = matches.value_of("protocol").unwrap();

    let subscribe_request = SubscribeInput {
        topic_arn: topic_arn.clone().to_string(),
        protocol: protocol.clone().to_string(),
        endpoint: Some(endpoint.to_string()),
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let resp = rt.block_on(subscribe_input(&client, subscribe_request));

    subscribe_response(resp.clone());
}
