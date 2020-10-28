extern crate rusoto_core;
extern crate rusoto_sqs;

use rusoto_core::Region;
use rusoto_sqs::{Sqs,
    SqsClient,
    CreateQueueRequest
};

#[tokio::main]
async fn main() {
    let client = SqsClient::new(Region::UsEast2);
    let my_queue_name = "my_queue_name".to_string();

    let create_queue_req = CreateQueueRequest {
        queue_name: my_queue_name.clone(),
        ..Default::default()
    };

    let resp = client.create_queue(create_queue_req).await;

    println!(
        "Topic {} created, resp: {:#?}",
        my_queue_name.clone(),
        resp.unwrap()
    );
}
