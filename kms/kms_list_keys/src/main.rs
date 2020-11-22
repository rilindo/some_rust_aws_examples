extern crate rusoto_core;
extern crate rusoto_kms;

use rusoto_core::Region;
use rusoto_kms::{Kms,
    KmsClient,
    ListKeysRequest
};

#[tokio::main]
async fn main() {

    // See https://docs.rs/rusoto_core/0.40.0/rusoto_core/region/enum.Region.html#default
    // to under how the defaults work for regions.

    let client = KmsClient::new(Region::default());
    let list_keys_request = ListKeysRequest {
        ..Default::default()
    };

    match client.list_keys(list_keys_request).await {
        Ok(output) => {
            match output.keys {
                Some(keys) => {
                    for k in keys {
                        match k.key_arn {
                            Some(key_arn) => println!("KMS arn: {}",key_arn),
                            None => println!("No kms keys in this region"),
                        }
                        match k.key_id {
                            Some(key_id) => println!("KMS id: {}",key_id),
                            None => println!("No kms keys in this region"),
                        }

                    }
                }
                None => println!("No kms keys in this region"),
            }
            match output.next_marker {
                Some(token) => println!("{}", token),
                None => println!("No tokens available"),
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
