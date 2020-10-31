extern crate rusoto_core;
extern crate rusoto_sqs;

use rusoto_core::Region;
use rusoto_sqs::{Sqs,
    SqsClient,
    SendMessageRequest,
    SendMessageResult,
    GetQueueUrlRequest,
    GetQueueUrlResult
};

async fn get_queue_url(client: &SqsClient, queue_name: &str) -> GetQueueUrlResult {

    let get_queue_url_req = GetQueueUrlRequest {
        queue_name: queue_name.to_string(),
        ..Default::default()
    };

    let resp = client.get_queue_url(get_queue_url_req).await;
    return resp.unwrap()
}

async fn send_message(client: &SqsClient, message_body: &str, queue_url: &GetQueueUrlResult) -> SendMessageResult {

    let create_message_req = SendMessageRequest {
        message_body: message_body.to_string(),
        queue_url: queue_url.queue_url.as_deref().unwrap_or("default string").to_string(),
        ..Default::default()
    };

    let resp = client.send_message(create_message_req).await;

    return resp.unwrap()
}

fn main() {
    let client = SqsClient::new(Region::UsEast2);
    let my_queue_name = "my_queue_name";
    let my_message_body = "this is a test message".to_string();
    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let queue_url_resp = rt.block_on(get_queue_url(&client, &my_queue_name));

    let send_message_resp = rt.block_on(send_message(&client, &my_message_body, &queue_url_resp));

    println!(
        "Queue message '{}' send, resp: {:#?}",
        my_message_body.clone(),
        send_message_resp
    );
}
