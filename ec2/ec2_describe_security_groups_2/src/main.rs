extern crate rusoto_core;
extern crate rusoto_ec2;

use rusoto_core::Region;
use rusoto_ec2::{Ec2,
    Ec2Client,
    DescribeSecurityGroupsRequest,
    DescribeSecurityGroupsResult,
    Filter
};


async fn describe_security_groups(client: &Ec2Client, describe_security_groups_req: DescribeSecurityGroupsRequest) -> DescribeSecurityGroupsResult {

    let resp = client.describe_security_groups(describe_security_groups_req).await;
    return resp.unwrap();

}

fn process_describe_security_groups(resp: DescribeSecurityGroupsResult) {

    match resp.security_groups {
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
    match resp.next_token {
        Some(token) => println!("{}", token),
        None => println!("No tokens available"),
    }


}

fn main() {
    // See https://docs.rs/rusoto_core/0.40.0/rusoto_core/region/enum.Region.html#default
    // to under how the defaults work for regions.

    let filter = Filter {
        name: Some(String::from("vpc-id")),
        values: Some(vec![String::from("vpc-abc123")])
    };

    let client = Ec2Client::new(Region::default());
    let describe_security_groups_req = DescribeSecurityGroupsRequest {
        filters: Some(vec![filter]),
        next_token: None,
        ..Default::default()
    };


    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let mut resp = rt.block_on(describe_security_groups(&client, describe_security_groups_req));

    process_describe_security_groups(resp.clone());

    while resp.clone().next_token != None {
        let filter = Filter {
            name: Some(String::from("vpc-id")),
            values: Some(vec![String::from("vpc-abc123")])
        };
        let describe_security_groups_req = DescribeSecurityGroupsRequest {
            filters: Some(vec![filter]),
            next_token: resp.clone().next_token,
            ..Default::default()
        };
        resp = rt.block_on(describe_security_groups(&client, describe_security_groups_req));
        process_describe_security_groups(resp.clone());
    }
}
