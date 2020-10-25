extern crate rusoto_core;
extern crate rusoto_s3;

use rusoto_core::Region;
use rusoto_s3::{S3, S3Client};

#[tokio::main]
async fn main() {

    let client = S3Client::new(Region::UsEast2);
    let resp = client.list_buckets().await;
    match resp {
      Ok(output) => {
        match output.buckets {
          Some(buckets) => {
            println!("List of buckets:");

            for bucket in buckets {
              println!("{}", bucket.name.unwrap());
            }
          }
          None => println!("No Buckets!"),
        }
      }
      Err(error) => {
        println!("Error: {:?}", error);
      }
    }
}
