extern crate rusoto_core;
extern crate rusoto_s3;

extern crate clap;
use clap::{Arg, App};

use rusoto_core::Region;
use rusoto_s3::{S3, S3Client, DeleteBucketRequest};

#[tokio::main]
async fn main() {

    let matches = App::new("Example Delete Bucket by Rust")
                            .version("1.0")
                            .author("rilindo.foster@<rilindo.foster@monzell.com")
                            .about("Delete Bucket")
                            .arg(Arg::with_name("BUCKETNAME")
                               .help("Bucket Name")
                               .required(true)
                               .index(1))
                              .get_matches();

    let bucket_name = matches.value_of("BUCKETNAME").unwrap();

    let client = S3Client::new(Region::default());
    let delete_bucket_request = DeleteBucketRequest {
        bucket: bucket_name.clone().to_string(),
        ..Default::default()
    };

    let resp = client.delete_bucket(delete_bucket_request).await;

    println!(
        "Bucket {} deleted, resp: {:#?}",
        bucket_name.clone(),
        resp.unwrap()
    );

}
