extern crate rusoto_core;
extern crate rusoto_sqs;

use rusoto_core::Region;
use rusoto_sqs::{Sqs,
    SqsClient,
    GetQueueUrlRequest
};

#[tokio::main]
async fn main() {

    // See https://docs.rs/rusoto_core/0.40.0/rusoto_core/region/enum.Region.html#default
    // to under how the defaults work for regions.

    let client = SqsClient::new(Region::default());
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
