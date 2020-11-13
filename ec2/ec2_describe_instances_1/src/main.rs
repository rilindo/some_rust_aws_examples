extern crate rusoto_core;
extern crate rusoto_ec2;

use rusoto_core::Region;
use rusoto_ec2::{Ec2,
    Ec2Client,
    DescribeInstancesRequest,
    Filter
};

#[tokio::main]
async fn main() {
    // See https://docs.rs/rusoto_core/0.40.0/rusoto_core/region/enum.Region.html#default
    // to under how the defaults work for regions.

    let filter = Filter {
        name: Some(String::from("vpc-id")),
        values: Some(vec![String::from("vpc-abc123")])
    };

    let client = Ec2Client::new(Region::default());
    let describe_instances_req = DescribeInstancesRequest {
        filters: Some(vec![filter]),
        ..Default::default()
    };

    match client.describe_instances(describe_instances_req).await {
        Ok(output) => {
            match output.reservations {
                Some(reservations) => {
                    for r in reservations {
                        match r.instances {
                            Some(instances) => {
                                for i in instances {
                                    match i.instance_id {
                                        Some(instance_id) => println!("{}",instance_id),
                                        None => println!("No instances in this region"),
                                    }
                                }
                            }
                            None => println!("No instances in this region"),
                        }
                    }
                }
                None => println!("No reservations available"),
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
