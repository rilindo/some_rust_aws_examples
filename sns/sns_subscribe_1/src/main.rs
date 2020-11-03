extern crate rusoto_core;
extern crate rusoto_sns;

use rusoto_core::Region;
use rusoto_sns::{Sns,
    SnsClient,
    SubscribeInput
};

#[tokio::main]
async fn main() {
    let client = SnsClient::new(Region::UsEast2);
    let my_topic_arn = "arn:aws:sns:us-east-2:123456789012:mytopicname".to_string();
    let my_endpoint = "jdoe@example.com".to_string();
    let my_protocol = "email".to_string();

    let subscribe_req = SubscribeInput {
        topic_arn: my_topic_arn.clone(),
        protocol: my_protocol.clone(),
        endpoint: Some(my_endpoint),
        ..Default::default()
    };

    match client.subscribe(subscribe_req).await {
        Ok(output) => {
            match output.subscription_arn {
                Some(subscription_arn) => {
                    println!("subscription status: {}",subscription_arn)
                }
                None => println!("Unable to subscribe endpoint"),
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
