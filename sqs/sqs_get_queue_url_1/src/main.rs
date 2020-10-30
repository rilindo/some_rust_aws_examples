extern crate rusoto_core;
extern crate rusoto_sqs;

use rusoto_core::Region;
use rusoto_sqs::{Sqs,
    SqsClient,
    GetQueueUrlRequest
};

#[tokio::main]
async fn main() {
    let client = SqsClient::new(Region::UsEast2);
    let my_queue_name = "my_queue_name".to_string();

    let get_queue_url_req = GetQueueUrlRequest {
        queue_name: my_queue_name.clone(),
        ..Default::default()
    };

    let resp = client.get_queue_url(get_queue_url_req).await;

    println!(
        "Return output of queue {}, resp: {:#?}",
        my_queue_name.clone(),
        resp.unwrap()
    );
}
