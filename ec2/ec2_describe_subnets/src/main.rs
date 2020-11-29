extern crate rusoto_core;
extern crate rusoto_ec2;

extern crate clap;
use clap::{App, Arg};

use rusoto_core::Region;
use rusoto_ec2::{DescribeSubnetsRequest, DescribeSubnetsResult, Ec2, Ec2Client, Filter};

async fn describe_subnets(
    client: &Ec2Client,
    describe_subnets_request: DescribeSubnetsRequest,
) -> DescribeSubnetsResult {
    let resp = client.describe_subnets(describe_subnets_request).await;
    return resp.unwrap();
}

fn process_describe_subnets(resp: DescribeSubnetsResult) {
    match resp.subnets {
        Some(subnets) => {
            for s in subnets {
                println!(
                    "VPC ID: {}, Subnet ID: {}, CIDR Block: {}",
                    s.vpc_id.unwrap(),
                    s.subnet_id.unwrap(),
                    s.cidr_block.unwrap()
                )
            }
        }
        None => println!("No subnets defined in provided VPC"),
    }
}

fn main() {
    let matches = App::new("An example of a describe subnets call using Rust and Rusoto")
        .version("1.0")
        .author("rilindo.foster@<rilindo.foster@monzell.com")
        .about("Describe Subnets")
        .arg(
            Arg::with_name("vpc_id")
                .short("v")
                .long("vpc_id")
                .help("VPC ID")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let vpc_id = matches.value_of("vpc_id").unwrap();

    let filter = Filter {
        name: Some(String::from("vpc-id")),
        values: Some(vec![String::from(vpc_id)]),
    };

    let client = Ec2Client::new(Region::default());
    let describe_subnets_request = DescribeSubnetsRequest {
        filters: Some(vec![filter]),
        next_token: None,
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let mut resp = rt.block_on(describe_subnets(&client, describe_subnets_request));

    process_describe_subnets(resp.clone());

    while resp.clone().next_token != None {
        let filter = Filter {
            name: Some(String::from("vpc-id")),
            values: Some(vec![String::from(vpc_id)]),
        };
        let describe_subnets_request = DescribeSubnetsRequest {
            filters: Some(vec![filter]),
            next_token: resp.clone().next_token,
            ..Default::default()
        };
        resp = rt.block_on(describe_subnets(&client, describe_subnets_request));
        process_describe_subnets(resp.clone());
    }
}
