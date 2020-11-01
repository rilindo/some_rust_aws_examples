extern crate rusoto_core;
extern crate rusoto_sqs;

use rusoto_core::Region;
use rusoto_sqs::{Sqs,
    SqsClient,
    DeleteMessageRequest,
    ReceiveMessageRequest,
    ReceiveMessageResult,
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

async fn receive_message(client: &SqsClient, queue_url: &GetQueueUrlResult) -> ReceiveMessageResult {

    let receive_message_req = ReceiveMessageRequest {
        queue_url: queue_url.queue_url.as_deref().unwrap_or("default string").to_string(),
        ..Default::default()
    };

    let resp = client.receive_message(receive_message_req).await;

    return resp.unwrap()
}

async fn delete_message(client: &SqsClient, queue_url: &GetQueueUrlResult, message: &ReceiveMessageResult) {

    let delete_message_req = DeleteMessageRequest {
        queue_url: queue_url.queue_url.as_deref().unwrap_or("default string").to_string(),
        receipt_handle: message.messages.as_ref().unwrap()[0].receipt_handle.as_ref().unwrap().to_string()
    };

    let _resp = client.delete_message(delete_message_req).await;

}

fn main() {
    let client = SqsClient::new(Region::UsEast2);
    let my_queue_name = "my_queue_name";
    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let queue_url_resp = rt.block_on(get_queue_url(&client, &my_queue_name));
    let received_message_resp = rt.block_on(receive_message(&client, &queue_url_resp));

    rt.block_on(delete_message(&client, &queue_url_resp, &received_message_resp));

}
