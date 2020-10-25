extern crate rusoto_core;
extern crate rusoto_s3;

use rusoto_core::Region;
use rusoto_s3::{S3, S3Client, DeleteBucketRequest};

#[tokio::main]
async fn main() {
    let client = S3Client::new(Region::UsEast2);
    let my_bucket_name = "mybucketname".to_string();

    let delete_bucket_req = DeleteBucketRequest {
        bucket: my_bucket_name.clone(),
        ..Default::default()
    };

    let resp = client.delete_bucket(delete_bucket_req).await;

    println!(
        "Bucket {} deleted, resp: {:#?}",
        my_bucket_name.clone(),
        resp.unwrap()
    );

}
