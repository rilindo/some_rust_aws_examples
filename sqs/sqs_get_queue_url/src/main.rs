extern crate rusoto_core;
extern crate rusoto_sqs;

extern crate clap;
use clap::{App, Arg};

use rusoto_core::Region;
use rusoto_sqs::{GetQueueUrlRequest, Sqs, SqsClient};

#[tokio::main]
async fn main() {
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
        .get_matches();

    let client = SqsClient::new(Region::default());
    let queue_name = matches.value_of("queue_name").unwrap();

    let get_queue_url_req = GetQueueUrlRequest {
        queue_name: queue_name.clone().to_string(),
        ..Default::default()
    };

    let resp = client.get_queue_url(get_queue_url_req).await;

    println!(
        "Return output of queue {}, resp: {:#?}",
        queue_name.clone(),
        resp.unwrap()
    );
}
