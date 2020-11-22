extern crate rusoto_core;
extern crate rusoto_route53;

use rusoto_core::Region;
use rusoto_route53::{Route53,
    Route53Client,
    ListHostedZonesRequest,
    ListHostedZonesResponse
};

async fn list_hosted_zones(client: &Route53Client, list_hosted_zones_request: ListHostedZonesRequest) -> ListHostedZonesResponse {

    let resp = client.list_hosted_zones(list_hosted_zones_request).await;
    return resp.unwrap();

}

fn process_list_hosted_zones(resp: ListHostedZonesResponse) {

    for h in resp.hosted_zones {
        match h.id {
            id => println!("Hosted Zone ID: {}", id),
        }
        match h.name {
            name => println!("Hosted Zone: {}", name),
        }
    }

    match resp.marker {
        marker => println!("Next Marker: {}", marker),
    }

}

fn main() {

    let client = Route53Client::new(Region::default());
    let list_hosted_zones_request = ListHostedZonesRequest {
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let mut resp = rt.block_on(list_hosted_zones(&client, list_hosted_zones_request));

    process_list_hosted_zones(resp.clone());

    while resp.clone().is_truncated {
        let list_hosted_zones_request = ListHostedZonesRequest {
            marker: resp.clone().next_marker,
            ..Default::default()
        };
        resp = rt.block_on(list_hosted_zones(&client, list_hosted_zones_request));
        process_list_hosted_zones(resp.clone());
    }
}
