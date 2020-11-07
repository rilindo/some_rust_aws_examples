extern crate rusoto_core;
extern crate rusoto_s3;

use rusoto_core::Region;
use rusoto_s3::{S3, S3Client, CreateBucketRequest, CreateBucketConfiguration};

#[tokio::main]
async fn main() {

    // See https://docs.rs/rusoto_core/0.40.0/rusoto_core/region/enum.Region.html#default
    // to under how the defaults work for regions.
    
    let client = S3Client::new(Region::default());

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
