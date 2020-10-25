extern crate rusoto_core;
extern crate rusoto_s3;

use rusoto_core::Region;
use rusoto_s3::{S3, S3Client, CreateBucketRequest, CreateBucketConfiguration};

#[tokio::main]
async fn main() {
    let client = S3Client::new(Region::UsEast2);

    let my_bucket_name = "mybucketname".to_string();
    let location_constraint = CreateBucketConfiguration {
        location_constraint: Some("us-east-2".to_string()),
    };

    let create_bucket_req = CreateBucketRequest {
        bucket: my_bucket_name.clone(),
        create_bucket_configuration: Some(location_constraint.clone()),
        ..Default::default()
    };

    let resp = client.create_bucket(create_bucket_req).await;

    println!(
        "Bucket {} created, resp: {:#?}",
        my_bucket_name.clone(),
        resp.unwrap()
    );
}
