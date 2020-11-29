extern crate rusoto_core;
extern crate rusoto_rds;

use rusoto_core::Region;
use rusoto_rds::{DBInstanceMessage, DescribeDBInstancesMessage, Rds, RdsClient};

async fn describe_db_instances_request(
    client: &RdsClient,
    describe_db_instances_req: DescribeDBInstancesMessage,
) -> DBInstanceMessage {
    let resp = client
        .describe_db_instances(describe_db_instances_req)
        .await;
    return resp.unwrap();
}

fn describe_db_instances_result(resp: DBInstanceMessage) {
    match resp.db_instances {
        Some(db_instances) => {
            let db_iter = db_instances.iter();
            for db in db_iter {
                println!(
                    "DB Endpoint: {}, DB ARN: {}",
                    db.endpoint.clone().unwrap().address.unwrap(),
                    db.db_instance_arn.clone().unwrap()
                );
            }
        }
        None => (),
    }
}

fn main() {
    let client = RdsClient::new(Region::default());
    let describe_db_instances_req = DescribeDBInstancesMessage {
        marker: None,
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let mut resp = rt.block_on(describe_db_instances_request(
        &client,
        describe_db_instances_req,
    ));

    describe_db_instances_result(resp.clone());

    while resp.clone().marker != None {
        let describe_db_instances_req = DescribeDBInstancesMessage {
            marker: resp.clone().marker,
            ..Default::default()
        };
        resp = rt.block_on(describe_db_instances_request(
            &client,
            describe_db_instances_req,
        ));
        describe_db_instances_result(resp.clone());
    }
}
