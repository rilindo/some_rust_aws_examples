extern crate rusoto_core;
extern crate rusoto_rds;

use rusoto_core::Region;
use rusoto_rds::{DBSnapshotMessage, DescribeDBSnapshotsMessage, Rds, RdsClient};

async fn describe_db_snapshots_request(
    client: &RdsClient,
    describe_db_snapshots_req: DescribeDBSnapshotsMessage,
) -> DBSnapshotMessage {
    let resp = client
        .describe_db_snapshots(describe_db_snapshots_req)
        .await;
    return resp.unwrap();
}

fn describe_db_snapshots_result(resp: DBSnapshotMessage) {
    for snapshot in resp.db_snapshots.unwrap().iter() {
        println!(
            "Snapshot ARN: {}",
            snapshot.db_snapshot_arn.as_ref().unwrap()
        );
    }
}

fn main() {
    let client = RdsClient::new(Region::default());
    let describe_db_snapshots_req = DescribeDBSnapshotsMessage {
        marker: None,
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let mut resp = rt.block_on(describe_db_snapshots_request(
        &client,
        describe_db_snapshots_req,
    ));

    describe_db_snapshots_result(resp.clone());

    while resp.clone().marker != None {
        let describe_db_snapshots_req = DescribeDBSnapshotsMessage {
            marker: resp.clone().marker,
            ..Default::default()
        };
        resp = rt.block_on(describe_db_snapshots_request(
            &client,
            describe_db_snapshots_req,
        ));
        describe_db_snapshots_result(resp.clone());
    }
}
