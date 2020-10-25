extern crate rusoto_core;
extern crate rusoto_s3;

use rusoto_core::Region;
use rusoto_s3::{S3, S3Client};

#[tokio::main]
async fn main() {

    let client = S3Client::new(Region::UsEast2);
    let resp = client.list_buckets().await;
    let resp = resp.unwrap();
    for bucket in resp.buckets.unwrap().iter() {
        print!("{}\n", bucket.name.as_ref().unwrap())
    }
}
