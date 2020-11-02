extern crate rusoto_core;
extern crate rusoto_sns;

use rusoto_core::Region;
use rusoto_sns::{Sns,
    SnsClient,
    ListTopicsInput,
};

#[tokio::main]
async fn main() {
    let client = SnsClient::new(Region::UsEast2);
    let list_topics_req = ListTopicsInput {
        ..Default::default()
    };

    match client.list_topics(list_topics_req).await {
        Ok(output) => {
            match output.topics {
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
            match output.next_token {
                Some(token) => println!("{}", token),
                None => println!("No tokens available"),
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
