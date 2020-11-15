extern crate rusoto_core;
extern crate rusoto_iam;

use rusoto_core::Region;
use rusoto_iam::{Iam,
    IamClient,
    ListRolesRequest,
    ListRolesResponse
};

async fn list_roles(client: &IamClient, list_roles_req: ListRolesRequest) -> ListRolesResponse {

    let resp = client.list_roles(list_roles_req).await;
    return resp.unwrap();

}

fn process_list_roles(resp: ListRolesResponse) {

    for r in resp.roles {
        match r.role_name {
            role_name => println!("Role Name: {}",role_name)
        }
        match r.arn {
            arn => println!("Role ARN: {}",arn)
        }

    }
    match resp.marker {
        Some(marker) => println!("Next Marker: {}", marker),
        None => println!("All markers have been processed"),
    }

}

fn main() {
    // See https://docs.rs/rusoto_core/0.40.0/rusoto_core/region/enum.Region.html#default
    // to under how the defaults work for regions.

    let client = IamClient::new(Region::default());
    let list_roles_req = ListRolesRequest {
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let mut resp = rt.block_on(list_roles(&client, list_roles_req));

    process_list_roles(resp.clone());

    while resp.clone().is_truncated.unwrap() {
        let list_roles_req = ListRolesRequest {
            marker: resp.clone().marker,
            ..Default::default()
        };
        resp = rt.block_on(list_roles(&client, list_roles_req));
        process_list_roles(resp.clone());
    }

}
