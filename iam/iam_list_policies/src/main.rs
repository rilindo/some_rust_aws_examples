extern crate rusoto_core;
extern crate rusoto_iam;

use rusoto_core::Region;
use rusoto_iam::{Iam,
    IamClient,
    ListPoliciesRequest,
    ListPoliciesResponse
};

async fn list_policies(client: &IamClient, list_policies_request: ListPoliciesRequest) -> ListPoliciesResponse {

    let resp = client.list_policies(list_policies_request).await;
    return resp.unwrap();

}

fn process_list_policies(resp: ListPoliciesResponse) {

    match resp.policies{
        Some(policies) =>
            for p in policies {
                match p.policy_name {
                    Some(policy_name) => println!("Role Name: {}",policy_name),
                    _ => (),
                }
                match p.arn {
                    Some(arn) => println!("Role ARN: {}",arn),
                    _ => (),
                }
            }
        _ => ()
    }
    match resp.marker {
        Some(_) => (),
        None => println!("List policies complete"),
    }

}

fn main() {

    let client = IamClient::new(Region::default());
    let list_policies_request = ListPoliciesRequest {
        marker: None,
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let mut resp = rt.block_on(list_policies(&client, list_policies_request));

    process_list_policies(resp.clone());

    while resp.clone().is_truncated.unwrap() {
        let list_policies_request = ListPoliciesRequest {
            marker: resp.clone().marker,
            ..Default::default()
        };
        resp = rt.block_on(list_policies(&client, list_policies_request));
        process_list_policies(resp.clone());
    }
}
