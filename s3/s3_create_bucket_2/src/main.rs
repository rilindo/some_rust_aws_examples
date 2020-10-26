extern crate rusoto_core;
extern crate rusoto_s3;

use rusoto_core::Region;
use rusoto_s3::{S3, S3Client, CreateBucketRequest, CreateBucketConfiguration};
use std::process;
use std::env;
use std::error::Error;

struct Config {
    bucket_name: String,
    region: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let bucket_name = args[1].clone();
        let region = args[2].clone();

        Ok(Config {
            bucket_name,
            region
        })
    }
}

async fn bucket_request(bucket_name: &str, location_constraint: CreateBucketConfiguration) {

    let client = S3Client::new(Region::UsEast2);

    let create_bucket_req = CreateBucketRequest {
        bucket: bucket_name.to_string(),
        create_bucket_configuration: Some(location_constraint),
        ..Default::default()
    };

    let resp = client.create_bucket(create_bucket_req).await;

    println!(
        "Bucket {} created, resp: {:#?}",
        bucket_name.to_string(),
        resp.unwrap()
    );
}

fn bucket_create(config: Config) -> Result<(), Box<dyn Error>> {

    let bucket_name = config.bucket_name;
    let location_constraint = CreateBucketConfiguration {
        location_constraint: Some(config.region),
    };
    let mut rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(bucket_request(&bucket_name, location_constraint));
    Ok(())
}

fn main() {

    let args: Vec<String> = env::args().collect();



    let config  = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    if let Err(e) = bucket_create(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };

}
