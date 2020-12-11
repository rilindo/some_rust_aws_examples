extern crate rusoto_core;
extern crate rusoto_ec2;

use rusoto_core::Region;
use rusoto_ec2::{DescribeKeyPairsRequest, DescribeKeyPairsResult, Ec2, Ec2Client};

async fn describe_key_pairs(
    client: &Ec2Client,
    describe_key_pairs_request: DescribeKeyPairsRequest,
) -> DescribeKeyPairsResult {
    let resp = client.describe_key_pairs(describe_key_pairs_request).await;
    return resp.unwrap();
}

fn process_describe_key_pairs(resp: DescribeKeyPairsResult) {
    for k in resp.key_pairs.unwrap() {
        println!("Key Pair Name: {}", k.key_name.unwrap());
    }
}

fn main() {
    let client = Ec2Client::new(Region::default());
    let describe_key_pairs_request = DescribeKeyPairsRequest {
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let resp = rt.block_on(describe_key_pairs(&client, describe_key_pairs_request));

    process_describe_key_pairs(resp.clone());
}
