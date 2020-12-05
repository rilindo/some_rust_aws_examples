extern crate rusoto_core;
extern crate rusoto_rds;

use rusoto_core::Region;
use rusoto_rds::{DBSubnetGroupMessage, DescribeDBSubnetGroupsMessage, Rds, RdsClient};

async fn describe_db_subnet_groups_request(
    client: &RdsClient,
    describe_db_subnet_groups_req: DescribeDBSubnetGroupsMessage,
) -> DBSubnetGroupMessage {
    let resp = client
        .describe_db_subnet_groups(describe_db_subnet_groups_req)
        .await;
    return resp.unwrap();
}

fn describe_db_subnet_groups_result(resp: DBSubnetGroupMessage) {
    match resp.db_subnet_groups {
        Some(db_subnet_groups) => {
            let db_subnets = db_subnet_groups.iter();
            for subnet in db_subnets {
                println!(
                    "VPC ID: {}, DB Subnet Group: {}",
                    subnet.vpc_id.clone().unwrap(),
                    subnet.db_subnet_group_arn.clone().unwrap()
                )
            }
        }
        None => (),
    }
}

fn main() {
    let client = RdsClient::new(Region::default());
    let describe_db_subnet_groups_req = DescribeDBSubnetGroupsMessage {
        marker: None,
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let mut resp = rt.block_on(describe_db_subnet_groups_request(
        &client,
        describe_db_subnet_groups_req,
    ));

    describe_db_subnet_groups_result(resp.clone());

    while resp.clone().marker != None {
        let describe_db_subnet_groups_req = DescribeDBSubnetGroupsMessage {
            marker: resp.clone().marker,
            ..Default::default()
        };
        resp = rt.block_on(describe_db_subnet_groups_request(
            &client,
            describe_db_subnet_groups_req,
        ));
        describe_db_subnet_groups_result(resp.clone());
    }
}
