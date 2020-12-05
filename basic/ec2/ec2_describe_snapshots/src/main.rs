extern crate rusoto_core;
extern crate rusoto_ec2;

use rusoto_core::Region;
use rusoto_ec2::{DescribeSnapshotsRequest, DescribeSnapshotsResult, Ec2, Ec2Client, Filter};

async fn describe_snapshots_request(
    client: &Ec2Client,
    describe_snapshots_req: DescribeSnapshotsRequest,
) -> DescribeSnapshotsResult {
    let resp = client.describe_snapshots(describe_snapshots_req).await;
    return resp.unwrap();
}

fn describe_snapshots_result(resp: DescribeSnapshotsResult) {
    match resp.snapshots {
        Some(snapshots) => {
            for s in snapshots {
                match s.description {
                    Some(description) => println!("{}", description),
                    None => println!("No description available"),
                }
                match s.snapshot_id {
                    Some(snapshot_id) => println!("{}", snapshot_id),
                    None => (),
                }
                match s.start_time {
                    Some(start_time) => println!("{}", start_time),
                    None => (),
                }
            }
        }
        None => println!("No snapshots this region"),
    }
    match resp.next_token {
        Some(token) => println!("{}", token),
        None => (),
    }
}

fn main() {
    let filter = Filter {
        name: None,
        values: None,
    };

    let client = Ec2Client::new(Region::default());
    let describe_snapshots_req = DescribeSnapshotsRequest {
        filters: Some(vec![filter]),
        next_token: None,
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let mut resp = rt.block_on(describe_snapshots_request(&client, describe_snapshots_req));

    describe_snapshots_result(resp.clone());

    while resp.clone().next_token != None {
        let filter = Filter {
            name: None,
            values: None,
        };
        let describe_snapshots_req = DescribeSnapshotsRequest {
            filters: Some(vec![filter]),
            next_token: resp.clone().next_token,
            ..Default::default()
        };
        resp = rt.block_on(describe_snapshots_request(&client, describe_snapshots_req));
        describe_snapshots_result(resp.clone());
    }
}
