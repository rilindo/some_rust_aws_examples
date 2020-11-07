extern crate rusoto_core;
extern crate rusoto_iam;

use rusoto_core::Region;
use rusoto_iam::{Iam,
    IamClient,
    ListRolesRequest
};

#[tokio::main]
async fn main() {
    // See https://docs.rs/rusoto_core/0.40.0/rusoto_core/region/enum.Region.html#default
    // to under how the defaults work for regions.

    let client = IamClient::new(Region::default());
    let list_roles_req = ListRolesRequest {
        ..Default::default()
    };

    match client.list_roles(list_roles_req).await {
        Ok(output) => {
            for r in output.roles {
                match r.role_name {
                    role_name => println!("Role Name: {}",role_name)
                }
                match r.arn {
                    arn => println!("Role ARN: {}",arn)
                }

            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
