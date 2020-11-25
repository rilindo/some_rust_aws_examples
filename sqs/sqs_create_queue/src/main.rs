extern crate rusoto_core;
extern crate rusoto_sqs;

extern crate clap;
use clap::{Arg, App};

use rusoto_core::Region;
use rusoto_sqs::{Sqs,
    SqsClient,
    CreateQueueRequest
};

#[tokio::main]
async fn main() {

    let matches = App::new("Example of a sqs create call using Rust")
                            .version("1.0")
                            .author("rilindo.foster@<rilindo.foster@monzell.com")
                            .about("Create Queue")
                            .arg(Arg::with_name("queue_name")
                                .short("q")
                                .long("queue_name")
                                .help("Set queue name")
                                .required(true)
                                .takes_value(true))
                            .get_matches();

    let client = SqsClient::new(Region::default());
    let queue_name = matches.value_of("queue_name").unwrap();

    let create_queue_request = CreateQueueRequest {
        queue_name: queue_name.clone().to_string(),
        ..Default::default()
    };

    let resp = client.create_queue(create_queue_request).await;

    println!(
        "Queue {} created, resp: {:#?}",
        queue_name.clone(),
        resp.unwrap()
    );
}
