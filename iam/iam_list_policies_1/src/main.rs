extern crate rusoto_core;
extern crate rusoto_iam;

use rusoto_core::Region;
use rusoto_iam::{Iam,
    IamClient,
    ListPoliciesRequest
};

#[tokio::main]
async fn main() {
    // See https://docs.rs/rusoto_core/0.40.0/rusoto_core/region/enum.Region.html#default
    // to under how the defaults work for regions.

    let client = IamClient::new(Region::default());
    let list_policies_req = ListPoliciesRequest {
        ..Default::default()
    };

    match client.list_policies(list_policies_req).await {
        Ok(output) => {
            match output.policies{
                Some(policies) =>
                    for p in policies {
                        match p.policy_name {
                            Some(policy_name) => println!("Role Name: {}",policy_name),
                            None => println!("No roles available"),
                        }
                        match p.arn {
                            Some(arn) => println!("Role ARN: {}",arn),
                            None => println!("No arns"),
                        }
                    }
                None => println!("No tokens available"),
            }
            match output.marker {
                Some(marker) => println!("{}", marker),
                None => println!("No tokens available"),
            }

        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
