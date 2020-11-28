extern crate rusoto_core;
extern crate rusoto_kms;

extern crate clap;
use clap::{App, Arg};

use rusoto_core::Region;
use rusoto_kms::{DescribeKeyRequest, DescribeKeyResponse, Kms, KmsClient};

async fn describe_key(
    client: &KmsClient,
    describe_key_request: DescribeKeyRequest,
) -> DescribeKeyResponse {
    let resp = client.describe_key(describe_key_request).await;
    return resp.unwrap();
}

fn describe_keys_response(resp: DescribeKeyResponse) {
    match resp.key_metadata {
        Some(key_metadata) => {
            match key_metadata.key_id {
                key_id => println!("Key ID: {}", key_id),
            }
            match key_metadata.description {
                Some(description) => println!("Description: {}", description),
                None => (),
            }
            match key_metadata.arn {
                Some(arn) => println!("ARN: {}", arn),
                None => (),
            }
            match key_metadata.aws_account_id {
                Some(aws_account_id) => println!("AWS Account ID: {}", aws_account_id),
                None => (),
            }
        }
        None => (),
    }
}

fn main() {
    let matches = App::new("Example of a describe key call using Rust")
        .version("1.0")
        .author("rilindo.foster@<rilindo.foster@monzell.com")
        .about("Describe Key Command")
        .arg(
            Arg::with_name("key_id")
                .short("k")
                .long("key_id")
                .help("Set key ID")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let key_id = matches.value_of("key_id").unwrap();

    let client = KmsClient::new(Region::default());
    let describe_key_request = DescribeKeyRequest {
        key_id: key_id.to_string(),
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let resp = rt.block_on(describe_key(&client, describe_key_request));

    describe_keys_response(resp.clone());
}
