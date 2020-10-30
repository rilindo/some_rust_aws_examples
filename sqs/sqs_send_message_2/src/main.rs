extern crate rusoto_core;
extern crate rusoto_sqs;

use rusoto_core::Region;
use rusoto_sqs::{Sqs,
    SqsClient,
    SendMessageRequest,
    GetQueueUrlRequest,
    GetQueueUrlResult,
    GetQueueUrlError
};

async fn get_queue_url<'a, 'b>(client: &'a SqsClient, queue_name: &'b str) -> GetQueueUrlResult {

// async fn get_queue_url(client: &SqsClient, queue_name: &str){

    let get_queue_url_req = GetQueueUrlRequest {
        queue_name: queue_name.to_string(),
        ..Default::default()
    };

    let resp = client.get_queue_url(get_queue_url_req).await;

    // println!(
    //     "Return output of queue {}, resp: {:#?}",
    //     queue_name.to_string(),
    //     resp.unwrap()
    // );

    return resp.unwrap()
}

async fn send_message<'a>(client: &'a SqsClient, message_body: &str, queue_url: &GetQueueUrlResult){

    let create_message_req = SendMessageRequest {
        message_body: message_body.to_string(),
        queue_url: queue_url.queue_url.as_deref().unwrap_or("default string").to_string(),
        ..Default::default()
    };
    
    let resp = client.send_message(create_message_req).await;
}

fn main() {
    let client = SqsClient::new(Region::UsEast2);
    let my_queue_name = "my_queue_name";
    let my_message_body = "this is a test message".to_string();
    /*
    I'll do the queue url lookup in another example.
    */
    // let my_queue_url = get_queue_url(&client, &my_queue_name);
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let queue_url_resp = rt.block_on(get_queue_url(&client, &my_queue_name));
    println!("{:#?}", queue_url_resp.queue_url );

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(send_message(&client, &my_message_body, &queue_url_resp));
    // get_queue_url(&client, &my_queue_name);

    // let my_message_body = "this is a test message".to_string();

    // let create_message_req = SendMessageRequest {
    //     message_body: my_message_body.clone(),
    //     queue_url: my_queue_url.clone(),
    //     ..Default::default()
    // };
    //
    // let resp = client.send_message(create_message_req).await;
    //
    // println!(
    //     "Queue message '{}' created, resp: {:#?}",
    //     my_message_body.clone(),
    //     resp.unwrap()
    // );
}
