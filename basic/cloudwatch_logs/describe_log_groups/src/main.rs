extern crate rusoto_core;
extern crate rusoto_logs;

use rusoto_core::Region;
use rusoto_logs::{
    CloudWatchLogs, CloudWatchLogsClient, DescribeLogGroupsRequest, DescribeLogGroupsResponse,
};

async fn describe_log_groups(
    client: &CloudWatchLogsClient,
    describe_log_groups_request: DescribeLogGroupsRequest,
) -> DescribeLogGroupsResponse {
    let resp = client
        .describe_log_groups(describe_log_groups_request)
        .await;
    return resp.unwrap();
}

fn process_describe_log_groups(resp: DescribeLogGroupsResponse) {
    match resp.log_groups {
        Some(log_groups) => {
            for l in log_groups {
                println!("Log Group Name: {}", l.log_group_name.unwrap());
            }
        }
        None => println!("No volumes in this region"),
    }
}

fn main() {
    let client = CloudWatchLogsClient::new(Region::default());
    let describe_log_groups_request = DescribeLogGroupsRequest {
        next_token: None,
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let mut resp = rt.block_on(describe_log_groups(&client, describe_log_groups_request));

    process_describe_log_groups(resp.clone());

    while resp.clone().next_token != None {
        let describe_log_groups_request = DescribeLogGroupsRequest {
            next_token: resp.clone().next_token,
            ..Default::default()
        };
        resp = rt.block_on(describe_log_groups(&client, describe_log_groups_request));
        process_describe_log_groups(resp.clone());
    }
}
