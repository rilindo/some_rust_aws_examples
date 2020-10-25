extern crate rusoto_core;
extern crate rusoto_s3;

// use std::default::Default;
use rusoto_core::Region;
use rusoto_s3::{S3, S3Client};
// use std::fmt::Display;

#[tokio::main]
async fn main() {

    let client = S3Client::new(Region::UsEast2);
    // let list_buckets_output: ListBucketsOutput = Default::default();
    let resp = client.list_buckets().await;
    // let resp = resp.unwrap();
    // for bucket in resp.buckets.unwrap().iter() {
    //     print!("{:?}\n", bucket.name)
    // }
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
