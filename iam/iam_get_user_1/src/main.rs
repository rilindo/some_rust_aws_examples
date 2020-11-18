extern crate rusoto_core;
extern crate rusoto_iam;

use rusoto_core::Region;
use rusoto_iam::{Iam,
    IamClient,
    GetUserRequest,
};

#[tokio::main]
async fn main() {
    // See https://docs.rs/rusoto_core/0.40.0/rusoto_core/region/enum.Region.html#default
    // to under how the defaults work for regions.

    let client = IamClient::new(Region::default());
    let user_name = "exampleuser".to_string();
    let get_user_req = GetUserRequest {
        user_name: Some(user_name.clone()),
        ..Default::default()
    };

    match client.get_user(get_user_req).await {
        Ok(output) => {
            println!("User Name: {}", output.user.user_name);
            println!("User ARN: {}", output.user.arn);
            println!("Create Date: {}", output.user.create_date);
            println!("User ID: {}", output.user.user_id);
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
