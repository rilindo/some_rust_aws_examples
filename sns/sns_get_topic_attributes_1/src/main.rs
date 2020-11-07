extern crate rusoto_core;
extern crate rusoto_sns;

use rusoto_core::Region;
use rusoto_sns::{Sns,
    SnsClient,
    GetTopicAttributesInput
};

#[tokio::main]
async fn main() {

    // See https://docs.rs/rusoto_core/0.40.0/rusoto_core/region/enum.Region.html#default
    // to under how the defaults work for regions.

    let client = SnsClient::new(Region::default());
    let my_topic_arn = "arn:aws:sns:us-east-2:123456789012:mytopicname".to_string();

    let get_topic_attributes_req = GetTopicAttributesInput {
        topic_arn: my_topic_arn.clone()
    };

    match client.get_topic_attributes(get_topic_attributes_req).await {
        Ok(output) => {
            match output.attributes {
                Some(attributes) =>
                for (key, value) in attributes.iter() {
                    println!("{}: {}", key, value);
                }
                None => println!("Unable to get topic attributes"),
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
