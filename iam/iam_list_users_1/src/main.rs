extern crate rusoto_core;
extern crate rusoto_iam;

use rusoto_core::Region;
use rusoto_iam::{Iam,
    IamClient,
    ListUsersRequest,
    ListUsersResponse
};

async fn list_users(client: &IamClient, list_users_req: ListUsersRequest) -> ListUsersResponse {

    let resp = client.list_users(list_users_req).await;
    return resp.unwrap();

}

fn process_list_users(resp: ListUsersResponse) {

    for u in resp.users {
        match u.user_name {
            user_name => println!("Username: {}",user_name)
        }
        match u.arn {
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
    let list_users_req = ListUsersRequest {
        ..Default::default()
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let mut resp = rt.block_on(list_users(&client, list_users_req));

    process_list_users(resp.clone());

    while resp.clone().is_truncated.unwrap() {
        let list_users_req = ListUsersRequest {
            marker: resp.clone().marker,
            ..Default::default()
        };
        resp = rt.block_on(list_users(&client, list_users_req));
        process_list_users(resp.clone());
    }

}
