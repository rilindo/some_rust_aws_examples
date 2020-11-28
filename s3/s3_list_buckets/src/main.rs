extern crate rusoto_core;
extern crate rusoto_s3;

use rusoto_core::Region;
use rusoto_s3::{S3Client, S3};

#[tokio::main]
async fn main() {
    // See https://docs.rs/rusoto_core/0.40.0/rusoto_core/region/enum.Region.html#default
    // to under how the defaults work for regions.

    let client = S3Client::new(Region::default());
    let resp = client.list_buckets().await;
    match resp {
        Ok(output) => match output.buckets {
            Some(buckets) => {
                println!("List of buckets:");

                for bucket in buckets {
                    println!("{}", bucket.name.unwrap());
                }
            }
            None => println!("No Buckets!"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
