extern crate rusoto_core;
extern crate rusoto_ec2;

extern crate clap;
use clap::{App, Arg};

use rusoto_core::Region;
use rusoto_ec2::{
    DescribeSecurityGroupsRequest, DescribeSecurityGroupsResult, Ec2, Ec2Client, Filter,
};

async fn describe_security_groups(
    client: &Ec2Client,
    describe_security_groups_req: DescribeSecurityGroupsRequest,
) -> DescribeSecurityGroupsResult {
    let resp = client
        .describe_security_groups(describe_security_groups_req)
        .await;
    return resp.unwrap();
}

fn process_describe_security_groups(resp: DescribeSecurityGroupsResult) {
    match resp.security_groups {
        Some(security_groups) => {
            for s in security_groups {
                match s.group_id {
                    Some(group_id) => println!("{}", group_id),
                    None => println!("No security group ids in this region"),
                }
                match s.ip_permissions {
                    Some(ip_permissions) => {
                        for i in ip_permissions {
                            match i.ip_ranges {
                                Some(ip_ranges) => {
                                    for r in ip_ranges {
                                        match r.cidr_ip {
                                            Some(cidr_ip) => println!("{}", cidr_ip),
                                            None => println!("No CIDR"),
                                        }
                                    }
                                }
                                None => println!("No Ip ranges"),
                            }
                        }
                    }
                    None => println!("No security group permissons"),
                }
            }
        }
        None => println!("No security groups in this region"),
    }
    match resp.next_token {
        Some(token) => println!("{}", token),
        None => println!("No tokens available"),
    }
}

fn main() {
    let matches = App::new("Example Describe Security groups Using Rust")
        .version("1.0")
        .author("rilindo.foster@<rilindo.foster@monzell.com")
        .about("Describe Security Groups")
        .arg(
            Arg::with_name("VPCID")
                .help("VPC ID")
                .required(true)
                .index(1),
        )
        .get_matches();
    let vpc_id = matches.value_of("VPCID").unwrap();

    let filter = Filter {
        name: Some(String::from("vpc-id")),
        values: Some(vec![String::from(vpc_id)]),
    };

    let client = Ec2Client::new(Region::default());
    let describe_security_groups_req = DescribeSecurityGroupsRequest {
        filters: Some(vec![filter]),
        next_token: None,
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let mut resp = rt.block_on(describe_security_groups(
        &client,
        describe_security_groups_req,
    ));

    process_describe_security_groups(resp.clone());

    while resp.clone().next_token != None {
        let filter = Filter {
            name: Some(String::from("vpc-id")),
            values: Some(vec![String::from(vpc_id)]),
        };
        let describe_security_groups_req = DescribeSecurityGroupsRequest {
            filters: Some(vec![filter]),
            next_token: resp.clone().next_token,
            ..Default::default()
        };
        resp = rt.block_on(describe_security_groups(
            &client,
            describe_security_groups_req,
        ));
        process_describe_security_groups(resp.clone());
    }
}
