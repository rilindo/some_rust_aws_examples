extern crate rusoto_cloudfront;
extern crate rusoto_core;

use rusoto_cloudfront::{
    CloudFront, CloudFrontClient, ListDistributionsRequest, ListDistributionsResult,
};
use rusoto_core::Region;

async fn list_distributions(
    client: &CloudFrontClient,
    list_distributions_request: ListDistributionsRequest,
) -> ListDistributionsResult {
    let resp = client.list_distributions(list_distributions_request).await;
    return resp.unwrap();
}

fn list_distributions_result(resp: ListDistributionsResult) {
    match resp.distribution_list {
        Some(distribution_list) => match distribution_list.items {
            Some(items) => {
                for i in items {
                    match i.arn {
                        arn => println!("{}", arn),
                    }
                }
            }
            None => (),
        },
        None => println!("No distributions in this region"),
    }
}

fn main() {
    let client = CloudFrontClient::new(Region::default());

    let list_distributions_request = ListDistributionsRequest {
        marker: None,
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let mut resp = rt.block_on(list_distributions(&client, list_distributions_request));

    list_distributions_result(resp.clone());

    while resp.clone().distribution_list.unwrap().is_truncated {
        let list_distributions_request = ListDistributionsRequest {
            marker: resp.clone().distribution_list.unwrap().next_marker,
            ..Default::default()
        };
        resp = rt.block_on(list_distributions(&client, list_distributions_request));
        list_distributions_result(resp.clone());
    }
}
