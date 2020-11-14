extern crate rusoto_core;
extern crate rusoto_ec2;

use rusoto_core::Region;
use rusoto_ec2::{Ec2,
    Ec2Client,
    DescribeVpcsRequest
};

#[tokio::main]
async fn main() {
    // See https://docs.rs/rusoto_core/0.40.0/rusoto_core/region/enum.Region.html#default
    // to under how the defaults work for regions.
    let client = Ec2Client::new(Region::default());
    let describe_vpcs_req = DescribeVpcsRequest {
        ..Default::default()
    };

    match client.describe_vpcs(describe_vpcs_req).await {
        Ok(output) => {
            match output.vpcs {
                Some(vpcs) => {
                    for v in vpcs {
                        match v.vpc_id {
                            Some(vpc_id) => println!("{}",vpc_id),
                            None => println!("No vpcs this region")
                        }
                    }
                }
                None => println!("No vpcs this region"),
            }
            match output.next_token {
                Some(token) => println!("{}", token),
                None => println!("No tokens available"),
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
