extern crate rusoto_core;
extern crate rusoto_sqs;

use rusoto_core::Region;
use rusoto_sqs::{Sqs,
    SqsClient,
    SendMessageRequest
};

#[tokio::main]
async fn main() {

    // See https://docs.rs/rusoto_core/0.40.0/rusoto_core/region/enum.Region.html#default
    // to under how the defaults work for regions.

    let client = SqsClient::new(Region::default());

    /*
    I'll do the queue url lookup in another example.
    */
    let my_queue_url = "https://sqs.us-east-1.amazonaws.com/1111111111111/my_queue_name".to_string();
    let my_message_body = "this is a test message".to_string();

    let create_message_req = SendMessageRequest {
        message_body: my_message_body.clone(),
        queue_url: my_queue_url.clone(),
        ..Default::default()
    };

    let resp = client.send_message(create_message_req).await;

    println!(
        "Queue message '{}' created, resp: {:#?}",
        my_message_body.clone(),
        resp.unwrap()
    );
}
