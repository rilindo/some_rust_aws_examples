extern crate rusoto_core;
extern crate rusoto_ec2;

use rusoto_core::Region;
use rusoto_ec2::{Ec2,
    Ec2Client,
    DescribeSecurityGroupsRequest,
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
    let describe_security_groups_req = DescribeSecurityGroupsRequest {
        filters: Some(vec![filter]),
        ..Default::default()
    };

    match client.describe_security_groups(describe_security_groups_req).await {
        Ok(output) => {
            match output.security_groups {
                Some(security_groups) => {
                    for s in security_groups {
                        match s.group_id {
                            Some(group_id) => println!("{}",group_id),
                            None => println!("No security group ids in this region"),
                        }
                        match s.ip_permissions {
                            Some(ip_permissions) => {
                                for i in ip_permissions {
                                    match i.ip_ranges {
                                        Some (ip_ranges) => {
                                            for r in ip_ranges {
                                                match r.cidr_ip {
                                                    Some(cidr_ip) => println!("{}",cidr_ip),
                                                    None => println!("No CIDR"),
                                                }
                                            }
                                        }
                                        None => println!("No Ip ranges")
                                    }
                                }
                            }
                            None => println!("No security group permissons"),
                        }
                    }
                }
                None => println!("No security groups in this region"),
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
