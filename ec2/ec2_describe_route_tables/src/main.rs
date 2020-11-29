extern crate rusoto_core;
extern crate rusoto_ec2;

extern crate clap;
use clap::{App, Arg};

use rusoto_core::Region;
use rusoto_ec2::{DescribeRouteTablesRequest, DescribeRouteTablesResult, Ec2, Ec2Client, Filter};

async fn describe_route_tables(
    client: &Ec2Client,
    describe_route_tables_request: DescribeRouteTablesRequest,
) -> DescribeRouteTablesResult {
    let resp = client
        .describe_route_tables(describe_route_tables_request)
        .await;
    return resp.unwrap();
}

fn process_describe_route_tables(resp: DescribeRouteTablesResult) {
    match resp.route_tables {
        Some(route_tables) => {
            for rt in route_tables {
                match rt.routes {
                    Some(routes) => {
                        for rs in routes {
                            println!(
                                "VPC ID: {}, Route Table ID: {}, Destination CIDR Block: {}",
                                rt.vpc_id.clone().unwrap(),
                                rt.route_table_id.clone().unwrap(),
                                rs.destination_cidr_block.unwrap()
                            )
                        }
                    }
                    None => (),
                }
            }
        }
        None => println!("No subnets defined in provided VPC"),
    }
}

fn main() {
    let matches = App::new("An example of a describe route tables call using Rust and Rusoto")
        .version("1.0")
        .author("rilindo.foster@<rilindo.foster@monzell.com")
        .about("Describe Route Tables")
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
    let describe_route_tables_request = DescribeRouteTablesRequest {
        filters: Some(vec![filter]),
        next_token: None,
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let mut resp = rt.block_on(describe_route_tables(
        &client,
        describe_route_tables_request,
    ));

    process_describe_route_tables(resp.clone());

    while resp.clone().next_token != None {
        let filter = Filter {
            name: Some(String::from("vpc-id")),
            values: Some(vec![String::from(vpc_id)]),
        };
        let describe_route_tables_request = DescribeRouteTablesRequest {
            filters: Some(vec![filter]),
            next_token: resp.clone().next_token,
            ..Default::default()
        };
        resp = rt.block_on(describe_route_tables(
            &client,
            describe_route_tables_request,
        ));
        process_describe_route_tables(resp.clone());
    }
}
