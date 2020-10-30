extern crate rusoto_core;
extern crate rusoto_sqs;

use rusoto_core::Region;
use rusoto_sqs::{Sqs,
    SqsClient,
    SendMessageRequest
};

#[tokio::main]
async fn main() {
    let client = SqsClient::new(Region::UsEast1);

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
