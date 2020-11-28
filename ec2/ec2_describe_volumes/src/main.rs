extern crate rusoto_core;
extern crate rusoto_ec2;

use rusoto_core::Region;
use rusoto_ec2::{DescribeVolumesRequest, DescribeVolumesResult, Ec2, Ec2Client, Filter};

async fn describe_volumes(
    client: &Ec2Client,
    describe_volumes_request: DescribeVolumesRequest,
) -> DescribeVolumesResult {
    let resp = client.describe_volumes(describe_volumes_request).await;
    return resp.unwrap();
}

fn process_describe_volumes(resp: DescribeVolumesResult) {
    match resp.volumes {
        Some(volumes) => {
            for v in volumes {
                match v.volume_id {
                    Some(volume_id) => {
                        let volume_size = v.size.unwrap().to_string();
                        let snapshot_id = v.snapshot_id.unwrap();
                        let volume_type = v.volume_type.unwrap();
                        println!(
                            "Volume ID: {}, Volume Type: {}, Volume Size: {}, Snapshot ID: {}",
                            volume_id, volume_type, volume_size, snapshot_id
                        )
                    }
                    None => (),
                }
            }
        }
        None => println!("No volumes in this region"),
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
    let describe_volumes_request = DescribeVolumesRequest {
        filters: Some(vec![filter]),
        next_token: None,
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let mut resp = rt.block_on(describe_volumes(&client, describe_volumes_request));

    process_describe_volumes(resp.clone());

    while resp.clone().next_token != None {
        let filter = Filter {
            name: None,
            values: None,
        };
        let describe_volumes_request = DescribeVolumesRequest {
            filters: Some(vec![filter]),
            next_token: resp.clone().next_token,
            ..Default::default()
        };
        resp = rt.block_on(describe_volumes(&client, describe_volumes_request));
        process_describe_volumes(resp.clone());
    }
}
