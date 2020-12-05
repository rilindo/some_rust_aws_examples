extern crate rusoto_core;
extern crate rusoto_s3;

extern crate clap;
use clap::{App, Arg};

use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, S3};

#[tokio::main]
async fn main() {
    let matches = App::new("Example of put object call using Rust and Rusoto")
        .version("1.0")
        .author("rilindo.foster@<rilindo.foster@monzell.com")
        .about("put object")
        .arg(
            Arg::with_name("bucket_name")
                .short("b")
                .long("bucket_name")
                .help("Bucket Name")
                .required(true)
                .takes_value(true),
        ).arg(
            Arg::with_name("value")
                .short("v")
                .long("value")
                .help("value")
                .required(true)
                .takes_value(true),
        ).arg(
            Arg::with_name("key")
                .short("k")
                .long("key")
                .help("key")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let bucket_name = matches.value_of("bucket_name").unwrap();
    let key = matches.value_of("key").unwrap();
    let value = matches.value_of("value").unwrap().to_string().into_bytes();

    let client = S3Client::new(Region::default());
    let put_request = PutObjectRequest {
        bucket: bucket_name.to_owned(),
        key: key.to_owned(),
        body: Some(value.into()),
        ..Default::default()
    };

    let resp = client.put_object(put_request).await;

    println!(
        "key {} create on {}, resp: {:#?}",
        key.clone(),
        bucket_name.clone(),
        resp.unwrap()
    );
}
