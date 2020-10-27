extern crate rusoto_core;
extern crate rusoto_sns;

use rusoto_core::Region;
use rusoto_sns::{Sns,
    SnsClient,
    CreateTopicInput
};

#[tokio::main]
async fn main() {
    let client = SnsClient::new(Region::UsEast2);
    let my_topic_name = "mytopicname".to_string();

    let create_topic_req = CreateTopicInput {
        name: my_topic_name.clone(),
        ..Default::default()
    };

    let resp = client.create_topic(create_topic_req).await;

    println!(
        "Topic {} created, resp: {:#?}",
        my_topic_name.clone(),
        resp.unwrap()
    );
}
