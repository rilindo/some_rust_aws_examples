extern crate rusoto_core;
extern crate rusoto_sns;

use rusoto_core::Region;
use rusoto_sns::{Sns,
    SnsClient,
    ListTopicsInput,
    ListTopicsResponse
};

async fn list_topics_input(client: &SnsClient, list_topics_request: ListTopicsInput) -> ListTopicsResponse {

    let resp = client.list_topics(list_topics_request).await;
    return resp.unwrap();
}

fn list_topics_response(resp: ListTopicsResponse) {

    match resp.topics {
        Some(topics) => {
            for t in topics {
                match t.topic_arn {
                    Some(topic) => println!("{}",topic),
                    None => println!("No topics in this region"),
                }

            }
        }
        None => println!("No topics in this region"),
    }
    match resp.next_token {
        Some(token) => println!("{}", token),
        None => (),
    }
}

fn main() {

    let client = SnsClient::new(Region::default());
    let list_topics_request = ListTopicsInput {
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let resp = rt.block_on(list_topics_input(&client, list_topics_request));

    list_topics_response(resp.clone());

}
