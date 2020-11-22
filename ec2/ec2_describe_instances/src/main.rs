extern crate rusoto_core;
extern crate rusoto_ec2;

extern crate clap;
use clap::{Arg, App};

use rusoto_core::Region;
use rusoto_ec2::{Ec2,
    Ec2Client,
    DescribeInstancesRequest,
    DescribeInstancesResult,
    Filter
};

async fn describe_instances(client: &Ec2Client, describe_instances_request: DescribeInstancesRequest) -> DescribeInstancesResult {

    let resp = client.describe_instances(describe_instances_request).await;
    return resp.unwrap();

}

fn process_describe_instances(resp: DescribeInstancesResult) {

    match resp.reservations {
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
    match resp.next_token {
        Some(token) => println!("{}", token),
        None => println!("No tokens available"),
    }
}

fn main() {

    let matches = App::new("Example Describe Instances")
                            .version("1.0")
                            .author("rilindo.foster@<rilindo.foster@monzell.com")
                            .about("Describe Instances")
                            .arg(Arg::with_name("VPCID")
                               .help("VPC ID")
                               .required(true)
                               .index(1))
                              .get_matches();
    let vpc_id = matches.value_of("VPCID").unwrap();

    let filter = Filter {
        name: Some(String::from("vpc-id")),
        values: Some(vec![String::from(vpc_id)])
    };

    let client = Ec2Client::new(Region::default());
    let describe_instances_request = DescribeInstancesRequest {
        filters: Some(vec![filter]),
        next_token: None,
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let mut resp = rt.block_on(describe_instances(&client, describe_instances_request));

    process_describe_instances(resp.clone());

    while resp.clone().next_token != None {
        let filter = Filter {
            name: Some(String::from("vpc-id")),
            values: Some(vec![String::from(vpc_id)])
        };
        let describe_instances_request = DescribeInstancesRequest {
            filters: Some(vec![filter]),
            next_token: resp.clone().next_token,
            ..Default::default()
        };
        resp = rt.block_on(describe_instances(&client, describe_instances_request));
        process_describe_instances(resp.clone());
    }
}
